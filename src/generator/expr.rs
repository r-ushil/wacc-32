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
  match bin_op {
    BinaryOper::Mul => todo!(),
    BinaryOper::Div => todo!(),
    BinaryOper::Mod => todo!(),
    BinaryOper::Add => {
      let dst = reg1.clone();
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
    BinaryOper::Sub => todo!(),
    BinaryOper::Gt => todo!(),
    BinaryOper::Gte => todo!(),
    BinaryOper::Lt => todo!(),
    BinaryOper::Lte => todo!(),
    BinaryOper::Eq => todo!(),
    BinaryOper::Neq => todo!(),
    BinaryOper::And => todo!(),
    BinaryOper::Or => todo!(),
  }
}

impl Generatable for ArrayElem {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

#[cfg(test)]
mod tests {}
