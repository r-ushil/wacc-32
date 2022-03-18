use std::collections::HashSet;

use self::CondCode::*;
use super::predef::{
  RequiredPredefs, PREDEF_AEABI_IDIV, PREDEF_AEABI_IDIVMOD,
  PREDEF_CHECK_ARRAY_BOUNDS, PREDEF_CHECK_DIVIDE_BY_ZERO,
  PREDEF_CHECK_NULL_POINTER, PREDEF_THROW_OVERFLOW_ERR,
};
use super::*;
use crate::analyser::context::*;
use crate::generator::asm::*;
use crate::generator::program::LabelPrefix;
use crate::generator::stat::generate_malloc_with_reg;
use stat::generate_malloc;

pub enum ExprArg {
  /* Write to this register from the expression. */
  Src(Reg),
  /* Read from this register into the expression. */
  Dst(Reg),
}
use ExprArg::*;

impl ExprArg {
  fn dst(self) -> Reg {
    match self {
      Dst(reg) => reg,
      Src(_) => panic!("Cannot write to this expression."),
    }
  }

  fn reg(&self) -> Reg {
    match self {
      Dst(reg) | Src(reg) => reg.clone(),
    }
  }
}

impl CFGable for StructLiter {
  type Input = Reg;

  fn cfg_generate<'a, 'cfg>(
    &self,
    scope: &ScopeReader,
    cfg: &'a mut CFG<'cfg>,
    dst: Reg,
  ) -> Flow<'cfg> {
    let StructLiter { id, fields } = self;

    /* Get size of struct. */
    let struct_def = scope
      .get_def(id)
      .expect("Analyser should ensure all struct usages are valid.");

    /* Malloc for the struct. */
    let mut flow = generate_malloc(struct_def.size, cfg, dst.clone().into());

    /* Expression evaluation can't use register malloc */
    let expr_reg = cfg.get_veg();

    /* For each field: */
    for (field_name, expr) in fields.iter() {
      /* Calculate offset. */
      let offset = struct_def.fields.get(field_name).unwrap().1;

      /* Evaluate expression. */
      flow += expr.cfg_generate(scope, cfg, Dst(expr_reg.clone()))

      /* Write to struct. */
      + cfg.flow(Asm::str(expr_reg.clone(), (dst.clone(), offset)));
    }

    flow
  }
}

impl CFGable for Expr {
  type Input = ExprArg;

  fn cfg_generate<'a, 'cfg>(
    &self,
    scope: &ScopeReader,
    cfg: &'a mut CFG<'cfg>,
    arg: ExprArg,
  ) -> Flow<'cfg> {
    match self {
      /* Identifiers, at this point only local variables and labels. */
      Expr::Ident(id) => generate_ident(scope, cfg, id, arg),
      /* Literal values. */
      Expr::IntLiter(val) => cfg.flow(Asm::ldr(arg.dst(), *val)),
      Expr::BoolLiter(val) => {
        cfg.flow(Asm::mov(arg.dst(), if *val { 1 } else { 0 }))
      }
      Expr::CharLiter(val) => generate_char_liter(cfg, val, arg.dst()),
      Expr::StrLiter(val) => generate_string_liter(cfg, val, arg.dst()),
      Expr::NullPairLiter => cfg.flow(Asm::ldr(arg.dst(), 0)),
      /* Container literals. */
      Expr::ArrayLiter(ArrayLiter(t, exprs)) => {
        generate_array_liter(scope, cfg, t, exprs, arg.dst())
      }
      Expr::StructLiter(liter) => liter.cfg_generate(scope, cfg, arg.dst()),
      Expr::UnaryApp(op, expr) => {
        generate_unary_app(cfg, scope, op, expr, arg.dst())
      }
      Expr::BinaryApp(expr1, op, expr2) => {
        generate_binary_app(cfg, scope, expr1, op, expr2, arg.dst())
      }
      Expr::PairLiter(e1, e2) => {
        generate_pair_liter(scope, cfg, e1, e2, arg.dst())
      }
      Expr::ArrayElem(elem_type, arr_expr, idx_expr) => {
        generate_array_elem(scope, cfg, elem_type, arr_expr, idx_expr, arg)
      }
      Expr::StructElem(elem) => generate_struct_elem(scope, cfg, elem, arg),
      Expr::PairElem(elem) => generate_pair_elem(scope, cfg, elem, arg),
      Expr::Call(func_type, ident, exprs) => {
        generate_call(scope, cfg, func_type.clone(), ident, exprs, arg.dst())
      }
      Expr::AnonFunc(func) => {
        generate_anon_func(scope, cfg, (**func).clone(), arg.dst())
      }
      Expr::BlankArrayLiter(t, size) => {
        generate_blank_arr(scope, cfg, t, size, arg.dst())
      }
    }
  }
}

