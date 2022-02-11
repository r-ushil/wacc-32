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

type Errors = Vec<SemanticError>;

#[derive(Debug)]
pub enum Context<'a> {
  Global {
    symbol_table: SymbolTable,
  },
  Nested {
    symbol_table: SymbolTable,
    location: ContextLocation,
    above_context: &'a Context<'a>,
  },
}

#[allow(dead_code)]
impl<'a> Context<'a> {
  /* Makes new Symbol table with initial global scope. */
  pub fn new() -> Self {
    Self::Global {
      symbol_table: HashMap::new(),
    }
  }

  pub fn add_error(&self, errors: &mut Vec<SemanticError>, error: SemanticError) {
    match self {
      Self::Global { .. } => {
        errors.push(error);
      }
      Self::Nested {
        location,
        above_context,
        ..
      } => above_context.add_error(errors, SemanticError::Nested(*location, Box::new(error))),
    }
  }

  /* Returns type of given ident */
  pub fn get(&self, ident: &Ident) -> Option<&Type> {
    let symbol_table = match self {
      Self::Global { symbol_table, .. } | Self::Nested { symbol_table, .. } => symbol_table,
    };

    match symbol_table.get(ident) {
      Some(t) => Some(t),
      None => match self {
        Self::Global { .. } => None,
        Self::Nested { above_context, .. } => above_context.get(ident),
      },
    }
  }

  /* Sets type of ident to val, if ident already exists, updates it and
  returns old value. */
  pub fn insert(&mut self, ident: &Ident, val: Type) -> Option<()> {
    let symbol_table = match self {
      Self::Global { symbol_table, .. } | Self::Nested { symbol_table, .. } => symbol_table,
    };

    match symbol_table.insert(ident.clone(), val) {
      Some(_) => None,
      None => Some(()),
    }
  }

  pub fn new_context(&'a self, location: ContextLocation) -> Self {
    Self::Nested {
      symbol_table: HashMap::new(),
      location,
      above_context: self,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_context<'a>() -> Context<'a> {
    let mut context = Context::new();

    for i in 0..4 {
      let mut curr: HashMap<String, Type> = HashMap::new();

      let var1 = format!("{}{}", "x", i);
      let var2 = format!("{}{}", "y", i);
      let var3 = format!("{}{}", "z", i);

      context.insert(&var1, Type::Bool);
      context.insert(&var2, Type::Int);
      context.insert(&var3, Type::String);

      context.new_context(ContextLocation::Function);
    }

    context
  }

  #[test]
  fn test_table_lookup() {
    let context = make_context();

    assert_eq!(context.get(&String::from("x3")), Some(&Type::Bool));
    assert_eq!(context.get(&String::from("z3")), Some(&Type::String));
    assert_ne!(context.get(&String::from("v3")), Some(&Type::String));

    assert_eq!(context.get(&String::from("random")), None,);
  }

  #[test]
  fn test_table_update() {
    let mut context = make_context();

    assert_eq!(context.insert(&String::from("g"), Type::Char), Some(()));

    assert_ne!(context.get(&String::from("g")), Some(&Type::Bool));
  }
}
