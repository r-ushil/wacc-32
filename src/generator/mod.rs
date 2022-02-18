use crate::asm::*;
use crate::ast::*;

mod expr;
mod program;
mod stat;

trait Generatable: std::fmt::Debug {
  fn generate(&self, code: &mut Vec<Instr>, registers: &[Reg]) {
    /* THIS DEFAULT IMPLEMENTATION IS JUST FOR TESTING PURPOSES */
    /* Because it's a default implementation, functionality not yet
    implemented will just return its inputs. */
    code.push(Instr::Label(format!(
      "{:?}.generate(_, {:?})",
      self, registers
    )))
  }
}

pub fn generate(ast: &Program) -> Vec<Instr> {
  let mut asm = vec![];
  let registers: Vec<Reg> = vec![];

  ast.generate(&mut asm, &registers);

  asm
}
