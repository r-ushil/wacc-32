/* This is code gen's version of context.rs, cannot re-use context.rs
because that's mutable. */

use crate::analyser::context::{self, *};
use crate::ast::*;

pub use context::SymbolTable;

#[derive(Debug)]
pub struct Scope<'a> {
  /* Maps identifiers to types for each variable declared in this scope. */
  symbol_table: &'a SymbolTable,
  /* The scope this scope is inside of,
  and where abouts within that scope it is. */
  /* context: None means this is the global scope. */
  context: Option<(ContextLocation, &'a Scope<'a>)>,
}

#[allow(dead_code)]
impl Scope<'_> {
  /* Makes new Symbol table with initial global scope. */
  pub fn new<'a>(symbol_table: &'a SymbolTable) -> Scope<'a> {
    Scope {
      symbol_table,
      context: None,
    }
  }

  /* Returns type of given ident */
  pub fn get_type(&self, ident: &Ident) -> Option<&Type> {
    match self.symbol_table.table.get(ident) {
      /* Identifier declared in this scope, return. */
      Some((t, _)) => Some(t),
      /* Look for identifier in parent scope, recurse. */
      None => self.context?.1.get_type(ident),
    }
  }

  pub fn get_offset(&self, ident: &Ident) -> Option<Offset> {
    match self.symbol_table.table.get(ident) {
      /* Identifier declared in this scope, return. */
      Some((_, base_offset)) => Some(self.symbol_table.size - base_offset),
      /* Look for identifier in parent scope, recurse. */
      None => Some(self.context?.1.get_offset(ident)? + self.symbol_table.size),
    }
  }

  /* Same as get_type, but only checks the bottom most table. */
  pub fn get_bottom(&self, ident: &Ident) -> Option<&Type> {
    match self.context {
      Some((_, parent)) => parent.get_bottom(ident),
      None => Some(&self.symbol_table.table.get(ident)?.0),
    }
  }

  pub fn get_total_offset(&self) -> Offset {
    if self.symbol_table.table.is_empty() && self.symbol_table.size == 4 {
      /* When there are no symbols but the scope is 4 bytes long, we're at the
      scope used to reserve space for the lr register. */
      0
    } else {
      /* Otherwise, add the size of this scope and all the above scopes. */
      self.symbol_table.size + self.context.unwrap().1.get_total_offset()
    }
  }

  pub fn new_scope<'a>(&'a self, symbol_table: &'a SymbolTable) -> Scope<'a> {
    Scope {
      symbol_table,
      context: Some((ContextLocation::Scope, self)),
    }
  }
}
