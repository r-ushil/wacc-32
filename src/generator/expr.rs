use self::CondCode::*;
use super::*;
use crate::generator::asm::*;

impl Generatable for Expr {
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, min_regs: &mut u8) {
    match self {
      Expr::IntLiter(val) => {
        /* LDR r{min_reg}, val */
        code.text.push(Asm::Instr(
          AL,
          Instr::Load(DataSize::Word, Reg::RegNum(*min_regs), LoadArg::Imm(*val)),
        ))
      }

      Expr::BoolLiter(val) => {
        //set imm to 1 or 0 depending on val
        let imm = if *val == true { 1 } else { 0 };

        /* MOV r{min_reg}, #imm */
        code.text.push(Asm::Instr(
          AL,
          Instr::Unary(
            UnaryInstr::Mov,
            Reg::RegNum(*min_regs),
            Op2::Imm(imm),
            false,
          ),
        ))
      }

      Expr::CharLiter(val) => {
        /* MOV r{min_reg}, #'val' */
        code.text.push(Asm::Instr(
          AL,
          Instr::Unary(
            UnaryInstr::Mov,
            Reg::RegNum(*min_regs),
            Op2::Char(*val),
            false,
          ),
        ))
      }

      Expr::StrLiter(val) => {
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
        code.text.push(Asm::Instr(
          AL,
          Instr::Load(
            DataSize::Word,
            Reg::RegNum(*min_regs),
            LoadArg::Label(format!("msg_{}", msg_no)),
          ),
        ))
      }

      // Expr::PairLiter => todo!(),
      // Expr::Ident(_) => todo!(),
      // Expr::ArrayElem(_) => todo!(),
      Expr::UnaryApp(op, exp) => {
        exp.generate(scope, code, min_regs);
        let reg = Reg::RegNum(*min_regs);
        unary_op_gen(op, code, reg);
      }
      Expr::BinaryApp(exp1, op, exp2) => {
        exp1.generate(scope, code, min_regs);

        let reg1 = Reg::RegNum(*min_regs);
        *min_regs = *min_regs + 1;
        exp2.generate(scope, code, &mut (*min_regs));
        let reg2 = Reg::RegNum(*min_regs);
        *min_regs = *min_regs - 1;

        binary_op_gen(op, code, reg1, reg2);
      }
      _ => code.text.push(Asm::Directive(Directive::Label(format!(
        "{:?}.generate(_, {:?})",
        self, min_regs
      )))),
    }
  }
}

fn unary_op_gen(unary_op: &UnaryOper, code: &mut GeneratedCode, reg: Reg) {
  match unary_op {
    UnaryOper::Bang => {
      /* EOR reg, reg, #1 */
      code.text.push(Asm::Instr(
        AL,
        Instr::Binary(
          BinaryInstr::Eor,
          reg.clone(),
          reg.clone(),
          Op2::Imm(1),
          false,
        ),
      ));
    }
    UnaryOper::Neg => {
      /* RSBS reg, reg, #0 */
      code.text.push(Asm::Instr(
        AL,
        Instr::Binary(
          BinaryInstr::RevSub,
          reg.clone(),
          reg.clone(),
          Op2::Imm(0),
          false,
        ),
      ));
    }
    UnaryOper::Len => {
      /* LDR r4, [sp, #4]
         LDR r4, [r4]

         // get array's stack offset, load into reg
         // get value at reg address (first index) for length

      */
      todo!();
    }
    UnaryOper::Ord => (), //handled as char is already moved into reg in main match statement
    UnaryOper::Chr => (), //similar logic to above
  }
}

