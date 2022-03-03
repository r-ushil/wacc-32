use self::CondCode::*;
use super::*;
use crate::generator::asm::*;

impl Generatable for Expr {
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg]) {
    match self {
      Expr::IntLiter(val) => generate_int_liter(code, regs, val),
      Expr::BoolLiter(val) => generate_bool_liter(code, regs, val),
      Expr::CharLiter(val) => generate_char_liter(code, regs, val),
      Expr::StrLiter(val) => generate_string_liter(code, regs, val),
      Expr::UnaryApp(op, expr) => generate_unary_app(code, regs, scope, op, expr),
      Expr::BinaryApp(expr1, op, expr2) => generate_binary_app(code, regs, scope, expr1, op, expr2),
      // Expr::PairLiter => todo!(),
      Expr::Ident(id) => generate_ident(scope, code, regs, &id),
      // Expr::ArrayElem(_) => todo!(),
      _ => generate_temp_default(self, code, regs),
    }
  }
}

/* Stores value of local variable specified by ident to regs[0]. */
fn generate_ident(scope: &Scope, code: &mut GeneratedCode, regs: &[Reg], id: &Ident) {
  let offset = scope.get_offset(id).unwrap();

  code.text.push(Asm::always(Instr::Load(
    scope.get_type(id).unwrap().size().into(),
    regs[0],
    LoadArg::MemAddress(Reg::StackPointer, offset),
  )))
}

fn generate_int_liter(code: &mut GeneratedCode, regs: &[Reg], val: &i32) {
  /* LDR r{min_reg}, val */
  code.text.push(always_instruction(Instr::Load(
    DataSize::Word,
    regs[0],
    LoadArg::Imm(*val),
  )))
}

fn generate_bool_liter(code: &mut GeneratedCode, regs: &[Reg], val: &bool) {
  //set imm to 1 or 0 depending on val
  let imm = if *val == true { 1 } else { 0 };
  /* MOV r{min_reg}, #imm */
  code.text.push(always_instruction(Instr::Unary(
    UnaryInstr::Mov,
    regs[0],
    Op2::Imm(imm),
    false,
  )))
}

fn generate_char_liter(code: &mut GeneratedCode, regs: &[Reg], val: &char) {
  /* MOV r{min_reg}, #'val' */
  code.text.push(always_instruction(Instr::Unary(
    UnaryInstr::Mov,
    regs[0],
    Op2::Char(*val),
    false,
  )))
}

fn generate_string_liter(code: &mut GeneratedCode, regs: &[Reg], val: &String) {
  let count = val.chars().count();
  let msg_no = code.data.len();

  /* Create a label msg_{msg_no} to display the text */
  /* msg_{msg_no}: */
  code
    .data
    .push(Asm::Directive(Directive::Label(format!("msg_{}", msg_no))));
  /* .word {count}         //allocate space for a word of size count */
  code.data.push(Asm::Directive(Directive::Word(count)));
  /* .ascii "{val}"         //convert into ascii */
  code
    .data
    .push(Asm::Directive(Directive::Ascii(val.clone())));

  /* LDR r{min_reg}, ={msg_{msg_no}} */
  code.text.push(always_instruction(Instr::Load(
    DataSize::Word,
    regs[0],
    LoadArg::Label(format!("msg_{}", msg_no)),
  )))
}

fn generate_unary_app(
  code: &mut GeneratedCode,
  regs: &[Reg],
  scope: &Scope,
  op: &UnaryOper,
  expr: &Box<Expr>,
) {
  /* Stores expression's value in regs[0]. */
  expr.generate(scope, code, regs);

  /* Applies unary operator to regs[0]. */
  generate_unary_op(code, regs[0], op);
}

fn generate_binary_app(
  code: &mut GeneratedCode,
  regs: &[Reg],
  scope: &Scope,
  expr1: &Box<Expr>,
  op: &BinaryOper,
  expr2: &Box<Expr>,
) {
  /* regs[0] = eval(expr1) */
  expr1.generate(scope, code, regs);

  /* regs[1] = eval(expr2) */
  expr2.generate(scope, code, &regs[1..]);

  /* regs[0] = regs[0] <op> regs[1] */
  generate_binary_op(code, regs[0], regs[1], op);
}

fn always_instruction(instruction: Instr) -> Asm {
  Asm::Instr(AL, instruction)
}

fn generate_temp_default(expr: &Expr, code: &mut GeneratedCode, regs: &[Reg]) {
  code.text.push(Asm::Directive(Directive::Label(format!(
    "{:?}.generate(...)",
    expr
  ))))
}

