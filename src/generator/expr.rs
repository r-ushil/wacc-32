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
      Expr::BinaryApp(_, _, _) => todo!(),
    }
  }
}

impl Generatable for ArrayElem {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

#[cfg(test)]
mod tests {}
