use super::{
  predef::{ReadFmt, PREDEF_FREE_ARRAY, PREDEF_FREE_PAIR},
  predef::{RequiredPredefs, PREDEF_SYS_MALLOC},
  *,
};
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

impl CFGable for ScopedStat {
  type Input = ();

  fn cfg_generate<'a, 'cfg>(
    &self,
    scope: &ScopeReader,
    cfg: &'a mut CFG<'cfg>,
    regs: &[GenReg],
    _aux: (),
  ) -> Flow<'cfg> {
    let ScopedStat(st, statement) = self;

    /* Allocate space on stack for variables declared in this scope. */
    let flow = cfg.imm_unroll(
      |offset| Asm::sub(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset)),
      st.size,
    );

    /* Enter new scope. */
    let scope = scope.new_scope(st);

    flow
      /* Generated statement. */
      + statement.cfg_generate(&scope, cfg, regs, ())

      /* Increment stack pointer to old position. */
      + cfg.imm_unroll(
        |offset| Asm::add(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset)),
        st.size,
      )
  }
}

fn generate_stat_assignment<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  lhs: &Expr,
  rhs: &Expr,
) -> Flow<'cfg> {
  /* regs[0] = eval(rhs) */
  rhs.cfg_generate(scope, cfg, regs, None)

  /* stores value of regs[0] into lhs */
  + lhs.cfg_generate(scope, cfg, &regs[1..], Some(Reg::General(regs[0])))
}

fn generate_stat_read<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  TypedExpr(dst_type, dst_expr): &TypedExpr,
) -> Flow<'cfg> {
  /* Allocate space on stack for p_read_{} to write into. */
  cfg.flow(Asm::sub(Reg::StackPointer, Reg::StackPointer, Op2::Imm(4)))

  /* Store stack pointer to r0 to pass to p_read_{} */
  + cfg.flow(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::StackPointer, 0),
  ))

  /* Determine if we need p_read_char or p_read_int, and mark it. */
  + {
    let read_type = match dst_type {
      Type::Char => {
        RequiredPredefs::ReadChar.mark(cfg.code);
        ReadFmt::Char
      }
      Type::Int => {
        RequiredPredefs::ReadInt.mark(cfg.code);
        ReadFmt::Int
      }
      _ => unreachable!(
        "Analyser has allowed reading from console to int to char variable."
      ),
    };

    /* Branch to the appropriate read branch. */
    cfg.flow(Asm::b(format!("p_read_{}", read_type)).link())
  }

  /* Save the read value into a register. */
  + {
    let value_reg = Reg::General(regs[0]);
    cfg.flow(Asm::ldr(value_reg, Reg::StackPointer))

    /* Deallocate space for this value. */
    + cfg.flow(Asm::add(Reg::StackPointer, Reg::StackPointer, Op2::Imm(4)))

    /* Write this value to the destination expression. */
    + dst_expr.cfg_generate(scope, cfg, regs, Some(value_reg))
  }
}

fn generate_stat_free<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) -> Flow<'cfg> {
  expr.cfg_generate(scope, cfg, regs, None)

  /* MOV r0, {min_reg}        //move heap address into r0 */
  + cfg.flow(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::General(regs[0]), 0),
  ))

  + match *t {
    Type::Array(_) => {
      RequiredPredefs::FreeArray.mark(cfg.code);

      /* BL p_free_array */
      cfg.flow(Asm::b(PREDEF_FREE_ARRAY).link())
    }
    Type::Pair(_, _) => {
      RequiredPredefs::FreePair.mark(cfg.code);

      /* BL p_free_pair */
      cfg.flow(Asm::b(PREDEF_FREE_PAIR).link())
    }
    _ => unreachable!("Can't free this type!"),
  }
}

fn generate_stat_return<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  expr: &Expr,
) -> Flow<'cfg> {
  /* regs[0] = eval(expr) */
  expr.cfg_generate(scope, cfg, regs, None)

  /* r0 = regs[0] */
  + cfg.flow(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::General(regs[0]), 0),
  ))

  + {
    let total_offset = scope.get_total_offset();

    /* ADD sp, sp, #{total_offset} */
    cfg.imm_unroll(
      |offset| Asm::add(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset)),
      total_offset,
    )
  }

  /* POP {pc} */
  + cfg.flow(Asm::pop(Reg::PC))
}

fn generate_stat_exit<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  expr: &Expr,
) -> Flow<'cfg> {
  /* regs[0] = eval(expr) */
  expr.cfg_generate(scope, cfg, regs, None)

  /* r0 = regs[0] */
  + cfg.flow(Asm::mov(
    Reg::Arg(ArgReg::R0),
    Op2::Reg(Reg::General(regs[0]), 0),
  ))

  /* BL exit */
  + cfg.flow(Asm::b(predef::PREDEF_SYS_EXIT).link())
}

fn generate_stat_print<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) -> Flow<'cfg> {
  expr.cfg_generate(scope, cfg, regs, None)
    + cfg.flow(Asm::mov(
      Reg::Arg(ArgReg::R0),
      Op2::Reg(Reg::General(regs[0]), 0),
    ))
    + {
      match t {
        Type::Int => RequiredPredefs::PrintInt.mark(cfg.code),
        Type::Bool => RequiredPredefs::PrintBool.mark(cfg.code),
        Type::String => RequiredPredefs::PrintString.mark(cfg.code),
        Type::Array(elem_type) => match **elem_type {
          Type::Char => RequiredPredefs::PrintString.mark(cfg.code),
          _ => RequiredPredefs::PrintRefs.mark(cfg.code),
        },
        Type::Pair(_, _) => RequiredPredefs::PrintRefs.mark(cfg.code),
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

      cfg.flow(Asm::instr(Branch(true, print_label.to_string())))
    }
}

