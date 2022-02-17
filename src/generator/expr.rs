use super::Generatable;
use crate::asm::*;
use crate::ast::*;

impl Generatable for Expr {
  fn generate(&self, _code: &mut Vec<Asm>, _registers: &[Reg]) {}
}

#[cfg(test)]
mod tests {}
