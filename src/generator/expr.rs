use std::collections::HashSet;

use self::CondCode::*;
use super::predef::{
  RequiredPredefs, PREDEF_AEABI_IDIV, PREDEF_AEABI_IDIVMOD,
  PREDEF_CHECK_ARRAY_BOUNDS, PREDEF_CHECK_DIVIDE_BY_ZERO,
  PREDEF_THROW_OVERFLOW_ERR,
};
use super::*;
use crate::analyser::context::*;
use crate::generator::asm::*;
use crate::generator::program::LabelPrefix;
use crate::generator::stat::generate_malloc_with_reg;
use stat::generate_malloc;
use typed_arena::Arena;

impl Generatable for Expr {
  /* If specified, this specifies the source register
  which should be written to this expression. */
  type Input = Option<Reg>;

  type Output = ();

  fn generate(
    &self,
    scope: &ScopeReader,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    src: Option<Reg>,
  ) {
    let arena = Arena::new();
    let cfg = &mut CFG::new(code, &arena);

    match self {
      /* Identifiers, at this point only local variables and labels. */
      Expr::Ident(id) => generate_ident(scope, cfg, regs, id, src),
      /* Literal values. */
      Expr::IntLiter(val) => cfg.flow(Asm::ldr(Reg::General(regs[0]), *val)),
      Expr::BoolLiter(val) => cfg.flow(Asm::mov(
        Reg::General(regs[0]),
        Op2::Imm(if *val { 1 } else { 0 }),
      )),
      Expr::CharLiter(val) => generate_char_liter(cfg, regs, val),
      Expr::StrLiter(val) => generate_string_liter(cfg, regs, val),
      Expr::NullPairLiter => {
        cfg.flow(Asm::ldr(Reg::General(regs[0]), LoadArg::Imm(0)))
      }
      other => {
        match other {
          Expr::ArrayLiter(ArrayLiter(t, exprs)) => {
            generate_array_liter(scope, code, regs, t, exprs)
          }
          Expr::StructLiter(liter) => liter.generate(scope, code, regs, ()),
          Expr::UnaryApp(op, expr) => {
            generate_unary_app(code, regs, scope, op, expr)
          }
          Expr::BinaryApp(expr1, op, expr2) => {
            generate_binary_app(code, regs, scope, expr1, op, expr2)
          }
          Expr::PairLiter(e1, e2) => {
            generate_pair_liter(scope, code, regs, e1, e2)
          }
          Expr::ArrayElem(elem_type, arr_expr, idx_expr) => {
            generate_array_elem(
              scope, code, regs, elem_type, arr_expr, idx_expr, src,
            )
          }
          Expr::StructElem(elem) => {
            generate_struct_elem(scope, code, regs, elem, src)
          }
          Expr::PairElem(elem) => {
            generate_pair_elem(scope, code, regs, elem, src)
          }
          Expr::Call(func_type, ident, exprs) => {
            generate_call(scope, code, regs, func_type.clone(), ident, exprs)
          }
          Expr::AnonFunc(func) => {
            generate_anon_func(scope, code, regs, (**func).clone())
          }
          Expr::BlankArrayLiter(t, size) => {
            generate_blank_arr(scope, code, regs, t, size)
          }
          _ => panic!("expression not caught!"),
        }
        return;
      }
    };

    cfg.linearise();
  }
}

fn generate_blank_arr(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  t: &Type,
  size: &Box<Expr>,
) {
  /* LDR {regs[0]}, =type_size */
  code.text.push(Asm::ldr(Reg::General(regs[0]), t.size()));

  size.generate(scope, code, &regs[1..], None);

  /* Malloc space for array. */
  generate_malloc_with_reg(
    Reg::General(regs[0]),
    Reg::General(regs[1]),
    code,
    Reg::General(regs[0]),
  );

  /* Write length to first byte.
  LDR r5, =3
  STR r5, [r4] */
  size.generate(scope, code, &regs[1..], None);
  code
    .text
    .push(Asm::str(Reg::General(regs[1]), (Reg::General(regs[0]), 0)));
}

