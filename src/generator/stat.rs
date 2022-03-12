use super::{
  predef::{
    ReadFmt, PREDEF_CHECK_NULL_POINTER, PREDEF_FREE_ARRAY, PREDEF_FREE_PAIR,
  },
  predef::{RequiredPredefs, PREDEF_SYS_MALLOC},
  *,
};
use crate::analyser::context::*;
use Directive::*;
use Instr::*;

fn generate_assign_lhs_ident(
  scope: &ScopeReader,
  t: Type,
  id: &Ident,
) -> <AssignLhs as Generatable>::Output {
  use IdentInfo::*;

  let offset = match scope.get(id) {
    Some(LocalVar(_, offset)) => offset,
    v => {
      unreachable!("ident must be a local variable, it's {:?}", v)
    }
  };

  (Reg::StackPointer, offset, t.size().into())
}

fn generate_assign_lhs_array_elem(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  elem: &ArrayElem,
) -> <AssignLhs as Generatable>::Output {
  /* Store address of array element into regs[1]. */
  let elem_size = elem.generate(scope, code, regs, ());

  (Reg::General(regs[0]), 0, elem_size)
}

fn generate_assign_lhs_pair_elem(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  elem: &PairElem,
) -> <AssignLhs as Generatable>::Output {
  /* Stores address of elem in regs[1]. */
  let elem_size = elem.generate(scope, code, regs, ());

  (Reg::General(regs[0]), 0, elem_size)
}

impl Generatable for AssignLhs {
  type Input = Type;
  type Output = (Reg, Offset, DataSize);

  /* Returns a (Reg, Offset) which specifies the memory address of
  this Lhs. Also returns how much data is stored at said address. */
  fn generate(
    &self,
    scope: &ScopeReader,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    t: Type,
  ) -> Self::Output {
    match self {
      AssignLhs::Ident(id) => generate_assign_lhs_ident(scope, t, id),
      AssignLhs::ArrayElem(elem) => {
        generate_assign_lhs_array_elem(scope, code, regs, elem)
      }
      AssignLhs::PairElem(elem) => {
        generate_assign_lhs_pair_elem(scope, code, regs, elem)
      }
      AssignLhs::StructElem(elem) => {
        generate_assign_lhs_struct_elem(scope, code, regs, elem)
      }
    }
  }
}

fn generate_assign_lhs_struct_elem(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  elem: &StructElem,
) -> <AssignLhs as Generatable>::Output {
  let StructElem(struct_name, expr, field_name) = elem;

  /* Get struct definition. */
  let def = scope.get_def(struct_name).unwrap();

  /* Get offset and type. */
  let (type_, offset) = def.fields.get(field_name).unwrap();

  /* Evaluate expression. */
  expr.generate(scope, code, regs, ());

  /* Return location. */
  (Reg::General(regs[0]), *offset, type_.size().into())
}

/* Mallocs {bytes} bytes and leaves the address in {reg}. */
pub fn generate_malloc(bytes: i32, code: &mut GeneratedCode, reg: Reg) {
  /* LDR r0, ={bytes} */
  code.text.push(Asm::ldr(Reg::Arg(ArgReg::R0), bytes));

  /* BL malloc */
  code.text.push(Asm::b(PREDEF_SYS_MALLOC).link());

  /* MOV {regs[0]}, r0 */
  if reg != Reg::Arg(ArgReg::R0) {
    code
      .text
      .push(Asm::mov(reg, Op2::Reg(Reg::Arg(ArgReg::R0), 0)));
  }
}

fn generate_assign_rhs_expr(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  expr: &Expr,
) {
  expr.generate(scope, code, regs, ())
}

impl Generatable for AssignRhs {
  type Input = ();
  type Output = ();

  fn generate(
    &self,
    scope: &ScopeReader,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    _: (),
  ) {
    match self {
      AssignRhs::Expr(expr) => {
        generate_assign_rhs_expr(scope, code, regs, expr)
      }
    }
  }
}

impl Generatable for StructLiter {
  type Input = ();
  type Output = ();