fn generate_blank_arr<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  t: &Type,
  size: &Box<Expr>,
  dst: Reg,
) -> Flow<'cfg> {
  /* LDR {regs[0]}, =type_size */
  let mut flow = cfg.flow(Asm::ldr(dst.clone(), t.size()));

  let size_reg = cfg.get_veg();

  flow += size.cfg_generate(scope, cfg, Dst(size_reg.clone()));

  /* Malloc space for array. */
  flow +=
    generate_malloc_with_reg(dst.clone(), size_reg.clone(), cfg, dst.clone());

  /* Write length to first byte.
  LDR r5, =3
  STR r5, [r4] */
  flow
    + size.cfg_generate(scope, cfg, Dst(size_reg.clone()))
    + cfg.flow(Asm::str(size_reg, (dst, 0)))
}

fn generate_anon_func<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  func: Func,
  dst: Reg,
) -> Flow<'cfg> {
  let anon_label = cfg.code.get_label();

  /* Generates function definition. */
  (anon_label.clone(), func).generate(
    scope,
    cfg.code,
    &[],
    LabelPrefix::AnonFunc,
  );

  /* Loads pointer to anonymous function into regs[0]. */
  cfg.flow(Asm::ldr(dst, generate_anon_func_name(anon_label)))
}

fn generate_call<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  func_type: Type,
  func: &Expr,
  exprs: &[Expr],
  dst: Reg,
) -> Flow<'cfg> {
  let func_reg = cfg.get_veg();
  let mut arg_regs = vec![];

  /* Put function pointer in func_reg. */
  let mut flow = func.cfg_generate(scope, cfg, Dst(func_reg));

  /*  */
  for expr in exprs {
    let expr_reg = cfg.get_veg();

    flow += expr.cfg_generate(scope, cfg, Dst(expr_reg));

    arg_regs.push(expr_reg);
  }

  /* Make into call instruction. */
  flow + cfg.flow(Asm::Call(dst, func_reg, arg_regs))

  /* Get arg types. */

  // let arg_types = match func_type {
  //   Type::Func(sig) => sig.param_types,
  //   _ => unreachable!("Analyser guarentees this is a function."),
  // };

  /* Save all registers we haven't been allowed to mangle. */
  /* Figure out which registers aren't safe to overwrite and therefore need
  saving. */
  // let mut unsafe_regs_set = GENERAL_REGS.iter().collect::<HashSet<_>>();
  /* TODO: only save the regs we need to. */
  // for reg in regs.iter() {
  //   unsafe_regs_set.remove(reg);
  // }

  /* Must put in some deterministic order so registers are popped in the
  same order as they are pushed. */
  // let unsafe_regs_vec = unsafe_regs_set.into_iter().collect::<Vec<_>>();

  /* Push all to stack. */
  /* TODO: Change Push instruction to do this with one instruction. */
  // let mut flow = cfg.dummy_flow();
  // for reg in unsafe_regs_vec.iter() {
  //   flow += cfg.flow(Asm::push(Reg::General(*reg.clone())));
  // }

  /* Now all registers are saved, we can use all registers! */
  // let safe_regs = &GENERAL_REGS;

  // let mut args_offset = 0;

  // for (expr, arg_type) in exprs.iter().zip(arg_types).rev() {
  //   let symbol_table = SymbolTable::default();

  //   let arg_offset_scope = scope.new_scope(&symbol_table);

  //   flow += expr.cfg_generate(&arg_offset_scope, cfg, Dst(safe_regs[0].into()));

  //   flow += cfg.flow(
  //     Asm::str(safe_regs[0], (Reg::StackPointer, -arg_type.size()))
  //       .size(arg_type.size().into())
  //       .pre_indexed(),
  //   );

  //   /* Make symbol table bigger. */
  //   args_offset += arg_type.size();
  // }

  /* Generate function pointer. */
  // flow += func.cfg_generate(
  //   /* Offset all stack accesses by the size the args take up. */
  //   &scope.new_scope(&SymbolTable::default()),
  //   cfg,
  //   Dst(safe_regs[0].into()),
  // );

  /* Jump to function pointer. */
  // flow += cfg.flow(Asm::bx(safe_regs[0]).link());

  /* Pop preserved register back from the stack. */
  /* TODO: Change Pop instruction to do this with one instruction. */
  // for reg in unsafe_regs_vec.iter().rev() {
  //   flow += cfg.flow(Asm::pop(Reg::General(*reg.clone())));
  // }

  /* Stack space was given to parameter to call function.
  We've finished calling so we can deallocate this space now. */
  // flow
  //   + cfg.imm_unroll(
  //     |offset| Asm::add(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset)),
  //     args_offset,
  //   )
  //   + cfg.flow(Asm::mov(dst, ArgReg::R0))
}

