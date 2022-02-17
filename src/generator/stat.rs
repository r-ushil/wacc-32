use super::Generatable;
use crate::asm::*;
use crate::ast::*;

impl Generatable for AssignLhs {
  fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for AssignRhs {
  fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for PairElem {
  fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for ArrayLiter {
  fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for Stat {
  fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

#[cfg(test)]
mod tests {}