fn generate_anon_func(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  func: Func,
) {
  let uncond_label = code.get_label();
  let anon_label = code.get_label();

  code.text.push(Asm::b(uncond_label.clone()));

  (anon_label.clone(), func).generate(scope, code, regs, LabelPrefix::AnonFunc);

  code
    .text
    .push(Asm::Directive(Directive::Label(uncond_label)));
  code.text.push(Asm::ldr(
    Reg::General(regs[0]),
    generate_anon_func_name(anon_label),
  ));
}

fn generate_call(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  func_type: Type,
  func: &Expr,
  exprs: &[Expr],
) {
  /* Get arg types. */

  let arg_types = match func_type {
    Type::Func(sig) => sig.param_types,
    _ => unreachable!("Analyser guarentees this is a function."),
  };

  /* Save all registers we haven't been allowed to mangle. */
  /* Figure out which registers aren't safe to overwrite and therefore need
  saving. */
  let mut unsafe_regs_set = GENERAL_REGS.iter().collect::<HashSet<_>>();
  for reg in regs.iter() {
    unsafe_regs_set.remove(reg);
  }

  /* Must put in some deterministic order so registers are popped in the
  same order as they are pushed. */
  let unsafe_regs_vec = unsafe_regs_set.into_iter().collect::<Vec<_>>();

  /* Push all to stack. */
  /* TODO: Change Push instruction to do this with one instruction. */
  for reg in unsafe_regs_vec.iter() {
    code.text.push(Asm::push(Reg::General(*reg.clone())));
  }

  /* Now all registers are saved, we can use all registers! */
  let safe_regs = &GENERAL_REGS;

  let mut args_offset = 0;

  for (expr, arg_type) in exprs.iter().zip(arg_types).rev() {
    let symbol_table = SymbolTable {
      size: args_offset,
      ..Default::default()
    };

    let arg_offset_scope = scope.new_scope(&symbol_table);

    expr.generate(&arg_offset_scope, code, safe_regs, None);

    code.text.push(
      Asm::str(
        Reg::General(safe_regs[0]),
        (Reg::StackPointer, -arg_type.size()),
      )
      .size(arg_type.size().into())
      .pre_indexed(),
    );

    /* Make symbol table bigger. */
    args_offset += arg_type.size();
  }

  /* Generate function pointer. */
  func.generate(
    /* Offset all stack accesses by the size the args take up. */
    &scope.new_scope(&SymbolTable::empty(args_offset)),
    code,
    regs,
    None,
  );

  /* Jump to function pointer. */
  code.text.push(Asm::bx(Reg::General(regs[0])).link());

  /* Pop preserved register back from the stack. */
  /* TODO: Change Pop instruction to do this with one instruction. */
  for reg in unsafe_regs_vec.iter().rev() {
    code.text.push(Asm::pop(Reg::General(*reg.clone())));
  }

  /* Stack space was given to parameter to call function.
  We've finished calling so we can deallocate this space now. */
  code.text.append(&mut Op2::imm_unroll(
    |offset| Asm::add(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset)),
    args_offset,
  ));

  code.text.push(Asm::mov(
    Reg::General(regs[0]),
    Op2::Reg(Reg::Arg(ArgReg::R0), 0),
  ));
}

fn generate_pair_liter(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  TypedExpr(e1_type, e1): &TypedExpr,
  TypedExpr(e2_type, e2): &TypedExpr,
) {
  let e1_size = e1_type.size();
  let e2_size = e2_type.size();

  /* Malloc for the pair.
  regs[0] = malloc(8) */
  generate_malloc(8, code, Reg::General(regs[0]));

  /* Evaluate e1.
  regs[1] = eval(e1) */
  e1.generate(scope, code, &regs[1..], None);

  /* Malloc for e1.
  r0 = malloc(e1_size) */
  generate_malloc(e1_size, code, Reg::Arg(ArgReg::R0));

  /* Write e1 to malloced space. */
  code.text.push(
    Asm::str(Reg::General(regs[1]), (Reg::Arg(ArgReg::R0), 0))
      .size(e1_size.into()),
  );

  /* Write pointer to e1 to pair. */
  code
    .text
    .push(Asm::str(Reg::Arg(ArgReg::R0), (Reg::General(regs[0]), 0)));

  /* Evaluate e2.
  regs[1] = eval(e2) */
  e2.generate(scope, code, &regs[1..], None);

  /* Malloc for e2.
  r0 = malloc(e2_size) */
  generate_malloc(e2_size, code, Reg::Arg(ArgReg::R0));

  /* Write e2 to malloced space. */
  code.text.push(
    Asm::str(Reg::General(regs[1]), (Reg::Arg(ArgReg::R0), 0))
      .size(e2_size.into()),
  );

  /* Write pointer to e2 to pair. */
  code.text.push(Asm::str(
    Reg::Arg(ArgReg::R0),
    (Reg::General(regs[0]), ARM_DSIZE_WORD),
  ))
}

