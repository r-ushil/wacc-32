use super::Generatable;
use crate::asm::*;
use crate::ast::*;

impl Generatable for Program {
  fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for Func {
  // fn generate(&self, code: &mut Vec<Instr>, registers: &[Reg]) {}
}

#[cfg(test)]
mod tests {}
