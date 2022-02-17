use crate::asm::*;
use crate::ast::*;

mod expr;
mod program;
mod stat;

trait Generatable {
  fn generate(&self, code: &mut Vec<Instr>, registers: &[Reg]);
}

pub fn generate(ast: &Program) -> Vec<Instr> {
  let mut asm = vec![];
  let registers: Vec<Reg> = vec![];

  ast.generate(&mut asm, &registers);

  asm
}
