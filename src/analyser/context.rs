use std::{cell::Cell, collections::HashMap};

use super::{AResult, SemanticError};
// pub use crate::generator::asm::Offset;
use crate::{
  ast::*,
  generator::asm::{Label, Reg, VegNum},
};

#[derive(PartialEq, Clone, Debug)]
pub enum IdentInfo {
  /* Ident is top level function, what is it's label? */
  Label(Type, Label),
  /* Ident is local var, reg is which register this local var is stored in. */
  LocalVar(Type, Reg),
  /* Ident is a struct definition. */
  TypeDef(Struct),
}

/* Associates each ident with an offset from the TOP of this stack frame,
also stores the total size of this stack frame. */
#[derive(PartialEq, Clone, Debug, Default)]
pub struct SymbolTable {
  /* The offsets stored in this hashmap are distances
  from THE TOP of the scope. (NOT FROM THE STACK POINTER) */
  pub table: HashMap<Ident, IdentInfo>,
  /* How many symbol tables are above us. */
  pub prefix: String,
}

impl SymbolTable {
  /* Makes an empty symbol table with size = offset, this has the effect
  of recognise the stack pointer having moved down by {offset} bytes, because
  all calls to .get_offset will now be {offset} greater than they were. */
  // pub fn empty(size: Reg) -> SymbolTable {
  //   SymbolTable {
  //     size,
  //     ..Default::default()
  //   }
  // }
}

#[derive(Debug)]
pub struct ScopeBuilder<'a> {
  /* Maps identifiers to types for each variable declared in this scope. */
  current: &'a mut SymbolTable,
  /* The scope this scope is inside of,
  and where abouts within that scope it is. */
  /* context: None means this is the global scope. */
  parents: Option<&'a ScopeBuilder<'a>>,
  /* Auto-increment for unique, internal use, idents. */
  uniques: u32,
  /* Reference to the vegs counter. */
  pub vegs: &'a Cell<VegNum>,
}

