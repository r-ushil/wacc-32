use std::ops::Add;

use self::CondCode::*;
use super::*;
use crate::generator::asm::*;

impl Generatable for Expr {
  fn generate(&self, code: &mut GeneratedCode, min_regs: &mut u8) {
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

      Expr::PairLiter => todo!(),
      Expr::Ident(_) => todo!(),
      Expr::ArrayElem(_) => todo!(),
      Expr::UnaryApp(_, _) => todo!(),
      Expr::BinaryApp(exp1, op, exp2) => {
        exp1.generate(code, min_regs);

        let reg1 = Reg::RegNum(*min_regs);
        *min_regs = *min_regs + 1;
        exp2.generate(code, &mut (*min_regs));
        let reg2 = Reg::RegNum(*min_regs);
        *min_regs = *min_regs - 1;

        binary_op_gen(op, code, reg1, reg2);
      }
    }
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
      //todo!() unary-op-gen(UnaryOp::Cmp, code, reg1.clone(), Op2::Reg(reg2.clone, 31))
    }
    BinaryOper::Div => {
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
    }
    BinaryOper::Mod => todo!(),
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

fn binary_comp_ops(
  cond1: CondCode,
  cond2: CondCode,
  code: &mut GeneratedCode,
  reg1: Reg,
  reg2: Reg,
) {
  /* CMP r4, r5 */
  //todo!(); //unary-op-gen(UnaryOp::Cmp, code, reg1.clone(), Op2::Reg(reg2, 0))

  /* MOV{cond1} r{min_reg}, #1 */
  code.text.push(Asm::Instr(
    cond1,
    Instr::Unary(UnaryInstr::Mov, reg1.clone(), Op2::Imm(1), true),
  ));
  /* MOV{cond2} r{min_reg}, #0 */
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