fn generate_pair_liter<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  TypedExpr(e1_type, e1): &TypedExpr,
  TypedExpr(e2_type, e2): &TypedExpr,
  dst: Reg,
) -> Flow<'cfg> {
  let e1_size = e1_type.size();
  let e2_size = e2_type.size();

  let elem_reg = cfg.get_veg();

  /* Malloc for the pair.
  regs[0] = malloc(8) */
  generate_malloc(8, cfg, dst.clone().into())

  /* Evaluate e1.
  regs[1] = eval(e1) */
  + e1.cfg_generate(scope, cfg, Dst(elem_reg.clone()))

  /* Malloc for e1.
  r0 = malloc(e1_size) */
  + generate_malloc(e1_size, cfg, ArgReg::R0.into())

  /* Write e1 to malloced space. */
  + cfg.flow(
    Asm::str(elem_reg.clone(), (Reg::Arg(ArgReg::R0), 0))
      .size(e1_size.into()),
  )

  /* Write pointer to e1 to pair. */
  + cfg.flow(Asm::str(ArgReg::R0, (dst.clone(), 0)))

  /* Evaluate e2.
  regs[1] = eval(e2) */
  + e2.cfg_generate(scope, cfg, Dst(elem_reg.clone()))

  /* Malloc for e2.
  r0 = malloc(e2_size) */
  + generate_malloc(e2_size, cfg, ArgReg::R0.into())

  /* Write e2 to malloced space. */
  + cfg.flow(
    Asm::str(elem_reg, (Reg::Arg(ArgReg::R0), 0))
      .size(e2_size.into()),
  )

  /* Write pointer to e2 to pair. */
  + cfg.flow(Asm::str(Reg::Arg(ArgReg::R0), (dst, ARM_DSIZE_WORD)))
}

fn generate_array_liter<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  elem_type: &Type,
  exprs: &[Expr],
  dst: Reg,
) -> Flow<'cfg> {
  let elem_reg = cfg.get_veg();

  (if exprs.len() > 0 {
    /* Calculate size of elements. */
    let elem_size = elem_type.size();

    /* Malloc space for array. */
    let mut flow = generate_malloc(
      ARM_DSIZE_WORD + elem_size * exprs.len() as i32,
      cfg,
      dst.clone(),
    );

    /* Write each expression to the array. */
    for (i, expr) in exprs.iter().enumerate() {
      /* Evaluate expr to r5. */
      flow += expr.cfg_generate(scope, cfg, Dst(elem_reg.clone()))

      /* Write r5 array. */
      + cfg.flow(
        Asm::str(
          elem_reg.clone(),
          (
            dst.clone(),
            ARM_DSIZE_WORD + (i as i32) * elem_size,
          ),
        )
        .size(elem_size.into()),
      );
    }

    flow
  } else {
    /* Malloc space for array. */
    generate_malloc(ARM_DSIZE_WORD, cfg, dst.clone().into())
  })

  /* Write length to first byte.
  LDR r5, =3
  STR r5, [r4] */
  + cfg.flow(Asm::ldr(elem_reg.clone(), exprs.len() as i32))
  + cfg.flow(Asm::str(elem_reg, (dst, 0)))
}

