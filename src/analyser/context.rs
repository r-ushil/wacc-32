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

type SymbolTable = HashMap<Ident, Type>;

#[derive(Debug)]
pub struct Scope<'a> {
  /* Maps identifiers to types for each variable declared in this scope. */
  symbol_table: SymbolTable,
  /* The scope this scope is inside of,
  and where abouts within that scope it is. */
  /* context: None means this is the global scope. */
  context: Option<(ContextLocation, &'a Scope<'a>)>,
}

#[allow(dead_code)]
impl<'a> Scope<'a> {
  /* Makes new Symbol table with initial global scope. */
  pub fn new() -> Self {
    Self {
      symbol_table: HashMap::new(),
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
  pub fn get(&self, ident: &Ident) -> Option<&Type> {
    match self.symbol_table.get(ident) {
      /* Identifier declared in this scope, return. */
      Some(t) => Some(t),
      /* Look for identifier in parent scope, recurse. */
      None => self.context?.1.get(ident),
    }
  }

  /* Sets type of ident to val, if ident already exists, updates it and
  returns old value. */
  pub fn insert(&mut self, ident: &Ident, val: Type) -> Option<()> {
    match self.symbol_table.insert(ident.clone(), val) {
      /* Val replaced something but we aren't allowed to change the type of
      variables, return None signifiying error. */
      Some(_) => None,
      /* No conflict, first time this identifier used in this scope, return
      unit signifiying success. */
      None => Some(()),
    }
  }

  pub fn new_scope(&'a self, location: ContextLocation) -> Self {
    Self {
      symbol_table: SymbolTable::new(),
      context: Some((location, self)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_scope<'a>() -> Scope<'a> {
    let mut scope = Scope::new();

    for i in 0..4 {
      let mut curr: HashMap<String, Type> = HashMap::new();

      let var1 = format!("{}{}", "x", i);
      let var2 = format!("{}{}", "y", i);
      let var3 = format!("{}{}", "z", i);

      scope.insert(&var1, Type::Bool);
      scope.insert(&var2, Type::Int);
      scope.insert(&var3, Type::String);

      scope.new_scope(ContextLocation::Function);
    }

    scope
  }

  #[test]
  fn test_table_lookup() {
    let scope = make_scope();

    assert_eq!(scope.get(&String::from("x3")), Some(&Type::Bool));
    assert_eq!(scope.get(&String::from("z3")), Some(&Type::String));
    assert_ne!(scope.get(&String::from("v3")), Some(&Type::String));

    assert_eq!(scope.get(&String::from("random")), None,);
  }

  #[test]
  fn test_table_update() {
    let mut scope = make_scope();

    assert_eq!(scope.insert(&String::from("g"), Type::Char), Some(()));

    assert_ne!(scope.get(&String::from("g")), Some(&Type::Bool));
  }
}
