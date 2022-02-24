use crate::ast::*;

mod asm;
mod display;
mod expr;
mod program;
mod stat;

use asm::*;

trait Generatable: std::fmt::Debug {
  fn generate(&self, code: &mut GeneratedCode, min_regs: &mut u8) {
    /* THIS DEFAULT IMPLEMENTATION IS JUST FOR TESTING PURPOSES */
    /* Because it's a default implementation, functionality not yet
    implemented will just return its inputs. */
    code.text.push(Asm::Directive(Directive::Label(format!(
      "{:?}.generate(_, {:?})",
      self, min_regs
    ))))
  }
}

pub fn generate(ast: &Program, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  let mut asm = GeneratedCode {
    data: vec![],
    text: vec![],
  };

  let mut min_regs = 4;

  ast.generate(&mut asm, &mut min_regs);

  write!(f, "{}", asm)
}
