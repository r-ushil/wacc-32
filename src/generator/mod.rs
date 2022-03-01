use crate::ast::*;

pub mod asm;
mod display;
mod expr;
mod predef;
mod program;
mod stat;

use asm::*;

trait Generatable: std::fmt::Debug {
  fn generate(&self, code: &mut GeneratedCode, min_reg: &mut u8) {
    /* THIS DEFAULT IMPLEMENTATION IS JUST FOR TESTING PURPOSES */
    /* Because it's a default implementation, functionality not yet
    implemented will just return its inputs. */
    code.text.push(Asm::Directive(Directive::Label(format!(
      "{:?}.generate(_, {:?})",
      self, min_reg
    ))))
  }
}

pub fn generate(ast: &Program) -> GeneratedCode {
  let mut asm = GeneratedCode::default();

  ast.generate(&mut asm, &mut 4);

  asm
}