fn generate_pair_elem<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  elem: &PairElem,
  arg: ExprArg,
) -> Flow<'cfg> {
  /*  */
  let (t, pair, offset) = match elem {
    PairElem::Fst(TypedExpr(t, pair)) => (t, pair, 0),
    PairElem::Snd(TypedExpr(t, pair)) => (t, pair, ARM_DSIZE_WORD),
  };
  let reg = arg.reg();

  RequiredPredefs::CheckNullPointer.mark(cfg.code);

  /* Store address of pair in regs[0]. */
  pair.cfg_generate(scope, cfg, Dst(reg.clone()))

  /* CHECK: regs[0] != NULL */
  + cfg.flow(Asm::mov(ArgReg::R0, reg.clone()))
  + cfg.flow(Asm::b(PREDEF_CHECK_NULL_POINTER).link())

  /* Dereference. */
  + cfg.flow(Asm::ldr(reg.clone(), (reg.clone().into(), offset)))

  /* Dereference. */
  + {
    let instr = match arg {
      Src(_) => Asm::str(reg.clone(), (reg, 0)),
      Dst(_) => Asm::ldr(reg.clone(), (reg.into(), 0)),
    };

    cfg.flow(instr.size(t.size().into()))
  }
}

fn generate_struct_elem<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  elem: &StructElem,
  arg: ExprArg,
) -> Flow<'cfg> {
  let StructElem(struct_name, expr, field_name) = elem;
  let reg = arg.reg();

  /* Get struct definition. */
  let def = scope.get_def(struct_name).unwrap();

  /* Get offset and type. */
  let (type_, offset) = def.fields.get(field_name).unwrap();

  /* Evaluate expression. */
  expr.cfg_generate(scope, cfg, Dst(reg.clone()))

  /* Dereference with offset. */
  + {
    let instr = match arg {
      Src(_) => Asm::str(reg.clone(), (reg, *offset)),
      Dst(_) => Asm::ldr(reg.clone(), (reg.into(), *offset)),
    };

    cfg.flow(instr.size(type_.size().into()))
  }
}

fn generate_array_elem<'a, 'cfg>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  elem_type: &Type,
  arr_expr: &Expr,
  idx_expr: &Expr,
  arg: ExprArg,
) -> Flow<'cfg> {
  let elem_size = elem_type.size();
  let arr_ptr_reg = arg.reg();
  let idx_reg = cfg.get_veg();

  RequiredPredefs::ArrayBoundsError.mark(cfg.code);

  /* Evaluate array. */
  arr_expr.cfg_generate(scope, cfg, Dst(arr_ptr_reg.clone()))

  /* Evaluate index. */
  + idx_expr.cfg_generate(scope, cfg, Dst(idx_reg.clone()))

  /* Array bounds check. */
  + cfg // RO = index
    .flow(Asm::mov(ArgReg::R0, idx_reg.clone()))
  + cfg // R1 = array ptr
    .flow(Asm::mov(ArgReg::R1, arr_ptr_reg.clone()))
  + cfg.flow(Asm::b(PREDEF_CHECK_ARRAY_BOUNDS).link())

  /* Move pointer to array to correct element. */
  /* Move pointer over array length field. */
  + cfg.flow(Asm::add(arr_ptr_reg.clone(), arr_ptr_reg.clone(), ARM_DSIZE_WORD))

  /* Calculate how big each element is. */
  + {
    let shift = match elem_size {
      ARM_DSIZE_WORD => 2, /* Hardcoded log_2(current_type.size()) :) */
      ARM_DSIZE_BYTE => 0,
      /* Elements of sizes not equal to 4 or 1 not implemented. */
      _ => unimplemented!(),
    };

    /* Move pointer over elements. */
    cfg.flow(Asm::add(
      arr_ptr_reg.clone(),
      arr_ptr_reg.clone(),
      Op2::Reg(idx_reg.into(), -shift),
    ))
  }

  /* Either write to or read from that location. */
  + {
    let instr = match arg {
      Src(_) => Asm::str(arr_ptr_reg.clone(), (arr_ptr_reg, 0)),
      Dst(_) => Asm::ldr(arr_ptr_reg.clone(), (arr_ptr_reg.into(), 0)),
    };

    /*  */
    cfg.flow(instr.size(elem_size.into()))
  }
}

/* match src {
  Some(reg) => writes value at reg to this identifier,
  None => Evaluates this identifier into regs[0]
} */
fn generate_ident<'cfg, 'a>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  id: &Ident,
  arg: ExprArg,
) -> Flow<'cfg> {
  use IdentInfo::*;

  match scope.get(id) {
    Some(LocalVar(type_, var_reg)) => {
      let instr = match arg {
        /* STR {reg}, [sp, #{offset}] */
        Src(reg) => Asm::mov(var_reg, reg),
        /* LDR {regs[0]}, [sp, #{offset}] */
        Dst(reg) => Asm::mov(reg, var_reg),
      };

      cfg.flow(instr)

      // cfg.flow(instr.size(type_.size().into()))
    }
    Some(Label(_, label)) => {
      /* Cannot write to labels. */
      let reg = arg.dst();

      /* LDR {regs[0]}, ={label} */
      cfg.flow(Asm::ldr(reg, label))
    }
    _ => panic!("ident must be a local variable or function"),
  }
}