  fn generate(
    &self,
    scope: &ScopeReader,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    aux: Self::Input,
  ) -> Self::Output {
    let StructLiter { id, fields } = self;

    /* Get size of struct. */
    let struct_def = scope
      .get_def(id)
      .expect("Analyser should ensure all struct usages are valid.");

    /* Malloc for the struct. */
    generate_malloc(struct_def.size, code, Reg::General(regs[0]));

    /* Expression evaluation can't use register malloc */
    let expr_regs = &regs[1..];

    /* For each field: */
    for (field_name, expr) in fields.iter() {
      /* Evaluate expression. */
      expr.generate(scope, code, expr_regs, aux);

      /* Calculate offset. */
      let offset = struct_def.fields.get(field_name).unwrap().1;

      /* Write to struct. */
      code.text.push(Asm::str(
        Reg::General(expr_regs[0]),
        (Reg::General(regs[0]), offset),
      ));
    }
  }
}

impl Generatable for PairElem {
  type Input = ();
  type Output = DataSize;

  /* Puts the address of the element in regs[0], returns size pointed to. */
  fn generate(
    &self,
    scope: &ScopeReader,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    _aux: (),
  ) -> DataSize {
    /*  */
    let (t, pair, offset) = match self {
      PairElem::Fst(TypedExpr(t, pair)) => (t, pair, 0),
      PairElem::Snd(TypedExpr(t, pair)) => (t, pair, ARM_DSIZE_WORD),
    };

    /* Store address of pair in regs[0]. */
    pair.generate(scope, code, regs, ());

    /* CHECK: regs[0] != NULL */
    code.text.push(Asm::mov(
      Reg::Arg(ArgReg::R0),
      Op2::Reg(Reg::General(regs[0]), 0),
    ));
    code.text.push(Asm::b(PREDEF_CHECK_NULL_POINTER).link());
    RequiredPredefs::CheckNullPointer.mark(code);

    /* Dereference. */
    code.text.push(Asm::ldr(
      Reg::General(regs[0]),
      (Reg::General(regs[0]), offset),
    ));

    /* Return how much data needs to be read from regs[0]. */
    t.size().into()
  }
}

impl Generatable for ScopedStat {
  type Input = ();
  type Output = ();
  fn generate(
    &self,
    scope: &ScopeReader,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    _aux: (),
  ) {
    let ScopedStat(st, statement) = self;

    /* Allocate space on stack for variables declared in this scope. */
    code.text.append(&mut Op2::imm_unroll(
      |offset| Asm::sub(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset)),
      st.size,
    ));

    /* Enter new scope. */
    let scope = scope.new_scope(st);

    /* Generated statement. */
    statement.generate(&scope, code, regs, ());

    /* Increment stack pointer to old position. */
    code.text.append(&mut Op2::imm_unroll(
      |offset| Asm::add(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset)),
      st.size,
    ));
  }
}

fn generate_stat_declaration(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  id: &str,
  rhs: &AssignRhs,
) {
  Stat::Assignment(AssignLhs::Ident(id.to_string()), t.clone(), rhs.clone())
    .generate(scope, code, regs, ());
}

fn generate_stat_assignment(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  lhs: &AssignLhs,
  t: &Type,
  rhs: &AssignRhs,
) {
  /* regs[0] = eval(rhs) */
  rhs.generate(scope, code, regs, ());

  /* stores value of regs[0] into lhs */
  let (ptr_reg, offset, data_size) =
    lhs.generate(scope, code, &regs[1..], t.clone());
  code
    .text
    .push(Asm::str(Reg::General(regs[0]), (ptr_reg, offset)).size(data_size));
}

fn generate_stat_read(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  type_: &Type,
  lhs: &AssignLhs,
) {
  let (ptr_reg, offset, _) = lhs.generate(scope, code, regs, type_.clone());

  if offset != 0 || Reg::General(regs[0]) != ptr_reg {
    code
      .text
      .push(Asm::add(Reg::General(regs[0]), ptr_reg, Op2::Imm(offset)));
  }

  /* MOV r0, {regs[0]} */
  code.text.push(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::General(regs[0]), 0),
  ));
  //expr.get_type //todo!() get type of ident
  let read_type = if *type_ == Type::Char {
    RequiredPredefs::ReadChar.mark(code);
    ReadFmt::Char
  } else if *type_ == Type::Int {
    RequiredPredefs::ReadInt.mark(code);
    ReadFmt::Int
  } else {
    unreachable!("CAN'T GET THIS TYPE!");
  };

  /* BL p_read_{read_type} */
  code
    .text
    .push(Asm::b(format!("p_read_{}", read_type)).link())
}