fn generate_unary_op(code: &mut GeneratedCode, reg: Reg, unary_op: &UnaryOper) {
  // TODO: Briefly explain the pre-condition that you created in the caller
  match unary_op {
    UnaryOper::Bang => generate_unary_bang(code, reg, unary_op),
    UnaryOper::Neg => generate_unary_negation(code, reg, unary_op),
    // TODO: Further explanation in comment
    UnaryOper::Ord => (), //handled as char is already moved into reg in main match statement
    // TODO: Further explanation in comment.
    UnaryOper::Chr => (), //similar logic to above
    // TODO: implement this function.
    // UnaryOper::Len => generate_unary_length(code, reg, unary_op),
    _ => generate_unary_temp_default(code, reg, unary_op),
  }
}

fn generate_unary_bang(code: &mut GeneratedCode, reg: Reg, unary_op: &UnaryOper) {
  /* EOR reg, reg, #1 */
  code.text.push(always_instruction(Instr::Binary(
    BinaryInstr::Eor,
    reg.clone(),
    reg.clone(),
    Op2::Imm(1),
    false,
  )));
}

fn generate_unary_negation(code: &mut GeneratedCode, reg: Reg, unary_op: &UnaryOper) {
  /* RSBS reg, reg, #0 */
  code.text.push(always_instruction(Instr::Binary(
    BinaryInstr::RevSub,
    reg.clone(),
    reg.clone(),
    Op2::Imm(0),
    false,
  )));
}

//TODO: Implement this function
fn generate_unary_length(code: &mut GeneratedCode, reg: Reg, unary_op: &UnaryOper) {
  //   /* LDR r4, [sp, #4]
  //      LDR r4, [r4]

  //      // get array's stack offset, load into reg
  //      // get value at reg address (first index) for length

  //   */
  //   todo!();
}

fn generate_unary_temp_default(code: &mut GeneratedCode, reg: Reg, unary_op: &UnaryOper) {
  code.text.push(Asm::Directive(Directive::Label(format!(
    "{:?}.generate(...)",
    unary_op
  ))))
}

fn generate_binary_op(code: &mut GeneratedCode, reg1: Reg, reg2: Reg, bin_op: &BinaryOper) {
  // TODO: Briefly explain the pre-condition that you created in the caller
  let dst = reg1.clone();
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
    }
    BinaryOper::Div => binary_div_mod(BinaryOper::Div, code, reg1, reg2),
    BinaryOper::Mod => binary_div_mod(BinaryOper::Mod, code, reg1, reg2),
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
      code.predefs.overflow_err = true;
      /* BLVS p_throw_overflow_error */
      code.text.push(Asm::Instr(
        VS,
        Instr::Branch(true, String::from("p_throw_overflow_error")),
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
      code.predefs.overflow_err = true;
      /* BLVS p_throw_overflow_error */
      code.text.push(Asm::Instr(
        VS,
        Instr::Branch(true, String::from("p_throw_overflow_error")),
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
        true,
      )));
    }
    BinaryOper::Or => {
      /* ORR r4, r4, r5 */
      code.text.push(always_instruction(Instr::Binary(
        BinaryInstr::Or,
        dst,
        reg1,
        Op2::Reg(reg2, 0),
        true,
      )));
    }
  }
}

fn binary_div_mod(op: BinaryOper, code: &mut GeneratedCode, reg1: Reg, reg2: Reg) {
  if op == BinaryOper::Div {
    /* MOV r0, reg1 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      Reg::RegNum(0),
      Op2::Reg(reg1, 0),
      true,
    )));
    /* MOV r1, reg2 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      Reg::RegNum(1),
      Op2::Reg(reg2, 0),
      true,
    )));

    /* BL p_check_divide_by_zero */
    code.predefs.div_by_zero = true;
    code.text.push(always_instruction(Instr::Branch(
      true,
      String::from("p_check_divide_by_zero"),
    )));

    /* BL __aeabi_idiv */
    code.text.push(always_instruction(Instr::Branch(
      true,
      String::from("__aeabi_idiv"),
    )));
  } else if op == BinaryOper::Mod {
    /* MOV r0, reg1 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      Reg::RegNum(0),
      Op2::Reg(reg1, 0),
      true,
    )));
    /* MOV r1, reg2 */
    code.text.push(always_instruction(Instr::Unary(
      UnaryInstr::Mov,
      Reg::RegNum(1),
      Op2::Reg(reg2, 0),
      true,
    )));

    /* BL p_check_divide_by_zero */
    code.predefs.div_by_zero = true;
    code.text.push(always_instruction(Instr::Branch(
      true,
      String::from("p_check_divide_by_zero"),
    )));

    /* BL __aeabi_idivmod */
    code.text.push(always_instruction(Instr::Branch(
      true,
      String::from("__aeabi_idivmod"),
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
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

#[cfg(test)]
mod tests {}