fn generate_char_liter<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  val: &char,
  dst: Reg,
) -> Flow<'cfg> {
  let ch = *val;
  let ch_op2 = if ch == '\0' {
    Op2::Imm(0)
  } else {
    Op2::Char(ch)
  };

  /* MOV r{min_reg}, #'val' */
  cfg.flow(Asm::mov(dst, ch_op2))
}

fn generate_string_liter<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  val: &str,
  dst: Reg,
) -> Flow<'cfg> {
  /* Create a label msg_{msg_no} to display the text */
  /* msg_{msg_no}: */
  let msg_label = cfg.code.get_msg(val);

  /* LDR r{min_reg}, ={msg_{msg_no}} */
  cfg.flow(Asm::ldr(dst, msg_label))
}

fn generate_unary_app<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  scope: &ScopeReader,
  op: &UnaryOper,
  expr: &Expr,
  dst: Reg,
) -> Flow<'cfg> {
  /* Stores expression's value in regs[0]. */
  expr.cfg_generate(scope, cfg, Dst(dst.clone()))

  /* Applies unary operator to regs[0]. */
  + generate_unary_op(cfg, dst, op)
}

fn generate_binary_app<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  scope: &ScopeReader,
  expr1: &Expr,
  op: &BinaryOper,
  expr2: &Expr,
  dst: Reg,
) -> Flow<'cfg> {
  let lhs_reg = cfg.get_veg();

  /* regs[0] = eval(expr1) */
  expr1.cfg_generate(scope, cfg, Dst(dst.clone()))

  /* Haven't run out of registers, evaluate normally. */
  + expr2.cfg_generate(scope, cfg, Dst(lhs_reg.clone()))

  /* regs[0] = regs[0] <op> regs[1] */
  + generate_binary_op(cfg, dst.clone(), dst, lhs_reg, op)
}

fn generate_unary_op<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  reg: Reg,
  unary_op: &UnaryOper,
) -> Flow<'cfg> {
  // TODO: Briefly explain the pre-condition that you created in the caller
  match unary_op {
    UnaryOper::Bang => generate_unary_bang(cfg, reg),
    UnaryOper::Neg => generate_unary_negation(cfg, reg),
    UnaryOper::Ord => cfg.dummy_flow(), //handled as char is already moved into reg in main match statement
    UnaryOper::Chr => cfg.dummy_flow(), //similar logic to above
    UnaryOper::Len => generate_unary_length(cfg, reg),
  }
}

fn generate_unary_bang<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  reg: Reg,
) -> Flow<'cfg> {
  /* EOR reg, reg, #1 */
  cfg.flow(Asm::eor(reg.clone(), reg, Op2::Imm(1)))
}

fn generate_unary_negation<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  reg: Reg,
) -> Flow<'cfg> {
  RequiredPredefs::OverflowError.mark(cfg.code);

  /* RSBS reg, reg, #0 */
  cfg.flow(Asm::rev_sub(reg.clone(), reg, Op2::Imm(0)).flags())

  /* BLVS p_throw_overflow_error */
    + cfg.flow(Asm::b(PREDEF_THROW_OVERFLOW_ERR.to_string()).link().vs())
}

fn generate_unary_length<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  reg: Reg,
) -> Flow<'cfg> {
  /* LDR reg, [reg]             //derefence value in reg */
  cfg.flow(Asm::ldr(reg.clone(), reg))
}

