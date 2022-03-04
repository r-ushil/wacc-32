use super::{predef::ReadFmt, predef::RequiredPredefs, *};
use Directive::*;
use Instr::*;

impl Generatable for AssignLhs {
  type Input = Type;
  type Output = (Reg, Offset, DataSize);

  /* Writes regs[0] to value specified by AssignLhs */
  fn generate(
    &self,
    scope: &Scope,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    t: Type,
  ) -> Self::Output {
    match self {
      AssignLhs::Ident(id) => {
        let offset = scope.get_offset(id).unwrap();

        (Reg::StackPointer, offset, t.size().into())
      }
      AssignLhs::ArrayElem(elem) => {
        /* Store address of array element into regs[1]. */
        let elem_size = elem.generate(scope, code, &regs[1..], ());

        (Reg::General(regs[1]), 0, elem_size)
      }
      AssignLhs::PairElem(elem) => {
        /* Stores address of elem in regs[1]. */
        let elem_size = elem.generate(scope, code, &regs[1..], ());

        (Reg::General(regs[1]), 0, elem_size)
      }
    }
  }
}

/* Mallocs {bytes} bytes and leaves the address in {reg}. */
fn generate_malloc(bytes: i32, code: &mut GeneratedCode, reg: Reg) {
  /* LDR r0, ={bytes} */
  code.text.push(Asm::always(Instr::Load(
    DataSize::Word,
    Reg::Arg(ArgReg::R0),
    LoadArg::Imm(bytes),
  )));

  /* BL malloc */
  code
    .text
    .push(Asm::always(Instr::Branch(true, String::from("malloc"))));

  /* MOV {regs[0]}, r0 */
  if reg != Reg::Arg(ArgReg::R0) {
    code.text.push(Asm::always(Instr::Unary(
      UnaryInstr::Mov,
      reg,
      Op2::Reg(Reg::Arg(ArgReg::R0), 0),
      false,
    )));
  }
}

