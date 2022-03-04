use self::CondCode::*;
use super::predef::{
  RequiredPredefs, PREDEF_AEABI_IDIV, PREDEF_AEABI_IDIVMOD, PREDEF_CHECK_ARRAY_BOUNDS,
  PREDEF_CHECK_DIVIDE_BY_ZERO, PREDEF_THROW_OVERFLOW_ERR,
};
use super::*;
use crate::generator::asm::*;

impl Generatable for Expr {
  type Input = ();
  type Output = ();
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[GenReg], _aux: ()) {
    match self {
      Expr::IntLiter(val) => generate_int_liter(code, regs, val),
      Expr::BoolLiter(val) => generate_bool_liter(code, regs, val),
      Expr::CharLiter(val) => generate_char_liter(code, regs, val),
      Expr::StrLiter(val) => generate_string_liter(code, regs, val),
      Expr::UnaryApp(op, expr) => generate_unary_app(code, regs, scope, op, expr),
      Expr::BinaryApp(expr1, op, expr2) => generate_binary_app(code, regs, scope, expr1, op, expr2),
      Expr::PairLiter => generate_pair_liter(code, regs),
      Expr::Ident(id) => generate_ident(scope, code, regs, &id),
      Expr::ArrayElem(elem) => generate_array_elem(scope, code, regs, elem),
    }
  }
}

fn generate_pair_liter(code: &mut GeneratedCode, regs: &[GenReg]) {
  /* LDR reg[0] =0 */
  code.text.push(Asm::always(Instr::Load(
    DataSize::Word,
    Reg::General(regs[0]),
    LoadArg::Imm(0),
  )));
}

fn generate_array_elem(scope: &Scope, code: &mut GeneratedCode, regs: &[GenReg], elem: &ArrayElem) {
  /* Get address of array elem and store in regs[0]. */
  let array_elem_size = elem.generate(scope, code, regs, ());

  /* Read from that address into regs[0]. */
  code.text.push(Asm::always(Instr::Load(
    array_elem_size.into(),
    Reg::General(regs[0]),
    LoadArg::MemAddress(Reg::General(regs[0]), 0),
  )));
}

/* Stores value of local variable specified by ident to regs[0]. */
fn generate_ident(scope: &Scope, code: &mut GeneratedCode, regs: &[GenReg], id: &Ident) {
  let offset = scope.get_offset(id).unwrap();

  code.text.push(Asm::always(Instr::Load(
    scope.get_type(id).unwrap().size().into(),
    Reg::General(regs[0]),
    LoadArg::MemAddress(Reg::StackPointer, offset),
  )))
}

fn generate_int_liter(code: &mut GeneratedCode, regs: &[GenReg], val: &i32) {
  /* LDR r{min_reg}, val */
  code.text.push(always_instruction(Instr::Load(
    DataSize::Word,
    Reg::General(regs[0]),
    LoadArg::Imm(*val),
  )))
}

fn generate_bool_liter(code: &mut GeneratedCode, regs: &[GenReg], val: &bool) {
  //set imm to 1 or 0 depending on val
  let imm = if *val == true { 1 } else { 0 };
  /* MOV r{min_reg}, #imm */
  code.text.push(always_instruction(Instr::Unary(
    UnaryInstr::Mov,
    Reg::General(regs[0]),
    Op2::Imm(imm),
    false,
  )))
}

fn generate_char_liter(code: &mut GeneratedCode, regs: &[GenReg], val: &char) {
  let ch = *val;
  let ch_op2 = if ch == '\0' {
    Op2::Imm(0)
  } else {
    Op2::Char(ch)
  };

  /* MOV r{min_reg}, #'val' */
  code.text.push(always_instruction(Instr::Unary(
    UnaryInstr::Mov,
    Reg::General(regs[0]),
    ch_op2,
    false,
  )))
}

fn generate_string_liter(code: &mut GeneratedCode, regs: &[GenReg], val: &String) {
  /* Create a label msg_{msg_no} to display the text */
  /* msg_{msg_no}: */
  let msg_label = code.get_msg(val);

  /* LDR r{min_reg}, ={msg_{msg_no}} */
  code.text.push(always_instruction(Instr::Load(
    DataSize::Word,
    Reg::General(regs[0]),
    LoadArg::Label(msg_label),
  )))
}