fn generate_stat_free(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) {
  expr.generate(scope, code, regs, ());

  /* MOV r0, {min_reg}        //move heap address into r0 */
  code.text.push(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::General(regs[0]), 0),
  ));
  match *t {
    Type::Array(_) => {
      RequiredPredefs::FreeArray.mark(code);

      /* BL p_free_array */
      code.text.push(Asm::b(PREDEF_FREE_ARRAY).link());
    }
    Type::Pair(_, _) => {
      RequiredPredefs::FreePair.mark(code);

      /* BL p_free_pair */
      code.text.push(Asm::b(PREDEF_FREE_PAIR).link());
    }
    _ => unreachable!("Can't free this type!"),
  }
}

fn generate_stat_return(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  expr: &Expr,
) {
  /* regs[0] = eval(expr) */
  expr.generate(scope, code, regs, ());

  /* r0 = regs[0] */
  code.text.push(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::General(regs[0]), 0),
  ));

  let total_offset = scope.get_total_offset();

  /* ADD sp, sp, #{total_offset} */
  code.text.append(&mut Op2::imm_unroll(
    |offset| Asm::add(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset)),
    total_offset,
  ));

  /* POP {pc} */
  code.text.push(Asm::pop(Reg::PC));
}

fn generate_stat_exit(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  expr: &Expr,
) {
  /* regs[0] = eval(expr) */
  expr.generate(scope, code, regs, ());

  /* r0 = regs[0] */
  code.text.push(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::General(regs[0]), 0),
  ));

  /* BL exit */
  code.text.push(Asm::b(predef::PREDEF_SYS_EXIT).link());
}

fn generate_stat_print(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) {
  expr.generate(scope, code, regs, ());

  code.text.push(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::General(regs[0]), 0),
  ));

  match t {
    Type::Int => RequiredPredefs::PrintInt.mark(code),
    Type::Bool => RequiredPredefs::PrintBool.mark(code),
    Type::String => RequiredPredefs::PrintString.mark(code),
    Type::Array(elem_type) => match **elem_type {
      Type::Char => RequiredPredefs::PrintString.mark(code),
      _ => RequiredPredefs::PrintRefs.mark(code),
    },
    Type::Pair(_, _) => RequiredPredefs::PrintRefs.mark(code),
    _ => (),
  };

  let print_label = match t {
    Type::Int => predef::PREDEF_PRINT_INT,
    Type::Bool => predef::PREDEF_PRINT_BOOL,
    Type::String => predef::PREDEF_PRINT_STRING,
    Type::Char => predef::PREDEF_SYS_PUTCHAR,
    Type::Array(elem_type) => match **elem_type {
      Type::Char => predef::PREDEF_PRINT_STRING,
      _ => predef::PREDEF_PRINT_REFS,
    },
    Type::Pair(_, _) => predef::PREDEF_PRINT_REFS,
    _ => unreachable!(),
  };

  code
    .text
    .push(Asm::instr(Branch(true, print_label.to_string())));
}

fn generate_stat_println(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) {
  generate_stat_print(scope, code, regs, t, expr);

  /* BL println */
  RequiredPredefs::PrintLn.mark(code);
  code.text.push(Asm::b(predef::PREDEF_PRINTLN).link());
}

fn generate_stat_if(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  cond: &Expr,
  true_body: &ScopedStat,
  false_body: &ScopedStat,
) {
  let false_label = code.get_label();
  let exit_label = code.get_label();

  /* regs[0] = eval(cond) */
  cond.generate(scope, code, regs, ());

  /* cmp(regs[0], 0) */
  code.text.push(Asm::cmp(Reg::General(regs[0]), Op2::Imm(0)));

  /* Branch to false case if cond == 0. */
  code
    .text
    .push(Asm::Instr(CondCode::EQ, Branch(false, false_label.clone())));

  /* True body. */
  true_body.generate(scope, code, regs, ());

  /* Exit if statement. */
  code
    .text
    .push(Asm::instr(Branch(false, exit_label.clone())));

  /* Label for false case to skip to. */
  code.text.push(Asm::Directive(Label(false_label)));

  /* False body. */
  false_body.generate(scope, code, regs, ());

  /* Label to exit if statement. */
  code.text.push(Asm::Directive(Label(exit_label)));
}

fn generate_stat_while(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  cond: &Expr,
  body: &ScopedStat,
) {
  let cond_label = code.get_label();
  let body_label = code.get_label();

  /* Jump to condition evaluation. */
  code.text.push(Asm::b(cond_label.clone()));

  /* Loop body label. */
  code.text.push(Asm::Directive(Label(body_label.clone())));

  /* Loop body. */
  body.generate(scope, code, regs, ());

  /* Cond label */
  code.text.push(Asm::Directive(Label(cond_label)));

  /* regs[0] = eval(cond) */
  cond.generate(scope, code, regs, ());

  /* cmp(regs[0], 1) */
  code.text.push(Asm::cmp(Reg::General(regs[0]), Op2::Imm(1)));

  /* If regs[0] == 1, jump back to loop body. */
  code
    .text
    .push(Asm::Instr(CondCode::EQ, Branch(false, body_label)));
}