fn generate_array_liter(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  elem_type: &Type,
  exprs: &[Expr],
) {
  if exprs.len() > 0 {
    /* Calculate size of elements. */
    let elem_size = elem_type.size();

    /* Malloc space for array. */
    generate_malloc(
      ARM_DSIZE_WORD + elem_size * exprs.len() as i32,
      code,
      Reg::General(regs[0]),
    );

    /* Write each expression to the array. */
    for (i, expr) in exprs.iter().enumerate() {
      /* Evaluate expr to r5. */
      expr.generate(scope, code, &regs[1..], None);

      /* Write r5 array. */
      code.text.push(
        Asm::str(
          Reg::General(regs[1]),
          (
            Reg::General(regs[0]),
            ARM_DSIZE_WORD + (i as i32) * elem_size,
          ),
        )
        .size(elem_size.into()),
      );
    }
  } else {
    /* Malloc space for array. */
    generate_malloc(ARM_DSIZE_WORD, code, Reg::General(regs[0]));
  }

  /* Write length to first byte.
  LDR r5, =3
  STR r5, [r4] */
  code
    .text
    .push(Asm::ldr(Reg::General(regs[1]), exprs.len() as i32));
  code
    .text
    .push(Asm::str(Reg::General(regs[1]), (Reg::General(regs[0]), 0)));
}

fn generate_pair_elem(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  elem: &PairElem,
  src: Option<Reg>,
) {
  /* Puts element address in regs[0]. */
  let elem_size = elem.generate(scope, code, regs, ());

  /* Dereference. */
  let instr = match src {
    Some(reg) => Asm::str(reg, (Reg::General(regs[0]), 0)),
    None => Asm::ldr(Reg::General(regs[0]), (Reg::General(regs[0]), 0)),
  };

  code.text.push(instr.size(elem_size));
}

fn generate_struct_elem(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  elem: &StructElem,
  src: Option<Reg>,
) {
  let StructElem(struct_name, expr, field_name) = elem;

  /* Get struct definition. */
  let def = scope.get_def(struct_name).unwrap();

  /* Get offset and type. */
  let (type_, offset) = def.fields.get(field_name).unwrap();

  /* Evaluate expression. */
  expr.generate(scope, code, regs, None);

  /* Dereference with offset. */
  let instr = match src {
    Some(reg) => Asm::str(reg, (Reg::General(regs[0]), *offset)),
    None => Asm::ldr(Reg::General(regs[0]), (Reg::General(regs[0]), *offset)),
  };

  code.text.push(instr.size(type_.size().into()));
}