fn generate_unary_app(
  code: &mut GeneratedCode,
  regs: &[GenReg],
  scope: &Scope,
  op: &UnaryOper,
  expr: &Box<Expr>,
) {
  /* Stores expression's value in regs[0]. */
  expr.generate(scope, code, regs, ());

  /* Applies unary operator to regs[0]. */
  generate_unary_op(code, Reg::General(regs[0]), op);
}

fn generate_binary_app(
  code: &mut GeneratedCode,
  regs: &[GenReg],
  scope: &Scope,
  expr1: &Box<Expr>,
  op: &BinaryOper,
  expr2: &Box<Expr>,
) {
  assert!(regs.len() >= 2);

  /* regs[0] = eval(expr1) */
  expr1.generate(scope, code, regs, ());

  if regs.len() > MIN_STACK_MACHINE_REGS {
    /* Haven't run out of registers, evaluate normally. */
    expr2.generate(scope, code, &regs[1..], ());

    /* regs[0] = regs[0] <op> regs[1] */
    generate_binary_op(code, regs[0], regs[0], regs[1], op);
  } else {
    /* Save regs[0] so we can use it for evaluating LHS. */
    code
      .text
      .push(Asm::always(Instr::Push(Reg::General(regs[0]))));

    /* The PUSH instruction above decremented stack pointer,
    so we need to expand symbol table to reflect this. */
    let st = SymbolTable {
      size: 4,
      ..Default::default()
    };

    /* Evaluate LHS using all registers. */
    expr2.generate(&scope.new_scope(&st), code, regs, ());

    /* Restore RHS into next available register. */
    code
      .text
      .push(Asm::always(Instr::Pop(Reg::General(regs[1]))));

    /* regs[0] = regs[1] <op> regs[0] */
    generate_binary_op(code, regs[0], regs[1], regs[0], op);
  }
}

fn always_instruction(instruction: Instr) -> Asm {
  Asm::Instr(AL, instruction)
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
  code.text.push(always_instruction(Instr::Binary(
    BinaryInstr::Eor,
    reg.clone(),
    reg.clone(),
    Op2::Imm(1),
    false,
  )));
}

fn generate_unary_negation(code: &mut GeneratedCode, reg: Reg) {
  /* RSBS reg, reg, #0 */
  code.text.push(always_instruction(Instr::Binary(
    BinaryInstr::RevSub,
    reg.clone(),
    reg.clone(),
    Op2::Imm(0),
    true,
  )));

  /* BLVS p_throw_overflow_error */
  code.text.push(Asm::Instr(
    VS,
    Instr::Branch(true, PREDEF_THROW_OVERFLOW_ERR.to_string()),
  ));

  RequiredPredefs::OverflowError.mark(code);
}