fn binary_op_gen(bin_op: &BinaryOper, code: &mut GeneratedCode, reg1: Reg, reg2: Reg) {
  let dst = reg1.clone();
  match bin_op {
    BinaryOper::Mul => {
      /* SMULL r4, r5, r4, r5 */
      code.text.push(Asm::Instr(
        AL,
        Instr::Multiply(reg1.clone(), reg2.clone(), reg1.clone(), reg2.clone()),
      ));

      /* CMP r5, r4, ASR #31 */
      code.text.push(Asm::Instr(
        AL,
        Instr::Unary(
          UnaryInstr::Cmp,
          reg2.clone(),
          Op2::Reg(reg1.clone(), 31),
          false,
        ),
      ));
    }
    BinaryOper::Div => binary_div_mod(BinaryOper::Div, code, reg1, reg2),
    BinaryOper::Mod => binary_div_mod(BinaryOper::Mod, code, reg1, reg2),
    BinaryOper::Add => {
      /* ADDS r4, r4, r5 */
      code.text.push(Asm::Instr(
        AL,
        Instr::Binary(BinaryInstr::Add, dst, reg1, Op2::Reg(reg2, 0), true),
      ));
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
      code.text.push(Asm::Instr(
        AL,
        Instr::Binary(BinaryInstr::Sub, dst, reg1, Op2::Reg(reg2, 0), true),
      ));
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
      code.text.push(Asm::Instr(
        AL,
        Instr::Binary(BinaryInstr::And, dst, reg1, Op2::Reg(reg2, 0), true),
      ));
    }
    BinaryOper::Or => {
      /* ORR r4, r4, r5 */
      code.text.push(Asm::Instr(
        AL,
        Instr::Binary(BinaryInstr::Or, dst, reg1, Op2::Reg(reg2, 0), true),
      ));
    }
  }
}

fn binary_div_mod(op: BinaryOper, code: &mut GeneratedCode, reg1: Reg, reg2: Reg) {
  if op == BinaryOper::Div {
    /* MOV r0, reg1 */
    code.text.push(Asm::Instr(
      AL,
      Instr::Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Reg(reg1, 0), true),
    ));
    /* MOV r1, reg2 */
    code.text.push(Asm::Instr(
      AL,
      Instr::Unary(UnaryInstr::Mov, Reg::RegNum(1), Op2::Reg(reg2, 0), true),
    ));

    /* BL p_check_divide_by_zero */
    code.predefs.div_by_zero = true;
    code.text.push(Asm::Instr(
      AL,
      Instr::Branch(true, String::from("p_check_divide_by_zero")),
    ));

    /* BL __aeabi_idiv */
    code.text.push(Asm::Instr(
      AL,
      Instr::Branch(true, String::from("__aeabi_idiv")),
    ));
  } else if op == BinaryOper::Mod {
    /* MOV r0, reg1 */
    code.text.push(Asm::Instr(
      AL,
      Instr::Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Reg(reg1, 0), true),
    ));
    /* MOV r1, reg2 */
    code.text.push(Asm::Instr(
      AL,
      Instr::Unary(UnaryInstr::Mov, Reg::RegNum(1), Op2::Reg(reg2, 0), true),
    ));

    /* BL p_check_divide_by_zero */
    code.predefs.div_by_zero = true;
    code.text.push(Asm::Instr(
      AL,
      Instr::Branch(true, String::from("p_check_divide_by_zero")),
    ));

    /* BL __aeabi_idivmod */
    code.text.push(Asm::Instr(
      AL,
      Instr::Branch(true, String::from("__aeabi_idivmod")),
    ));
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
  code.text.push(Asm::Instr(
    AL,
    Instr::Unary(
      UnaryInstr::Cmp,
      reg1.clone(),
      Op2::Reg(reg2.clone(), 0),
      false,
    ),
  ));

  /* MOV{cond1} reg1, #1 */
  code.text.push(Asm::Instr(
    cond1,
    Instr::Unary(UnaryInstr::Mov, reg1.clone(), Op2::Imm(1), true),
  ));
  /* MOV{cond2} reg1, #0 */
  code.text.push(Asm::Instr(
    cond2,
    Instr::Unary(UnaryInstr::Mov, reg1.clone(), Op2::Imm(0), true),
  ));
}

impl Generatable for ArrayElem {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

#[cfg(test)]
mod tests {}