fn generate_stat_scope(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  stat: &ScopedStat,
) {
  stat.generate(scope, code, regs, ())
}

fn generate_stat_sequence(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  head: &Stat,
  tail: &Stat,
) {
  head.generate(scope, code, regs, ());
  tail.generate(scope, code, regs, ());
}

fn generate_stat_for(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  decl: &Box<Stat>,
  cond: &Expr,
  body: &ScopedStat,
  assign: &Box<Stat>,
) {
  let cond_label = code.get_label();
  let body_label = code.get_label();

  //generate declaration
  decl.generate(scope, code, regs, ());

  /* Jump to condition evaluation. */
  code.text.push(Asm::b(cond_label.clone()));

  /* Loop body label */
  code.text.push(Asm::Directive(Label(body_label.clone())));

  /* Loop body. */
  body.generate(scope, code, regs, ());

  /* Generate assign */
  assign.generate(scope, code, regs, ());

  /* Cond label */
  code.text.push(Asm::Directive(Label(cond_label)));

  /* regs[0] = eval(cond) */
  cond.generate(scope, code, regs, ());

  /* cmp(regs[0], 1) */
  code.text.push(Asm::cmp(Reg::General(regs[0]), Op2::Imm(1)));

  /* If regs[0] == 1, jump back to loop body. */
  code
    .text
    .push(Asm::Instr(CondCode::EQ, Branch(false, body_label)));
}

impl Generatable for Stat {
  type Input = ();
  type Output = ();
  fn generate(
    &self,
    scope: &ScopeReader,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    _aux: (),
  ) {
    match self {
      Stat::Skip => (),
      Stat::Declaration(t, id, rhs) => {
        generate_stat_declaration(scope, code, regs, t, id, rhs);
      }
      Stat::Assignment(lhs, t, rhs) => {
        generate_stat_assignment(scope, code, regs, lhs, t, rhs)
      }
      Stat::Read(type_, lhs) => {
        generate_stat_read(scope, code, regs, type_, lhs)
      }
      Stat::Free(TypedExpr(t, expr)) => {
        generate_stat_free(scope, code, regs, t, expr)
      }
      Stat::Return(expr) => generate_stat_return(scope, code, regs, expr),
      Stat::Exit(expr) => generate_stat_exit(scope, code, regs, expr),
      Stat::Print(TypedExpr(t, expr)) => {
        generate_stat_print(scope, code, regs, t, expr)
      }
      Stat::Println(TypedExpr(t, expr)) => {
        generate_stat_println(scope, code, regs, t, expr)
      }
      Stat::If(cond, body_t, body_f) => {
        generate_stat_if(scope, code, regs, cond, body_t, body_f)
      }
      Stat::While(cond, body) => {
        generate_stat_while(scope, code, regs, cond, body)
      }
      Stat::Scope(stat) => generate_stat_scope(scope, code, regs, stat),
      Stat::Sequence(head, tail) => {
        generate_stat_sequence(scope, code, regs, head, tail)
      }
      Stat::For(decl, cond, body, assign) => {
        generate_stat_for(scope, code, regs, decl, cond, body, assign)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn exit_statement() {
    let symbol_table = SymbolTable::default();
    let scope = &ScopeReader::new(&symbol_table);
    let expr = Expr::IntLiter(0);
    let stat = Stat::Exit(expr.clone());
    let regs = &GENERAL_REGS;

    /* Actual output. */
    let mut actual_code = GeneratedCode::default();
    stat.generate(scope, &mut actual_code, regs, ());

    /* Expected output. */
    let mut expected_code = GeneratedCode::default();
    expr.generate(scope, &mut expected_code, regs, ());

    /* MOV r0, r4 */
    expected_code.text.push(Asm::mov(
      Reg::Arg(ArgReg::R0),
      Op2::Reg(Reg::General(GenReg::R4), 0),
    ));

    /* BL exit */
    expected_code
      .text
      .push(Asm::b(predef::PREDEF_SYS_EXIT).link());

    assert_eq!(format!("{}", actual_code), format!("{}", expected_code));
  }
}