fn generate_array_elem(
  scope: &ScopeReader,
  code: &mut GeneratedCode,
  regs: &[GenReg],
  elem_type: &Type,
  arr_expr: &Expr,
  idx_expr: &Expr,
  src: Option<Reg>,
) {
  let elem_size = elem_type.size();
  let arr_ptr_reg = Reg::General(regs[0]);
  let idx_reg = Reg::General(regs[1]);

  /* Evaluate array. */
  arr_expr.generate(scope, code, regs, None);

  /* Evaluate index. */
  idx_expr.generate(scope, code, &regs[1..], None);

  /* Array bounds check. */
  code // RO = index
    .text
    .push(Asm::mov(Reg::Arg(ArgReg::R0), Op2::Reg(idx_reg, 0)));
  code // R1 = array ptr
    .text
    .push(Asm::mov(Reg::Arg(ArgReg::R1), Op2::Reg(arr_ptr_reg, 0)));
  code.text.push(Asm::b(PREDEF_CHECK_ARRAY_BOUNDS).link());
  RequiredPredefs::ArrayBoundsError.mark(code);

  /* Move pointer to array to correct element. */
  /* Move pointer over array length field. */
  code
    .text
    .push(Asm::add(arr_ptr_reg, arr_ptr_reg, Op2::Imm(ARM_DSIZE_WORD)));

  /* Calculate how big each element is. */
  let shift = match elem_size {
    ARM_DSIZE_WORD => 2, /* Hardcoded log_2(current_type.size()) :) */
    ARM_DSIZE_BYTE => 0,
    /* Elements of sizes not equal to 4 or 1 not implemented. */
    _ => unimplemented!(),
  };

  /* Move pointer over elements. */
  code.text.push(Asm::add(
    arr_ptr_reg,
    arr_ptr_reg,
    Op2::Reg(idx_reg, -shift),
  ));

  /* Either write to or read from that location. */
  let instr = match src {
    Some(reg) => Asm::str(reg, (Reg::General(regs[0]), 0)),
    None => Asm::ldr(Reg::General(regs[0]), (Reg::General(regs[0]), 0)),
  };

  /*  */
  code.text.push(instr.size(elem_size.into()));
}

/* match src {
  Some(reg) => writes value at reg to this identifier,
  None => Evaluates this identifier into regs[0]
} */
fn generate_ident<'cfg, 'a>(
  scope: &ScopeReader,
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  id: &Ident,
  src: Option<Reg>,
) -> Flow<'cfg> {
  use IdentInfo::*;

  match scope.get(id) {
    Some(LocalVar(type_, offset)) => {
      let instr = match src {
        /* STR {reg}, [sp, #{offset}] */
        Some(reg) => Asm::str(reg, (Reg::StackPointer, offset)),
        /* LDR {regs[0]}, [sp, #{offset}] */
        None => Asm::ldr(Reg::General(regs[0]), (Reg::StackPointer, offset)),
      };

      cfg.flow(instr.size(type_.size().into()))
    }
    Some(Label(_, label)) => {
      /* Cannot write to labels. */
      assert!(src.is_none());

      /* LDR {regs[0]}, ={label} */
      cfg.flow(Asm::ldr(Reg::General(regs[0]), label))
    }
    _ => panic!("ident must be a local variable or function"),
  }
}

fn generate_char_liter<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  val: &char,
) -> Flow<'cfg> {
  let ch = *val;
  let ch_op2 = if ch == '\0' {
    Op2::Imm(0)
  } else {
    Op2::Char(ch)
  };

  /* MOV r{min_reg}, #'val' */
  cfg.flow(Asm::mov(Reg::General(regs[0]), ch_op2))
}

fn generate_string_liter<'a, 'cfg>(
  cfg: &'a mut CFG<'cfg>,
  regs: &[GenReg],
  val: &str,
) -> Flow<'cfg> {
  /* Create a label msg_{msg_no} to display the text */
  /* msg_{msg_no}: */
  let msg_label = cfg.code.get_msg(val);

  /* LDR r{min_reg}, ={msg_{msg_no}} */
  cfg.flow(Asm::ldr(Reg::General(regs[0]), msg_label))
}

fn generate_unary_app(
  code: &mut GeneratedCode,
  regs: &[GenReg],
  scope: &ScopeReader,
  op: &UnaryOper,
  expr: &Expr,
) {
  /* Stores expression's value in regs[0]. */
  expr.generate(scope, code, regs, None);

  /* Applies unary operator to regs[0]. */
  generate_unary_op(code, Reg::General(regs[0]), op);
}

