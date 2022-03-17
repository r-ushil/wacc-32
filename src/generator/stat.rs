use super::{
  predef::{
    ReadFmt, PREDEF_CHECK_NULL_POINTER, PREDEF_FREE_ARRAY, PREDEF_FREE_PAIR,
  },
  predef::{RequiredPredefs, PREDEF_SYS_MALLOC},
  *,
};
use Directive::*;
use Instr::*;

/* Mallocs {bytes} bytes and leaves the address in {reg}. */
pub fn generate_malloc<'a, 'cfg>(
  bytes: i32,
  cfg: &'a mut CFG<'cfg>,
  reg: Reg,
) -> Flow<'cfg> {
  /* LDR r0, ={bytes} */
  let mut flow = cfg.flow(Asm::ldr(Reg::Arg(ArgReg::R0), bytes))

  /* BL malloc */
  + cfg.flow(Asm::b(PREDEF_SYS_MALLOC).link());

  /* MOV {regs[0]}, r0 */
  if reg != Reg::Arg(ArgReg::R0) {
    flow += cfg.flow(Asm::mov(reg, Op2::Reg(Reg::Arg(ArgReg::R0), 0)));
  }

  flow
}

pub fn generate_malloc_with_reg<'a, 'cfg>(
  type_size: Reg,
  exprs_size: Reg,
  cfg: &'a mut CFG<'cfg>,
  reg: Reg,
) -> Flow<'cfg> {
  /* Mallocs {bytes} bytes and leaves the address in {reg}. */
  /* MOV r1, {bytes} */
  let mut flow = cfg.flow(Asm::mov(Reg::Arg(ArgReg::R1), Op2::Reg(type_size, 0)))

  /* MOV r0, {reg} */
  + cfg.flow(Asm::mov(Reg::Arg(ArgReg::R0), Op2::Reg(exprs_size, 0)))

  /* SMULL r0, r1, r0, r1 */
  + cfg.flow(Asm::smull(
    Reg::Arg(ArgReg::R0),
    Reg::Arg(ArgReg::R1),
    Reg::Arg(ArgReg::R0),
    Reg::Arg(ArgReg::R1),
  ))

  /* ADD r0, r0, #4 */
  + cfg.flow(Asm::add(
    Reg::Arg(ArgReg::R0),
    Reg::Arg(ArgReg::R0),
    Op2::Imm(ARM_DSIZE_WORD),
  ))

  /* BL malloc */
  + cfg.flow(Asm::b(PREDEF_SYS_MALLOC).link());

  /* MOV {regs[0]}, r0 */
  if reg != Reg::Arg(ArgReg::R0) {
    flow += cfg.flow(Asm::mov(reg, Op2::Reg(Reg::Arg(ArgReg::R0), 0)));
  }

  flow
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

fn generate_stat_assignment(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  lhs: &Expr,
  rhs: &Expr,
) {
  /* regs[0] = eval(rhs) */
  rhs.generate(scope, code, regs, None);

  /* stores value of regs[0] into lhs */
  lhs.generate(scope, code, &regs[1..], Some(Reg::General(regs[0])));
}

fn generate_stat_read(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  TypedExpr(dst_type, dst_expr): &TypedExpr,
) {
  /* Allocate space on stack for p_read_{} to write into. */
  code
    .text
    .push(Asm::sub(Reg::StackPointer, Reg::StackPointer, Op2::Imm(4)));

  /* Store stack pointer to r0 to pass to p_read_{} */
  code.text.push(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::StackPointer, 0),
  ));

  /* Determine if we need p_read_char or p_read_int, and mark it. */
  let read_type = match dst_type {
    Type::Char => {
      RequiredPredefs::ReadChar.mark(code);
      ReadFmt::Char
    }
    Type::Int => {
      RequiredPredefs::ReadInt.mark(code);
      ReadFmt::Int
    }
    _ => unreachable!(
      "Analyser has allowed reading from console to int to char variable."
    ),
  };

  /* Branch to the appropriate read branch. */
  code
    .text
    .push(Asm::b(format!("p_read_{}", read_type)).link());

  /* Save the read value into a register. */
  let value_reg = Reg::General(regs[0]);
  code.text.push(Asm::ldr(value_reg, Reg::StackPointer));

  /* Deallocate space for this value. */
  code
    .text
    .push(Asm::add(Reg::StackPointer, Reg::StackPointer, Op2::Imm(4)));

  /* Write this value to the destination expression. */
  dst_expr.generate(scope, code, regs, Some(value_reg));
}

fn generate_stat_free(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) {
  expr.generate(scope, code, regs, None);

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
  expr.generate(scope, code, regs, None);

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
  expr.generate(scope, code, regs, None);

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
  expr.generate(scope, code, regs, None);

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
  cond.generate(scope, code, regs, None);

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
  cond.generate(scope, code, regs, None);

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
      Stat::Declaration(_, dst, rhs) => {
        generate_stat_assignment(scope, code, regs, &dst, rhs);
      }
      Stat::Assignment(lhs, _, rhs) => {
        generate_stat_assignment(scope, code, regs, lhs, rhs)
      }
      Stat::Read(dst) => generate_stat_read(scope, code, regs, dst),
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
    expr.generate(scope, &mut expected_code, regs, None);

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