impl Generatable for AssignRhs {
  type Input = Type;
  type Output = ();

  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[GenReg], t: Type) {
    match self {
      AssignRhs::Expr(expr) => expr.generate(scope, code, regs, ()),
      AssignRhs::ArrayLiter(ArrayLiter(exprs)) => {
        /* Calculate size of elements. */
        let elem_size = match t {
          Type::Array(elem_type) => elem_type.size(),
          /* Semantic analyser should ensure this is an array. */
          _ => unreachable!(),
        };

        /* Malloc space for array. */
        generate_malloc(
          4 + elem_size * exprs.len() as i32,
          code,
          Reg::General(regs[0]),
        );

        /* Write each expression to the array. */
        for (i, expr) in exprs.iter().enumerate() {
          /* Evaluate expr to r5. */
          expr.generate(scope, code, &regs[1..], ());

          /* Write r5 array. */
          code.text.push(Asm::always(Instr::Store(
            elem_size.into(),
            Reg::General(regs[1]),
            (Reg::General(regs[0]), 4 + (i as i32) * elem_size),
            AddressingMode::Default,
          )));
        }

        /* Write length to first byte.
        LDR r5, =3
        STR r5, [r4] */
        code.text.push(Asm::always(Instr::Load(
          DataSize::Word,
          Reg::General(regs[1]),
          LoadArg::Imm(exprs.len() as i32),
        )));
        code.text.push(Asm::always(Instr::Store(
          DataSize::Word,
          Reg::General(regs[1]),
          (Reg::General(regs[0]), 0),
          AddressingMode::Default,
        )));
      }
      AssignRhs::Pair(e1, e2) => {
        let (e1_size, e2_size) = match t {
          Type::Pair(t1, t2) => (t1.size(), t2.size()),
          /* Semantic analyser should ensure this is a pair. */
          _ => unreachable!(),
        };

        /* Malloc for the pair.
        regs[0] = malloc(8) */
        generate_malloc(8, code, Reg::General(regs[0]));

        /* Evaluate e1.
        regs[1] = eval(e1) */
        e1.generate(scope, code, &regs[1..], ());

        /* Malloc for e1.
        r0 = malloc(e1_size) */
        generate_malloc(e1_size, code, Reg::Arg(ArgReg::R0));

        /* Write e1 to malloced space. */
        code.text.push(Asm::always(Instr::Store(
          e1_size.into(),
          Reg::General(regs[1]),
          (Reg::Arg(ArgReg::R0), 0),
          AddressingMode::Default,
        )));

        /* Write pointer to e1 to pair. */
        code.text.push(Asm::always(Instr::Store(
          DataSize::Word,
          Reg::Arg(ArgReg::R0),
          (Reg::General(regs[0]), 0),
          AddressingMode::Default,
        )));

        /* Evaluate e2.
        regs[1] = eval(e2) */
        e2.generate(scope, code, &regs[1..], ());

        /* Malloc for e2.
        r0 = malloc(e2_size) */
        generate_malloc(e2_size, code, Reg::Arg(ArgReg::R0));

        /* Write e2 to malloced space. */
        code.text.push(Asm::always(Instr::store(
          e2_size.into(),
          Reg::General(regs[1]),
          (Reg::Arg(ArgReg::R0), 0),
        )));

        /* Write pointer to e2 to pair. */
        code.text.push(Asm::always(Instr::store(
          DataSize::Word,
          Reg::Arg(ArgReg::R0),
          (Reg::General(regs[0]), 4),
        )));
      }
      AssignRhs::PairElem(elem) => {
        /* Puts element address in regs[0]. */
        let elem_size = elem.generate(scope, code, regs, ());

        /* Dereference. */
        code.text.push(Asm::always(Instr::Load(
          elem_size,
          Reg::General(regs[0]),
          LoadArg::MemAddress(Reg::General(regs[0]), 0),
        )));
      }
      AssignRhs::Call(ident, exprs) => {
        let args = if let Type::Func(function_sig) = scope.get_bottom(ident).expect("Unreachable!")
        {
          &function_sig.params
        } else {
          unreachable!();
        };

        let mut args_offset = 0;

        for (expr, (arg_type, _arg_ident)) in exprs.iter().zip(args).rev() {
          let symbol_table = SymbolTable {
            size: args_offset,
            ..Default::default()
          };

          let arg_offset_scope = scope.new_scope(&symbol_table);

          expr.generate(&arg_offset_scope, code, regs, ());

          code.text.push(Asm::always(Instr::store_with_mode(
            arg_type.size().into(),
            Reg::General(regs[0]),
            (Reg::StackPointer, -arg_type.size()),
            AddressingMode::PreIndexed,
          )));

          /* Make symbol table bigger. */
          args_offset += arg_type.size();
        }

        code.text.push(Asm::always(Branch(
          true,
          generate_function_name(ident.to_string()),
        )));

        /* Stack space was given to parameter to call function.
        We've finished calling so we can deallocate this space now. */
        code.text.append(&mut Op2::imm_unroll(
          |offset| {
            Asm::always(Binary(
              BinaryInstr::Add,
              Reg::StackPointer,
              Reg::StackPointer,
              Op2::Imm(offset),
              false,
            ))
          },
          args_offset,
        ));

        code.text.push(Asm::always(Unary(
          UnaryInstr::Mov,
          Reg::General(regs[0]),
          Op2::Reg(Reg::Arg(ArgReg::R0), 0),
          false,
        )));
      }
    }
  }
}

impl Generatable for PairElem {
  type Input = ();
  type Output = DataSize;

