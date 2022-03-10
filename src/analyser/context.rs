use std::collections::HashMap;

use super::{AResult, SemanticError};
use crate::ast::*;

pub type Offset = i32;
/* Associates each ident with an offset from the TOP of this stack frame,
also stores the total size of this stack frame. */
#[derive(PartialEq, Clone, Debug, Default)]
pub struct SymbolTable {
  pub table: HashMap<Ident, (Type, Offset)>,
  /* Sum of offsets in table */
  pub size: Offset,
  /* How many symbol tables are above us. */
  pub prefix: String,
}

impl SymbolTable {
  /* Makes an empty symbol table with size = offset, this has the effect
  of recognise the stack pointer having moved down by {offset} bytes, because
  all calls to .get_offset will now be {offset} greater than they were. */
  pub fn empty(size: Offset) -> SymbolTable {
    SymbolTable {
      size,
      ..Default::default()
    }
  }
}

#[derive(Debug)]
pub struct ScopeBuilder<'a> {
  /* Maps identifiers to types for each variable declared in this scope. */
  current: &'a mut SymbolTable,
  /* The scope this scope is inside of,
  and where abouts within that scope it is. */
  /* context: None means this is the global scope. */
  parents: Option<&'a ScopeBuilder<'a>>,
  /* Pointer to type defs hashmap on root ast node. */
  type_defs: &'a TypeDefs,
}

#[allow(dead_code)]
impl ScopeBuilder<'_> {
  /* Makes new Symbol table with initial global scope. */
  pub fn new<'a>(symbol_table: &'a mut SymbolTable, type_defs: &'a TypeDefs) -> ScopeBuilder<'a> {
    /* This is base symbol table, depth = 0. */
    symbol_table.prefix = String::new();

    /*  */
    ScopeBuilder {
      current: symbol_table,
      parents: None,
      type_defs,
    }
  }

  /* Returns type of given ident */
  pub fn get_type(&self, ident: &Ident) -> Option<(&Type, Ident)> {
    match self.current.table.get(ident) {
      /* Identifier declared in this scope, return. */
      Some((t, offset)) => {
        let new_id = format!("{}{}", self.current.prefix, offset);
        Some((t, new_id))
      }
      /* Look for identifier in parent scope, recurse. */
      None => self.parents?.get_type(ident),
    }
  }

  pub fn get_offset(&self, ident: &Ident) -> Option<Offset> {
    match self.current.table.get(ident) {
      /* Identifier declared in this scope, return. */
      Some((_, base_offset)) => Some(self.current.size - base_offset),
      /* Look for identifier in parent scope, recurse. */
      None => Some(self.parents?.get_offset(ident)? + self.current.size),
    }
  }

  /* Retrieves a type's definition from it's identifier. */
  pub fn get_def(&self, ident: &Ident) -> Option<Struct> {
    Some(self.type_defs.get(ident)?.clone())
  }

  /* Sets type of ident to val, if ident already exists, updates it and
  returns old value. */
  pub fn insert(&mut self, ident: &Ident, val: Type) -> AResult<Ident> {
    /* Stackframe will be increased in size by val bytes */
    self.current.size += val.size();

    /* Offset of this variable from top of stack frame will be size
    of stack from. */
    let offset = self.current.size;

    match self.current.table.insert(ident.clone(), (val, offset)) {
      /* Val replaced something but we aren't allowed to change the type of
      variables, return None signifiying error. */
      Some(_) => Err(SemanticError::Normal(format!(
        "Attempt to redefine variable in current scope."
      ))),
      /* No conflict, first time this identifier used in this scope, return
      unit signifiying success. */
      None => Ok(format!("{}{}", self.current.prefix, offset)),
    }
  }

  pub fn new_scope<'a>(&'a self, symbol_table: &'a mut SymbolTable) -> ScopeBuilder<'a> {
    /* Every time we enter a new scope, add another _ to all the variable names. */
    symbol_table.prefix = format!("{}_", self.current.prefix);

    ScopeBuilder {
      current: symbol_table,
      parents: Some(self),
      type_defs: self.type_defs,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_scope<'a>(
    symbol_table: &'a mut SymbolTable,
    type_defs: &'a TypeDefs,
  ) -> ScopeBuilder<'a> {
    let mut scope = ScopeBuilder::new(symbol_table, &type_defs);

    for i in 0..4 {
      let var1 = format!("{}{}", "x", i);
      let var2 = format!("{}{}", "y", i);
      let var3 = format!("{}{}", "z", i);

      scope.insert(&var1, Type::Bool).unwrap();
      scope.insert(&var2, Type::Int).unwrap();
      scope.insert(&var3, Type::String).unwrap();
    }

    scope
  }

  #[test]
  fn test_table_lookup() {
    let mut symbol_table = SymbolTable::default();
    let type_defs = TypeDefs::default();
    let scope = make_scope(&mut symbol_table, &type_defs);

    assert!(matches!(
      scope.get_type(&String::from("x3")),
      Some((&Type::Bool, _))
    ));
    assert!(matches!(
      scope.get_type(&String::from("z3")),
      Some((&Type::String, _))
    ));
    assert!(!matches!(
      scope.get_type(&String::from("v3")),
      Some((&Type::String, _))
    ));

    assert_eq!(scope.get_type(&String::from("random")), None,);
  }

  #[test]
  fn test_table_update() {
    let mut symbol_table = SymbolTable::default();
    let type_defs = TypeDefs::default();
    let mut scope = make_scope(&mut symbol_table, &type_defs);

    assert!((scope.insert(&String::from("g"), Type::Char).is_ok()));

    assert!(!matches!(
      scope.get_type(&String::from("g")),
      Some((&Type::Bool, _))
    ));
  }
}