fn generate_unary_length(code: &mut GeneratedCode, reg: Reg) {
  /* LDR reg, [reg]             //derefence value in reg */
  code.text.push(Asm::always(Instr::Load(
    DataSize::Word,
    reg,
    LoadArg::MemAddress(reg, 0),
  )));
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
      code.text.push(always_instruction(Instr::Multiply(
        reg1.clone(),
        reg2.clone(),
        reg1.clone(),
        reg2.clone(),
      )));

      /* CMP r5, r4, ASR #31 */
      code.text.push(always_instruction(Instr::Unary(
        UnaryInstr::Cmp,
        reg2.clone(),
        Op2::Reg(reg1.clone(), 31),
        false,
      )));

      /* BLNE p_throw_overflow_error */
      code.text.push(Asm::Instr(
        CondCode::NE,
        Instr::Branch(true, PREDEF_THROW_OVERFLOW_ERR.to_string()),
      ));
      RequiredPredefs::OverflowError.mark(code);
    }
    BinaryOper::Div => binary_div_mod(BinaryOper::Div, code, gen_reg1, gen_reg2),
    BinaryOper::Mod => binary_div_mod(BinaryOper::Mod, code, gen_reg1, gen_reg2),
    BinaryOper::Add => {
      /* ADDS r4, r4, r5 */
      code.text.push(always_instruction(Instr::Binary(
        BinaryInstr::Add,
        dst,
        reg1,
        Op2::Reg(reg2, 0),
        true,
      )));
      //set overflow error branch to true
      RequiredPredefs::OverflowError.mark(code);
      /* BLVS p_throw_overflow_error */
      code.text.push(Asm::Instr(
        VS,
        Instr::Branch(true, PREDEF_THROW_OVERFLOW_ERR.to_string()),
      ));
    }
    BinaryOper::Sub => {
      /* SUBS r4, r4, r5 */
      code.text.push(always_instruction(Instr::Binary(
        BinaryInstr::Sub,
        dst,
        reg1,
        Op2::Reg(reg2, 0),
        true,
      )));
      //set overflow error branch to true
      RequiredPredefs::OverflowError.mark(code);
      /* BLVS p_throw_overflow_error */
      code.text.push(Asm::Instr(
        VS,
        Instr::Branch(true, PREDEF_THROW_OVERFLOW_ERR.to_string()),
      ));
    }
    BinaryOper::Gt => binary_comp_ops(GT, LE, code, reg1, reg2),
    BinaryOper::Gte => binary_comp_ops(GE, LT, code, reg1, reg2),
    BinaryOper::Lt => binary_comp_ops(LT, GE, code, reg1, reg2),
    BinaryOper::Lte => binary_comp_ops(LE, GT, code, reg1, reg2),
    BinaryOper::Eq => binary_comp_ops(EQ, NE, code, reg1, reg2),
    BinaryOper::Neq => binary_comp_ops(NE, EQ, code, reg1, reg2),
    BinaryOper::And => {
      /* AND r4, r4, r5 */
      code.text.push(always_instruction(Instr::Binary(
        BinaryInstr::And,
        dst,
        reg1,
        Op2::Reg(reg2, 0),
        false,
      )));
    }
    BinaryOper::Or => {
      /* ORR r4, r4, r5 */
      code.text.push(always_instruction(Instr::Binary(
        BinaryInstr::Or,
        dst,
        reg1,
        Op2::Reg(reg2, 0),
        false,
      )));
    }
  }
}