#[allow(dead_code)]
impl ScopeBuilder<'_> {
  /* Makes new Symbol table with initial global scope. */
  pub fn new<'a>(
    symbol_table: &'a mut SymbolTable,
    vegs: &'a Cell<VegNum>,
  ) -> ScopeBuilder<'a> {
    /* This is base symbol table, depth = 0. */
    symbol_table.prefix = String::new();

    ScopeBuilder {
      current: symbol_table,
      parents: None,
      uniques: 0,
      vegs,
    }
  }

  /* Get the information about this ident,
  renames it if it's a local variable (global rename). */
  /* The offsets returned are distances from THE BOTTOM
  of this scope. (THE STACK POINTER) */
  /* ONLY CALL THIS ONCE PER AST IDENT, OTHERWISE THE RENAME WILL HAPPEN TWICE. */
  pub fn get(&self, ident: &mut Ident) -> Option<IdentInfo> {
    use IdentInfo::*;

    match self.current.table.get(ident) {
      /* Identifier declared in this scope, return. */
      Some(info) => {
        if let LocalVar(type_, reg) = info {
          /* Local variables get renamed. */
          *ident = format!("{}{}", self.current.prefix, reg);

          Some(LocalVar(type_.clone(), reg.clone()))
        } else {
          Some(info.clone())
        }
      }
      /* Look for identifier in parent scope, recurse. */
      None => match self.parents?.get(ident)? {
        LocalVar(t, reg) => Some(LocalVar(t, reg)),
        info => Some(info),
      },
    }
  }

  fn get_veg(&mut self) -> Reg {
    self.vegs.set(self.vegs.get() + 1);
    Reg::Virtual(self.vegs.get())
  }

  pub fn get_var(&self, ident: &mut Ident) -> Option<(Type, Reg)> {
    match self.get(ident)? {
      IdentInfo::LocalVar(t, reg) => Some((t, reg)),
      _ => None,
    }
  }

  pub fn get_def(&self, ident: &mut Ident) -> Option<Struct> {
    match self.get(ident)? {
      IdentInfo::TypeDef(def) => Some(def),
      _ => None,
    }
  }

  pub fn get_label(&self, ident: &mut Ident) -> Option<(Type, Label)> {
    match self.get(ident)? {
      IdentInfo::Label(t, label) => Some((t, label)),
      _ => None,
    }
  }

  /* Returns a unique identifer. */
  pub fn get_unique(&mut self) -> Ident {
    /* Increments how many uniques there are. */
    self.uniques += 1;

    /* Makes a unique identifier based on this. */
    format!("@{}{}", self.current.prefix, self.uniques)
  }

  pub fn insert_var(&mut self, ident: &mut Ident, t: Type) -> AResult<()> {
    /* Local variables increase the size of this scope. */
    // self.current.size += t.size();

    /* Offset of this variable from top of stack frame will be size
    of stack from. */
    // let offset = self.current.size;
    let reg = self.get_veg();

    self.insert(ident, IdentInfo::LocalVar(t, reg))?;

    /* Local variables get renamed. */
    *ident = format!("{}{}", self.current.prefix, reg);

    Ok(())
  }

  /* Sets type of ident to val, if ident already exists, updates it and
  returns old value. */
  pub fn insert(&mut self, ident: &Ident, val: IdentInfo) -> AResult<()> {
    match self.current.table.insert(ident.clone(), val) {
      /* Val replaced something but we aren't allowed to change the type of
      variables, return None signifiying error. */
      Some(_) => Err(SemanticError::Normal(format!(
        "Attempt to redefine identifier in current scope."
      ))),
      /* No conflict, first time this identifier used in this scope, return
      unit signifiying success. */
      None => Ok(()),
    }
  }

  pub fn new_scope<'a>(
    &'a self,
    symbol_table: &'a mut SymbolTable,
  ) -> ScopeBuilder<'a> {
    /* Every time we enter a new scope, add another _ to all the variable names. */
    symbol_table.prefix = format!("{}_", self.current.prefix);

    ScopeBuilder {
      current: symbol_table,
      parents: Some(self),
      uniques: 0,
      vegs: self.vegs,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // fn make_scope<'a>(symbol_table: &'a mut SymbolTable) -> ScopeBuilder<'a> {
  //   let mut scope = ScopeBuilder::new(symbol_table, &Cell::new(0));

  //   for i in 0..4 {
  //     let mut var1 = format!("{}{}", "x", i);
  //     let mut var2 = format!("{}{}", "y", i);
  //     let mut var3 = format!("{}{}", "z", i);

  //     scope.insert_var(&mut var1, Type::Bool).unwrap();
  //     scope.insert_var(&mut var2, Type::Int).unwrap();
  //     scope.insert_var(&mut var3, Type::String).unwrap();
  //   }

  //   scope
  // }

  // #[test]
  // fn test_table_lookup() {
  //   let mut symbol_table = SymbolTable::default();
  //   let scope = make_scope(&mut symbol_table);

  //   assert!(matches!(
  //     scope.get(&mut "x3".to_string()),
  //     Some(IdentInfo::LocalVar(Type::Bool, _))
  //   ));
  //   assert!(matches!(
  //     scope.get(&mut String::from("z3")),
  //     Some(IdentInfo::LocalVar(Type::String, _))
  //   ));
  //   assert!(!matches!(
  //     scope.get(&mut String::from("v3")),
  //     Some(IdentInfo::LocalVar(Type::String, _))
  //   ));

  //   assert_eq!(scope.get(&mut String::from("random")), None);
  // }

  // #[test]
  // fn test_table_update() {
  //   let mut symbol_table = SymbolTable::default();
  //   let mut scope = make_scope(&mut symbol_table);

  //   assert!((scope.insert_var(&mut String::from("g"), Type::Char).is_ok()));

  //   assert!(!matches!(
  //     scope.get(&mut String::from("g")),
  //     Some(IdentInfo::LocalVar(Type::Bool, _))
  //   ));
  // }
}