fn generate_binary_app(
  code: &mut GeneratedCode,
  regs: &[GenReg],
  scope: &ScopeReader,
  expr1: &Expr,
  op: &BinaryOper,
  expr2: &Expr,
) {
  assert!(regs.len() >= 2);

  /* regs[0] = eval(expr1) */
  expr1.generate(scope, code, regs, None);

  if regs.len() > MIN_STACK_MACHINE_REGS {
    /* Haven't run out of registers, evaluate normally. */
    expr2.generate(scope, code, &regs[1..], None);

    /* regs[0] = regs[0] <op> regs[1] */
    generate_binary_op(code, regs[0], regs[0], regs[1], op);
  } else {
    /* Save regs[0] so we can use it for evaluating LHS. */
    code.text.push(Asm::push(Reg::General(regs[0])));

    /* The PUSH instruction above decremented stack pointer,
    so we need to expand symbol table to reflect this. */
    let st = SymbolTable::empty(ARM_DSIZE_WORD);

    /* Evaluate LHS using all registers. */
    expr2.generate(&scope.new_scope(&st), code, regs, None);

    /* Restore RHS into next available register. */
    code.text.push(Asm::pop(Reg::General(regs[1])));

    /* regs[0] = regs[1] <op> regs[0] */
    generate_binary_op(code, regs[0], regs[1], regs[0], op);
  }
}

fn generate_unary_op(code: &mut GeneratedCode, reg: Reg, unary_op: &UnaryOper) {
  // TODO: Briefly explain the pre-condition that you created in the caller
  match unary_op {
    UnaryOper::Bang => generate_unary_bang(code, reg),
    UnaryOper::Neg => generate_unary_negation(code, reg),
    UnaryOper::Ord => (), //handled as char is already moved into reg in main match statement
    UnaryOper::Chr => (), //similar logic to above
    UnaryOper::Len => generate_unary_length(code, reg),
  }
}

fn generate_unary_bang(code: &mut GeneratedCode, reg: Reg) {
  /* EOR reg, reg, #1 */
  code.text.push(Asm::eor(reg, reg, Op2::Imm(1)));
}

fn generate_unary_negation(code: &mut GeneratedCode, reg: Reg) {
  /* RSBS reg, reg, #0 */
  code.text.push(Asm::rev_sub(reg, reg, Op2::Imm(0)).flags());

  /* BLVS p_throw_overflow_error */
  code
    .text
    .push(Asm::b(PREDEF_THROW_OVERFLOW_ERR.to_string()).link().vs());

  RequiredPredefs::OverflowError.mark(code);
}

fn generate_unary_length(code: &mut GeneratedCode, reg: Reg) {
  /* LDR reg, [reg]             //derefence value in reg */
  code.text.push(Asm::ldr(reg, reg));
}

fn generate_binary_op(
  code: &mut GeneratedCode,
  gen_dst: GenReg,
  gen_reg1: GenReg,
  gen_reg2: GenReg,
  bin_op: &BinaryOper,
) {
  let dst = Reg::General(gen_dst);
  let reg1 = Reg::General(gen_reg1);
  let reg2 = Reg::General(gen_reg2);

  match bin_op {
    BinaryOper::Mul => {
      /* SMULL r4, r5, r4, r5 */
      code.text.push(Asm::smull(reg1, reg2, reg1, reg2));

      /* CMP r5, r4, ASR #31 */
      code.text.push(Asm::cmp(reg2, Op2::Reg(reg1, 31)));

      /* BLNE p_throw_overflow_error */
      code
        .text
        .push(Asm::b(PREDEF_THROW_OVERFLOW_ERR).link().ne());
      RequiredPredefs::OverflowError.mark(code);
    }
    BinaryOper::Div => binary_div(code, gen_reg1, gen_reg2),
    BinaryOper::Mod => binary_mod(code, gen_reg1, gen_reg2),
    BinaryOper::Add => {
      /* ADDS r4, r4, r5 */
      code
        .text
        .push(Asm::add(dst, reg1, Op2::Reg(reg2, 0)).flags());

      //set overflow error branch to true
      RequiredPredefs::OverflowError.mark(code);

      /* BLVS p_throw_overflow_error */
      code
        .text
        .push(Asm::b(PREDEF_THROW_OVERFLOW_ERR).link().vs());
    }
    BinaryOper::Sub => {
      /* SUBS r4, r4, r5 */
      code
        .text
        .push(Asm::sub(dst, reg1, Op2::Reg(reg2, 0)).flags());

      //set overflow error branch to true
      RequiredPredefs::OverflowError.mark(code);

      /* BLVS p_throw_overflow_error */
      code
        .text
        .push(Asm::b(PREDEF_THROW_OVERFLOW_ERR).link().vs());
    }
    BinaryOper::Gt => binary_comp_ops(GT, LE, code, reg1, reg2),
    BinaryOper::Gte => binary_comp_ops(GE, LT, code, reg1, reg2),
    BinaryOper::Lt => binary_comp_ops(LT, GE, code, reg1, reg2),
    BinaryOper::Lte => binary_comp_ops(LE, GT, code, reg1, reg2),
    BinaryOper::Eq => binary_comp_ops(EQ, NE, code, reg1, reg2),
    BinaryOper::Neq => binary_comp_ops(NE, EQ, code, reg1, reg2),
    BinaryOper::And => {
      /* AND r4, r4, r5 */
      code.text.push(Asm::and(dst, reg1, Op2::Reg(reg2, 0)));
    }
    BinaryOper::Or => {
      /* ORR r4, r4, r5 */
      code.text.push(Asm::or(dst, reg1, Op2::Reg(reg2, 0)));
    }
  }
}

