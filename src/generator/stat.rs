use std::ops::Add;

use super::Generatable;
use crate::asm::*;
use crate::ast::*;

impl Generatable for AssignLhs {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for AssignRhs {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for PairElem {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for ArrayLiter {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for Stat {
  fn generate(&self, code: &mut Vec<Instr>, min_regs: &mut u8) {
    match self {
      Stat::Skip => (),
      Stat::Declaration(_, _, _) => todo!(),
      Stat::Assignment(_, _) => todo!(),
      Stat::Read(_) => todo!(),
      Stat::Free(_) => todo!(),
      Stat::Return(_) => todo!(),

      Stat::Exit(expr) => {
        match expr {
          Expr::IntLiter(exit_code) => {
            code.push(Instr::LoadImm(Reg::RegNum(*min_regs), *exit_code));
            code.push(Instr::Mov(
              Reg::RegNum(0),
              Op2::Reg(Reg::RegNum(*min_regs)),
              CondCode::AL,
            ));
            //*min_regs += 1; don't need to increment!
            code.push(Instr::Branch(String::from("exit"), CondCode::AL));
          }
          _ => unreachable!("Unreachable Syntax Error"),
        }
      }

      Stat::Print(_) => todo!(),
      Stat::Println(_) => todo!(),
      Stat::If(_, _, _) => todo!(),
      Stat::While(_, _) => todo!(),
      Stat::Scope(_) => todo!(),
      Stat::Sequence(_, _) => todo!(),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::f32::consts::E;

  use super::*;

  #[test]
  fn exit_statement() {
    let exit_code = 0;
    let stat = Stat::Exit(Expr::IntLiter(exit_code));
    let mut min_regs = 4;

    let actual_code = &mut vec![];
    stat.generate(actual_code, &mut min_regs);

    let expected_code = &mut vec![];
    //todo!(); anything here?
    expected_code.push(Instr::LoadImm(Reg::RegNum(4), 0)); //LDR r4, #0
    expected_code.push(Instr::Mov(
      Reg::RegNum(0),
      Op2::Reg(Reg::RegNum(4)),
      CondCode::AL,
    )); //MOV r0, r4
    expected_code.push(Instr::Branch(String::from("exit"), CondCode::AL)); //BL exit

    assert_eq!(min_regs, 4); //assert r4 isn't reserved
    assert_eq!(actual_code, expected_code);
  }

  #[test]
  fn if_statement() {
    let cond = Expr::BoolLiter(true); // true
    let true_body = Stat::Println(Expr::StrLiter(String::from("True Body"))); // println "True Body"
    let false_body = Stat::Println(Expr::StrLiter(String::from("False Body"))); // println "False Body"

    let if_statement = Stat::If(
      // if
      cond.clone(),                 // true
      Box::new(true_body.clone()),  // then println "True Body"
      Box::new(false_body.clone()), // else println "False Body"
    ); // fi

    let mut min_regs = 4;

    let actual_code = &mut vec![];
    if_statement.generate(actual_code, &mut min_regs);

    let expected_code = &mut vec![];
    cond.generate(expected_code, &mut min_regs);
    expected_code.push(Instr::Cmp(Reg::RegNum(4), Op2::Imm(0))); // CMP r4, #0
    expected_code.push(Instr::Branch(String::from("L0"), CondCode::EQ)); // BEQ L0
    true_body.generate(expected_code, &mut min_regs);
    expected_code.push(Instr::Branch(String::from("L1"), CondCode::AL)); // B L1
    expected_code.push(Instr::Label(String::from("L0"))); // LO:
    false_body.generate(expected_code, &mut min_regs);
    expected_code.push(Instr::Label(String::from("L1"))); // LO:

    assert_eq!(actual_code, expected_code);
  }
}
