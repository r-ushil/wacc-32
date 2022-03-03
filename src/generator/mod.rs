use crate::ast::*;

pub mod asm;
mod display;
mod expr;
mod predef;
mod program;
mod scope;
mod stat;

use asm::*;
use scope::*;

trait Generatable: std::fmt::Debug {
  fn generate(&self, _scope: &Scope, code: &mut GeneratedCode, regs: &[Reg]) {
    /* THIS DEFAULT IMPLEMENTATION IS JUST FOR TESTING PURPOSES */
    /* Because it's a default implementation, functionality not yet
    implemented will just return its inputs. */
    code.text.push(Asm::Directive(Directive::Label(format!(
      "{:?}.generate(...)",
      self
    ))))
  }
}

pub fn generate(ast: &Program) -> GeneratedCode {
  let mut asm = GeneratedCode::default();

  /* This symbol table will always be empty, but it means every AST node
  is generated with the same inputs. */
  let base_symbol_table = SymbolTable::default();
  let base_scope = Scope::new(&base_symbol_table);

  /* Initally, all general purpose registers are free. */
  let regs = &GENERAL_REGS;

  ast.generate(&base_scope, &mut asm, regs);

  asm
}