fn generate_stat_println<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  t: &Type,
  expr: &Expr,
) -> Flow<'cfg> {
  RequiredPredefs::PrintLn.mark(cfg.code);

  generate_stat_print(scope, cfg, regs, t, expr)

  /* BL println */
  + cfg.flow(Asm::b(predef::PREDEF_PRINTLN).link())
}

fn generate_stat_if<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  cond: &Expr,
  true_body: &ScopedStat,
  false_body: &ScopedStat,
) -> Flow<'cfg> {
  let cond_flow =
    /* regs[0] = eval(cond) */
    cond.cfg_generate(scope, cfg, regs, None)
    /* cmp(regs[0], 0) */
    + cfg.flow(Asm::cmp(Reg::General(regs[0]), Op2::Imm(0)));

  /* True body. */
  let true_flow = true_body.cfg_generate(scope, cfg, regs, ());

  /* False body. */
  let false_flow = false_body.cfg_generate(scope, cfg, regs, ());

  /* Block to jump to. */
  let exit_flow = cfg.dummy_flow();

  /* Link cond -> true & false. */
  cond_flow.add_succ_cond(CondCode::EQ, &false_flow);
  cond_flow.add_succ_cond(CondCode::NE, &true_flow);

  /* Link true & false -> exit. */
  true_flow.add_succ(&exit_flow);
  false_flow.add_succ(&exit_flow);

  cond_flow.tunnel(&exit_flow)
}

fn generate_stat_while<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  cond: &Expr,
  body: &ScopedStat,
) -> Flow<'cfg> {
  let body_flow =
    /* Loop body. */
    body.cfg_generate(scope, cfg, regs, ());

  let cond_flow =
    /* regs[0] = eval(cond) */
    cond.cfg_generate(scope, cfg, regs, None)
    /* cmp(regs[0], 1) */
    + cfg.flow(Asm::cmp(Reg::General(regs[0]), Op2::Imm(1)));

  /* Two way link from cond to body. */
  cond_flow.add_succ_cond(CondCode::EQ, &body_flow);
  body_flow.add_succ(&cond_flow);

  /* Start to end link. */
  cond_flow
}

fn generate_stat_scope<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  stat: &ScopedStat,
) -> Flow<'cfg> {
  stat.cfg_generate(scope, cfg, regs, ())
}

fn generate_stat_sequence<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  head: &Stat,
  tail: &Stat,
) -> Flow<'cfg> {
  head.cfg_generate(scope, cfg, regs, ())
    + tail.cfg_generate(scope, cfg, regs, ())
}

impl CFGable for Stat {
  type Input = ();

  fn cfg_generate<'a, 'cfg>(
    &self,
    scope: &ScopeReader,
    cfg: &'a mut CFG<'cfg>,
    regs: &[GenReg],
    _aux: (),
  ) -> Flow<'cfg> {
    match self {
      Stat::Skip => cfg.dummy_flow(),
      Stat::Declaration(_, lhs, rhs) | Stat::Assignment(lhs, _, rhs) => {
        generate_stat_assignment(scope, cfg, regs, lhs, rhs)
      }
      Stat::Read(dst) => generate_stat_read(scope, cfg, regs, dst),
      Stat::Free(TypedExpr(t, expr)) => {
        generate_stat_free(scope, cfg, regs, t, expr)
      }
      Stat::Return(expr) => generate_stat_return(scope, cfg, regs, expr),
      Stat::Exit(expr) => generate_stat_exit(scope, cfg, regs, expr),
      Stat::Print(TypedExpr(t, expr)) => {
        generate_stat_print(scope, cfg, regs, t, expr)
      }
      Stat::Println(TypedExpr(t, expr)) => {
        generate_stat_println(scope, cfg, regs, t, expr)
      }
      Stat::If(cond, body_t, body_f) => {
        generate_stat_if(scope, cfg, regs, cond, body_t, body_f)
      }
      Stat::While(cond, body) => {
        generate_stat_while(scope, cfg, regs, cond, body)
      }
      Stat::Scope(stat) => generate_stat_scope(scope, cfg, regs, stat),
      Stat::Sequence(head, tail) => {
        generate_stat_sequence(scope, cfg, regs, head, tail)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use typed_arena::Arena;

  use super::*;

  #[test]
  fn exit_statement() {
    let symbol_table = SymbolTable::default();
    let scope = &ScopeReader::new(&symbol_table);
    let expr = Expr::IntLiter(0);
    let stat = Stat::Exit(expr.clone());
    let regs = &GENERAL_REGS;
    let arena = Arena::new();

    /* Actual output. */
    let mut actual_code = GeneratedCode::default();
    let mut actual_cfg = CFG::new(&mut actual_code, &arena);
    let _ = stat.cfg_generate(scope, &mut actual_cfg, regs, ());
    actual_cfg.linearise();

    /* Expected output. */
    let mut expected_code = GeneratedCode::default();
    let mut expected_cfg = CFG::new(&mut expected_code, &arena);
    let _ =
      /* Evaluate exit code. */
      expr.cfg_generate(scope, &mut expected_cfg, regs, None)
      /* MOV r0, r4 */
      + expected_cfg.flow(Asm::mov(
        Reg::Arg(ArgReg::R0),
        Op2::Reg(Reg::General(GenReg::R4), 0),
      ))
      /* BL exit */
      + expected_cfg.flow(Asm::b(predef::PREDEF_SYS_EXIT).link());

    expected_cfg.linearise();

    assert_eq!(format!("{}", actual_code), format!("{}", expected_code));
  }
}
