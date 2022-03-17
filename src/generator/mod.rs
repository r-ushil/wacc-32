use crate::ast::*;

pub mod asm;
mod cfg;
mod display;
mod expr;
mod local_vec;
mod predef;
mod program;
mod scope;
mod stat;

use asm::*;
use cfg::*;
use scope::*;

pub const WACC_PROGRAM_MAIN_LABEL: &str = "main";

trait Generatable {
  type Input;
  type Output;

  fn generate(
    &self,
    _scope: &ScopeReader,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    aux: Self::Input,
  ) -> Self::Output;
}

trait CFGable {
  type Input;

  fn cfg_generate<'a, 'cfg>(
    &self,
    scope: &ScopeReader,
    cfg: &'a mut CFG<'cfg>,
    regs: &[GenReg],
    aux: Self::Input,
  ) -> Flow<'cfg>;
}

pub fn generate(ast: &Program) -> GeneratedCode {
  let mut asm = GeneratedCode::default();

  /* This symbol table will always be empty, but it means every AST node
  is generated with the same inputs. */
  let base_symbol_table = SymbolTable::default();
  let base_scope = ScopeReader::new(&base_symbol_table);

  /* Initally, all general purpose registers are free. */
  let regs = &GENERAL_REGS;

  ast.generate(&base_scope, &mut asm, regs, ());

  asm
}

fn generate_function_name(function_name: String) -> String {
  format!("f_{}", function_name)
}

fn generate_anon_func_name(func_name: String) -> String {
  format!("af_{}", func_name)
}