  /* Puts the address of the element in regs[0], returns size pointed to. */
  fn generate(
    &self,
    scope: &Scope,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    _aux: (),
  ) -> DataSize {
    /*  */
    let (t, pair, offset) = match self {
      PairElem::Fst(t, pair) => (t, pair, 0),
      PairElem::Snd(t, pair) => (t, pair, 4),
    };

    /* Store address of pair in regs[0]. */
    pair.generate(scope, code, regs, ());

    /* CHECK: regs[0] != NULL */
    code.text.push(Asm::always(Instr::Unary(
      UnaryInstr::Mov,
      Reg::Arg(ArgReg::R0),
      Op2::Reg(Reg::General(regs[0]), 0),
      false,
    )));
    code.text.push(Asm::always(Instr::Branch(
      true,
      String::from("p_check_null_pointer"),
    )));
    RequiredPredefs::CheckNullPointer.mark(code);

    /* Dereference. */
    code.text.push(Asm::always(Instr::Load(
      DataSize::Word,
      Reg::General(regs[0]),
      LoadArg::MemAddress(Reg::General(regs[0]), offset),
    )));

    /* Return how much data needs to be read from regs[0]. */
    t.size().into()
  }
}

impl Generatable for ScopedStat {
  type Input = ();
  type Output = ();
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[GenReg], aux: ()) {
    let ScopedStat(st, statement) = self;

    /* Allocate space on stack for variables declared in this scope. */
    code.text.append(&mut Op2::imm_unroll(
      |offset| {
        Asm::always(Instr::Binary(
          BinaryInstr::Sub,
          Reg::StackPointer,
          Reg::StackPointer,
          Op2::Imm(offset),
          false,
        ))
      },
      st.size,
    ));

    /* Enter new scope. */
    let scope = scope.new_scope(st);

    /* Generated statement. */
    statement.generate(&scope, code, regs, ());

    /* Increment stack pointer to old position. */
    code.text.append(&mut Op2::imm_unroll(
      |offset| {
        Asm::always(Instr::Binary(
          BinaryInstr::Add,
          Reg::StackPointer,
          Reg::StackPointer,
          Op2::Imm(offset),
          false,
        ))
      },
      st.size,
    ));
  }
}

fn generate_stat_declaration(
  scope: &Scope,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  id: &str,
  rhs: &AssignRhs,
) {
  Stat::Assignment(AssignLhs::Ident(id.to_string()), t.clone(), rhs.clone()).generate(
    scope,
    code,
    regs,
    (),
  );
}

fn generate_stat_assignment(
  scope: &Scope,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  lhs: &AssignLhs,
  t: &Type,
  rhs: &AssignRhs,
) {
  /* regs[0] = eval(rhs) */
  rhs.generate(scope, code, regs, t.clone());

  /* stores value of regs[0] into lhs */
  let (ptr_reg, offset, data_size) = lhs.generate(scope, code, regs, t.clone());
  code.text.push(Asm::always(Instr::Store(
    data_size,
    Reg::General(regs[0]),
    (ptr_reg, offset),
    AddressingMode::Default,
  )));
}