fn generate_binary_op<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  dst: Reg,
  reg1: Reg,
  reg2: Reg,
  bin_op: &BinaryOper,
) -> Flow<'cfg> {
  match bin_op {
    BinaryOper::Mul => {
      RequiredPredefs::OverflowError.mark(cfg.code);

      /* SMULL r4, r5, r4, r5 */
      cfg.flow(Asm::smull(reg1.clone(), reg2.clone(), reg1.clone(), reg2.clone()))

      /* CMP r5, r4, ASR #31 */
      + cfg.flow(Asm::cmp(reg2.clone(), Op2::Reg(reg1.clone().into(), 31)))

      /* BLNE p_throw_overflow_error */
      + cfg.flow(Asm::b(PREDEF_THROW_OVERFLOW_ERR).link().ne())
    }
    BinaryOper::Div => binary_div(cfg, reg1, reg2),
    BinaryOper::Mod => binary_mod(cfg, reg1, reg2),
    BinaryOper::Add => {
      //set overflow error branch to true
      RequiredPredefs::OverflowError.mark(cfg.code);

      /* ADDS r4, r4, r5 */
      cfg.flow(Asm::add(dst, reg1, Op2::Reg(reg2.into(), 0)).flags())

      /* BLVS p_throw_overflow_error */
      + cfg.flow(Asm::b(PREDEF_THROW_OVERFLOW_ERR).link().vs())
    }
    BinaryOper::Sub => {
      //set overflow error branch to true
      RequiredPredefs::OverflowError.mark(cfg.code);

      /* SUBS r4, r4, r5 */
      cfg.flow(Asm::sub(dst, reg1, Op2::Reg(reg2.into(), 0)).flags())

      /* BLVS p_throw_overflow_error */
      + cfg.flow(Asm::b(PREDEF_THROW_OVERFLOW_ERR).link().vs())
    }
    BinaryOper::Gt => binary_comp_ops(GT, LE, cfg, reg1.into(), reg2.into()),
    BinaryOper::Gte => binary_comp_ops(GE, LT, cfg, reg1.into(), reg2.into()),
    BinaryOper::Lt => binary_comp_ops(LT, GE, cfg, reg1.into(), reg2.into()),
    BinaryOper::Lte => binary_comp_ops(LE, GT, cfg, reg1.into(), reg2.into()),
    BinaryOper::Eq => binary_comp_ops(EQ, NE, cfg, reg1.into(), reg2.into()),
    BinaryOper::Neq => binary_comp_ops(NE, EQ, cfg, reg1.into(), reg2.into()),
    BinaryOper::And => {
      /* AND r4, r4, r5 */
      cfg.flow(Asm::and(dst, reg1, reg2))
    }
    BinaryOper::Or => {
      /* ORR r4, r4, r5 */
      cfg.flow(Asm::or(dst, reg1, reg2))
    }
  }
}

fn binary_div<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  reg1: Reg,
  reg2: Reg,
) -> Flow<'cfg> {
  RequiredPredefs::DivideByZeroError.mark(cfg.code);

  cfg.flow(Asm::mov(Reg::Arg(ArgReg::R0), reg1.clone()))
  /* MOV r1, reg2 */
  + cfg.flow(Asm::mov(Reg::Arg(ArgReg::R1), reg2))

  /* BL p_check_divide_by_zero */
  + cfg.flow(Asm::b(PREDEF_CHECK_DIVIDE_BY_ZERO).link())

  /* BL __aeabi_idiv */
  + cfg.flow(Asm::b(PREDEF_AEABI_IDIV).link())

  /* MOV reg1, r0 */
  + cfg.flow(Asm::mov(reg1, ArgReg::R0))
}

fn binary_mod<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  reg1: Reg,
  reg2: Reg,
) -> Flow<'cfg> {
  RequiredPredefs::DivideByZeroError.mark(cfg.code);

  /* MOV r0, reg1 */
  cfg.flow(Asm::mov(Reg::Arg(ArgReg::R0), reg1.clone()))
  /* MOV r1, reg2 */
  + cfg.flow(Asm::mov(Reg::Arg(ArgReg::R1), reg2))

  /* BL p_check_divide_by_zero */
  + cfg.flow(Asm::b(PREDEF_CHECK_DIVIDE_BY_ZERO).link())

  /* BL __aeabi_idivmod */
  + cfg.flow(Asm::b(PREDEF_AEABI_IDIVMOD).link())

  /* MOV reg1, r1 */
  + cfg.flow(Asm::mov(reg1, ArgReg::R1))
}

fn binary_comp_ops<'a, 'cfg>(
  cond1: CondCode,
  cond2: CondCode,
  cfg: &'a mut CFG<'cfg>,
  reg1: Reg,
  reg2: Reg,
) -> Flow<'cfg> {
  /* CMP r4, r5 */
  cfg.flow(Asm::cmp(reg1.clone(), Op2::Reg(reg2, 0)))

  /* MOV{cond1} reg1, #1 */
  + cfg.flow(Asm::mov(reg1.clone(), Op2::Imm(1)).cond(cond1))
  /* MOV{cond2} reg1, #0 */
  + cfg.flow(Asm::mov(reg1.clone(), Op2::Imm(0)).cond(cond2))
}
