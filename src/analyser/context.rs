use std::collections::HashMap;

use super::SemanticError;
use crate::ast::*;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum ContextLocation {
  ProgramBody,
  Function,
  Scope,
  While,
  If,
}

pub type Offset = i32;
/* Associates each ident with an offset from the TOP of this stack frame,
also stores the total size of this stack frame. */
#[derive(PartialEq, Clone, Debug, Default)]
pub struct SymbolTable {
  pub table: HashMap<Ident, (Type, Offset)>,
  /* Sum of offsets in table */
  pub size: Offset,
}

#[derive(Debug)]
pub struct ScopeMut<'a> {
  /* Maps identifiers to types for each variable declared in this scope. */
  symbol_table: &'a mut SymbolTable,
  /* The scope this scope is inside of,
  and where abouts within that scope it is. */
  /* context: None means this is the global scope. */
  context: Option<(ContextLocation, &'a ScopeMut<'a>)>,
}

#[allow(dead_code)]
impl ScopeMut<'_> {
  /* Makes new Symbol table with initial global scope. */
  pub fn new<'a>(symbol_table: &'a mut SymbolTable) -> ScopeMut<'a> {
    ScopeMut {
      symbol_table,
      context: None,
    }
  }

  pub fn add_error(&self, errors: &mut Vec<SemanticError>, error: SemanticError) {
    if let Some((location, parent)) = self.context {
      /* Scope has parent, wrap error in nested. */
      parent.add_error(errors, SemanticError::Nested(location, Box::new(error)))
    } else {
      /* Global scope, no more nesting to do. */
      errors.push(error);
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

  /* Sets type of ident to val, if ident already exists, updates it and
  returns old value. */
  pub fn insert(&mut self, ident: &Ident, val: Type) -> Option<()> {
    /* Stackframe will be increased in size by val bytes */
    self.symbol_table.size += val.size();

    /* Offset of this variable from top of stack frame will be size
    of stack from. */
    let offset = self.symbol_table.size;

    match self.symbol_table.table.insert(ident.clone(), (val, offset)) {
      /* Val replaced something but we aren't allowed to change the type of
      variables, return None signifiying error. */
      Some(_) => None,
      /* No conflict, first time this identifier used in this scope, return
      unit signifiying success. */
      None => Some(()),
    }
  }

  pub fn new_scope<'a>(&'a self, symbol_table: &'a mut SymbolTable) -> ScopeMut<'a> {
    ScopeMut {
      symbol_table,
      context: Some((ContextLocation::Scope, self)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_scope<'a>(symbol_table: &'a mut SymbolTable) -> ScopeMut<'a> {
    let mut scope = ScopeMut::new(symbol_table);

    for i in 0..4 {
      let var1 = format!("{}{}", "x", i);
      let var2 = format!("{}{}", "y", i);
      let var3 = format!("{}{}", "z", i);

      scope.insert(&var1, Type::Bool);
      scope.insert(&var2, Type::Int);
      scope.insert(&var3, Type::String);
    }

    scope
  }

  #[test]
  fn test_table_lookup() {
    let mut symbol_table = SymbolTable::default();
    let scope = make_scope(&mut symbol_table);

    assert_eq!(scope.get_type(&String::from("x3")), Some(&Type::Bool));
    assert_eq!(scope.get_type(&String::from("z3")), Some(&Type::String));
    assert_ne!(scope.get_type(&String::from("v3")), Some(&Type::String));

    assert_eq!(scope.get_type(&String::from("random")), None,);
  }

  #[test]
  fn test_table_update() {
    let mut symbol_table = SymbolTable::default();
    let mut scope = make_scope(&mut symbol_table);

    assert_eq!(scope.insert(&String::from("g"), Type::Char), Some(()));

    assert_ne!(scope.get_type(&String::from("g")), Some(&Type::Bool));
  }
}