fn generate_stat_read(
  scope: &Scope,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  type_: &Type,
  lhs: &AssignLhs,
) {
  let (ptr_reg, offset, _) = lhs.generate(scope, code, regs, type_.clone());

  code.text.push(Asm::always(Instr::Binary(
    BinaryInstr::Add,
    Reg::General(regs[0]),
    ptr_reg,
    Op2::Imm(offset),
    false,
  )));

  /* MOV r0, {regs[0]} */
  code.text.push(Asm::Instr(
    CondCode::AL,
    Instr::Unary(
      UnaryInstr::Mov,
      Reg::Arg(ArgReg::R0),
      Op2::Reg(Reg::General(regs[0]), 0),
      false,
    ),
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
  code.text.push(Asm::always(Instr::Branch(
    true,
    format!("p_read_{}", read_type),
  )))
}

fn generate_stat_free(
  scope: &Scope,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) {
  expr.generate(scope, code, regs, ());

  /* MOV r0, {min_reg}        //move heap address into r0 */
  code.text.push(Asm::Instr(
    CondCode::AL,
    Instr::Unary(
      UnaryInstr::Mov,
      Reg::Arg(ArgReg::R0),
      Op2::Reg(Reg::General(regs[0]), 0),
      false,
    ),
  ));
  match *t {
    Type::Array(_) => {
      RequiredPredefs::FreeArray.mark(code);

      /* BL p_free_array */
      code.text.push(Asm::always(Instr::Branch(
        true,
        String::from("p_free_array"),
      )));
    }
    Type::Pair(_, _) => {
      RequiredPredefs::FreePair.mark(code);

      /* BL p_free_pair */
      code.text.push(Asm::always(Instr::Branch(
        true,
        String::from("p_free_pair"),
      )));
    }
    _ => unreachable!("Can't free this type!"),
  }
}

fn generate_stat_return(scope: &Scope, code: &mut GeneratedCode, regs: &[GenReg], expr: &Expr) {
  /* regs[0] = eval(expr) */
  expr.generate(scope, code, regs, ());

  /* r0 = regs[0] */
  code.text.push(Asm::Instr(
    CondCode::AL,
    Instr::Unary(
      UnaryInstr::Mov,
      Reg::Arg(ArgReg::R0),
      Op2::Reg(Reg::General(regs[0]), 0),
      false,
    ),
  ));

  let total_offset = scope.get_total_offset();

  /* ADD sp, sp, #{total_offset} */
  code.text.append(&mut Op2::imm_unroll(
    |offset| {
      Asm::always(Instr::Binary(
        BinaryInstr::Add,
        Reg::StackPointer,
        Reg::StackPointer,
        Op2::Imm(offset),
        false,
      ))
    },
    total_offset,
  ));

  /* POP {pc} */
  code
    .text
    .push(Asm::Instr(CondCode::AL, Instr::Pop(Reg::PC)));
}

fn generate_stat_exit(scope: &Scope, code: &mut GeneratedCode, regs: &[GenReg], expr: &Expr) {
  /* regs[0] = eval(expr) */
  expr.generate(scope, code, regs, ());

  /* r0 = regs[0] */
  code.text.push(Asm::Instr(
    CondCode::AL,
    Instr::Unary(
      UnaryInstr::Mov,
      Reg::Arg(ArgReg::R0),
      Op2::Reg(Reg::General(regs[0]), 0),
      false,
    ),
  ));

  /* B exit */
  code.text.push(Asm::Instr(
    CondCode::AL,
    Instr::Branch(true, String::from("exit")),
  ));
}

fn generate_stat_print(
  scope: &Scope,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) {
  expr.generate(scope, code, regs, ());

  code.text.push(Asm::always(Unary(
    UnaryInstr::Mov,
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::General(regs[0]), 0),
    false,
  )));

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
    Type::Char => predef::PREDEF_PRINT_CHAR,
    Type::Array(elem_type) => match **elem_type {
      Type::Char => predef::PREDEF_PRINT_STRING,
      _ => predef::PREDEF_PRINT_REFS,
    },
    Type::Pair(_, _) => predef::PREDEF_PRINT_REFS,
    _ => unreachable!(),
  };

  code
    .text
    .push(Asm::always(Branch(true, print_label.to_string())));
}

fn generate_stat_println(
  scope: &Scope,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) {
  generate_stat_print(scope, code, regs, t, expr);

  /* BL println */
  RequiredPredefs::PrintLn.mark(code);
  code.text.push(Asm::always(Instr::Branch(
    true,
    predef::PREDEF_PRINTLN.to_string(),
  )));
}

