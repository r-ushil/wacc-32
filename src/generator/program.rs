use super::Generatable;
use crate::asm::*;
use crate::ast::*;

impl Generatable for Program {
  fn generate(&self, _code: &mut Vec<Instr>, min_regs: &mut i32) {}
}

impl Generatable for Func {
  // fn generate(&self, _code: &mut Vec<Instr>, min_regs: &mut i32) {}
}

#[cfg(test)]
mod tests {}
