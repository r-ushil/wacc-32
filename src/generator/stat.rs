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
  // fn generate(&self, code: &mut Vec<Instr>, registers: &[Reg]) {}
}

#[cfg(test)]
mod tests {
  use super::*;

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

    let registers = &vec![];

    let actual_code = &mut vec![];
    if_statement.generate(actual_code, registers);

    let expected_code = &mut vec![];
    cond.generate(expected_code, registers);
    expected_code.push(Instr::Cmp(Reg::RegNum(4), Op2::Imm(0))); // CMP r4, #0
    expected_code.push(Instr::Branch(String::from("L0"), CondCode::EQ)); // BEQ L0
    true_body.generate(expected_code, registers);
    expected_code.push(Instr::Branch(String::from("L1"), CondCode::AL)); // B L1
    expected_code.push(Instr::Label(String::from("L0"))); // LO:
    false_body.generate(expected_code, registers);
    expected_code.push(Instr::Label(String::from("L1"))); // LO:

    assert_eq!(actual_code, expected_code);
  }
}