fn generate_stat_if(
  scope: &Scope,
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
  code.text.push(Asm::always(Unary(
    UnaryInstr::Cmp,
    Reg::General(regs[0]),
    Op2::Imm(0),
    false,
  )));

  /* Branch to false case if cond == 0. */
  code
    .text
    .push(Asm::Instr(CondCode::EQ, Branch(false, false_label.clone())));

  /* True body. */
  true_body.generate(scope, code, regs, ());

  /* Exit if statement. */
  code
    .text
    .push(Asm::always(Branch(false, exit_label.clone())));

  /* Label for false case to skip to. */
  code.text.push(Asm::Directive(Label(false_label)));

  /* False body. */
  false_body.generate(scope, code, regs, ());

  /* Label to exit if statement. */
  code.text.push(Asm::Directive(Label(exit_label)));
}

fn generate_stat_while(
  scope: &Scope,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  cond: &Expr,
  body: &ScopedStat,
) {
  let cond_label = code.get_label();
  let body_label = code.get_label();

  /* Jump to condition evaluation. */
  code
    .text
    .push(Asm::always(Instr::Branch(false, cond_label.clone())));

  /* Loop body label. */
  code.text.push(Asm::Directive(Label(body_label.clone())));

  /* Loop body. */
  body.generate(scope, code, regs, ());

  /* Cond label */
  code.text.push(Asm::Directive(Label(cond_label)));

  /* regs[0] = eval(cond) */
  cond.generate(scope, code, regs, ());

  /* cmp(regs[0], 1) */
  code.text.push(Asm::always(Unary(
    UnaryInstr::Cmp,
    Reg::General(regs[0]),
    Op2::Imm(1),
    false,
  )));

  /* If regs[0] == 1, jump back to loop body. */
  code
    .text
    .push(Asm::Instr(CondCode::EQ, Branch(false, body_label.clone())));
}

fn generate_stat_scope(
  scope: &Scope,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  stat: &ScopedStat,
) {
  stat.generate(scope, code, regs, ())
}

fn generate_stat_sequence(
  scope: &Scope,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  head: &Stat,
  tail: &Stat,
) {
  head.generate(scope, code, regs, ());
  tail.generate(scope, code, regs, ());
}

impl Generatable for Stat {
  type Input = ();
  type Output = ();
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[GenReg], aux: ()) {
    match self {
      Stat::Skip => (),
      Stat::Declaration(t, id, rhs) => generate_stat_declaration(scope, code, regs, t, id, rhs),
      Stat::Assignment(lhs, t, rhs) => generate_stat_assignment(scope, code, regs, lhs, t, rhs),
      Stat::Read(type_, lhs) => generate_stat_read(scope, code, regs, type_, lhs),
      Stat::Free(t, expr) => generate_stat_free(scope, code, regs, t, expr),
      Stat::Return(expr) => generate_stat_return(scope, code, regs, expr),
      Stat::Exit(expr) => generate_stat_exit(scope, code, regs, expr),
      Stat::Print(t, expr) => generate_stat_print(scope, code, regs, t, expr),
      Stat::Println(t, expr) => generate_stat_println(scope, code, regs, t, expr),
      Stat::If(cond, body_t, body_f) => generate_stat_if(scope, code, regs, cond, body_t, body_f),
      Stat::While(cond, body) => generate_stat_while(scope, code, regs, cond, body),
      Stat::Scope(stat) => generate_stat_scope(scope, code, regs, stat),
      Stat::Sequence(head, tail) => generate_stat_sequence(scope, code, regs, head, tail),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn exit_statement() {
    let symbol_table = SymbolTable::default();
    let scope = &Scope::new(&symbol_table);
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
    expected_code.text.push(Asm::Instr(
      CondCode::AL,
      Instr::Unary(
        UnaryInstr::Mov,
        Reg::Arg(ArgReg::R0),
        Op2::Reg(Reg::General(GenReg::R4), 0),
        false,
      ),
    ));

    /* B exit */
    expected_code.text.push(Asm::Instr(
      CondCode::AL,
      Instr::Branch(true, String::from("exit")),
    ));

    assert_eq!(format!("{}", actual_code), format!("{}", expected_code));
  }
}