fn binary_div(code: &mut GeneratedCode, gen_reg1: GenReg, gen_reg2: GenReg) {
  let reg1 = Reg::General(gen_reg1);
  let reg2 = Reg::General(gen_reg2); /* MOV r0, reg1 */
  code
    .text
    .push(Asm::mov(Reg::Arg(ArgReg::R0), Op2::Reg(reg1, 0)));
  /* MOV r1, reg2 */
  code
    .text
    .push(Asm::mov(Reg::Arg(ArgReg::R1), Op2::Reg(reg2, 0)));

  /* BL p_check_divide_by_zero */
  RequiredPredefs::DivideByZeroError.mark(code);
  code.text.push(Asm::b(PREDEF_CHECK_DIVIDE_BY_ZERO).link());

  /* BL __aeabi_idiv */
  code.text.push(Asm::b(PREDEF_AEABI_IDIV).link());

  /* MOV reg1, r0 */
  code
    .text
    .push(Asm::mov(reg1, Op2::Reg(Reg::Arg(ArgReg::R0), 0)));
}

fn binary_mod(code: &mut GeneratedCode, gen_reg1: GenReg, gen_reg2: GenReg) {
  let reg1 = Reg::General(gen_reg1);
  let reg2 = Reg::General(gen_reg2);
  /* MOV r0, reg1 */
  code
    .text
    .push(Asm::mov(Reg::Arg(ArgReg::R0), Op2::Reg(reg1, 0)));
  /* MOV r1, reg2 */
  code
    .text
    .push(Asm::mov(Reg::Arg(ArgReg::R1), Op2::Reg(reg2, 0)));

  /* BL p_check_divide_by_zero */
  RequiredPredefs::DivideByZeroError.mark(code);
  code.text.push(Asm::b(PREDEF_CHECK_DIVIDE_BY_ZERO).link());

  /* BL __aeabi_idivmod */
  code.text.push(Asm::b(PREDEF_AEABI_IDIVMOD).link());

  /* MOV reg1, r1 */
  code
    .text
    .push(Asm::mov(reg1, Op2::Reg(Reg::Arg(ArgReg::R1), 0)));
}

fn binary_comp_ops(
  cond1: CondCode,
  cond2: CondCode,
  code: &mut GeneratedCode,
  reg1: Reg,
  reg2: Reg,
) {
  /* CMP r4, r5 */
  code.text.push(Asm::cmp(reg1, Op2::Reg(reg2, 0)));

  /* MOV{cond1} reg1, #1 */
  code.text.push(Asm::mov(reg1, Op2::Imm(1)).cond(cond1));
  /* MOV{cond2} reg1, #0 */
  code.text.push(Asm::mov(reg1, Op2::Imm(0)).cond(cond2));
}