fn binary_div_mod(op: BinaryOper, code: &mut GeneratedCode, gen_reg1: GenReg, gen_reg2: GenReg) {
  let reg1 = Reg::General(gen_reg1);
  let reg2 = Reg::General(gen_reg2);
  if op == BinaryOper::Div {
    /* MOV r0, reg1 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      Reg::Arg(ArgReg::R0),
      Op2::Reg(reg1, 0),
      false,
    )));
    /* MOV r1, reg2 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      Reg::Arg(ArgReg::R1),
      Op2::Reg(reg2, 0),
      false,
    )));

    /* BL p_check_divide_by_zero */
    RequiredPredefs::DivideByZeroError.mark(code);
    code.text.push(always_instruction(Instr::Branch(
      true,
      PREDEF_CHECK_DIVIDE_BY_ZERO.to_string(),
    )));

    /* BL __aeabi_idiv */
    code.text.push(always_instruction(Instr::Branch(
      true,
      PREDEF_AEABI_IDIV.to_string(),
    )));

    /* MOV reg1, r0 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      reg1,
      Op2::Reg(Reg::Arg(ArgReg::R0), 0),
      false,
    )));
  } else if op == BinaryOper::Mod {
    /* MOV r0, reg1 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      Reg::Arg(ArgReg::R0),
      Op2::Reg(reg1, 0),
      false,
    )));
    /* MOV r1, reg2 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      Reg::Arg(ArgReg::R1),
      Op2::Reg(reg2, 0),
      false,
    )));

    /* BL p_check_divide_by_zero */
    RequiredPredefs::DivideByZeroError.mark(code);
    code.text.push(always_instruction(Instr::Branch(
      true,
      PREDEF_CHECK_DIVIDE_BY_ZERO.to_string(),
    )));

    /* BL __aeabi_idivmod */
    code.text.push(always_instruction(Instr::Branch(
      true,
      PREDEF_AEABI_IDIVMOD.to_string(),
    )));

    /* MOV reg1, r1 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      reg1,
      Op2::Reg(Reg::Arg(ArgReg::R1), 0),
      false,
    )));
  } else {
    unreachable!("undefined!");
  }
}

fn binary_comp_ops(
  cond1: CondCode,
  cond2: CondCode,
  code: &mut GeneratedCode,
  reg1: Reg,
  reg2: Reg,
) {
  /* CMP r4, r5 */
  code.text.push(always_instruction(Instr::Unary(
    UnaryInstr::Cmp,
    reg1.clone(),
    Op2::Reg(reg2.clone(), 0),
    false,
  )));

  /* MOV{cond1} reg1, #1 */
  code.text.push(Asm::Instr(
    cond1,
    Instr::Unary(UnaryInstr::Mov, reg1.clone(), Op2::Imm(1), false),
  ));
  /* MOV{cond2} reg1, #0 */
  code.text.push(Asm::Instr(
    cond2,
    Instr::Unary(UnaryInstr::Mov, reg1.clone(), Op2::Imm(0), false),
  ));
}

impl Generatable for ArrayElem {
  type Input = ();
  type Output = DataSize;

  /* Stores the address of the element in regs[0],
  returns size of element. */
  fn generate(
    &self,
    scope: &Scope,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    _aux: (),
  ) -> DataSize {
    let ArrayElem(id, indexes) = self;
    let mut current_type = scope.get_type(id).unwrap();
    let array_ptr_reg = Reg::General(regs[0]);
    let index_regs = &regs[1..];

    /* Get reference to {id}.
    Put address of array in regs[0].
    ADD {regs[0]}, sp, #{offset} */
    let offset = scope.get_offset(id).unwrap();
    code.text.push(Asm::always(Instr::Binary(
      BinaryInstr::Add,
      array_ptr_reg,
      Reg::StackPointer,
      Op2::Imm(offset),
      false,
    )));

    /* For each index. */
    for index in indexes {
      /* Each index unwraps the type by one.
      Type::Array(t) => t */
      current_type = match current_type {
        Type::Array(t) => t,
        /* Semantic analysis ensures array lookups
        only happen on arrays. */
        _ => unreachable!(),
      };

      /* index_regs[0] = eval(index)
      LDR {index_regs[0]} {index}     //load index into first index reg */
      index.generate(scope, code, index_regs, ());

      /* Dereference. */
      /* LDR {array_ptr_reg} [{array_ptr_reg}] */
      code.text.push(Asm::always(Instr::Load(
        DataSize::Word,
        array_ptr_reg,
        LoadArg::MemAddress(array_ptr_reg, 0),
      )));

      /* Move index_reg into r0 */
      /* MOV r0, {index_reg[0]} */
      code.text.push(Asm::always(Instr::Unary(
        UnaryInstr::Mov,
        Reg::Arg(ArgReg::R0),
        Op2::Reg(Reg::General(index_regs[0]), 0),
        false,
      )));

      /* Move array_ptr_reg into r1 */
      /* MOV r1, {array_ptr_reg} */
      code.text.push(Asm::always(Instr::Unary(
        UnaryInstr::Mov,
        Reg::Arg(ArgReg::R1),
        Op2::Reg(array_ptr_reg, 0),
        false,
      )));

      /* Branch to check array bounds */
      /* BL p_check_array_bounds */
      code.text.push(Asm::always(Instr::Branch(
        true,
        PREDEF_CHECK_ARRAY_BOUNDS.to_string(),
      )));

      /* Move over size field.
      ADD {array_ptr_reg} {array_ptr_reg} #4 */
      code.text.push(Asm::always(Instr::Binary(
        BinaryInstr::Add,
        array_ptr_reg,
        array_ptr_reg,
        Op2::Imm(4),
        false,
      )));

      /* Move to correct element. */
      let shift = match current_type.size() {
        4 => 2, /* Hardcoded log_2(current_type.size()) :) */
        1 => 0,
        /* Elements of sizes not equal to 4 or 1 not implemented. */
        _ => unimplemented!(),
      };
      code.text.push(Asm::always(Instr::Binary(
        BinaryInstr::Add,
        array_ptr_reg,
        array_ptr_reg,
        Op2::Reg(Reg::General(index_regs[0]), -shift),
        false,
      )))
    }

    RequiredPredefs::ArrayBoundsError.mark(code);

    current_type.size().into()
  }
}
