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

  // #[test]
  // fn foo() {
  //   let mut a = Context::new();
  //   a.insert(&("foo".to_owned()), Type::Int);

  //   let mut b = a.new_context(String::from("Nested"));
  //   let b_ident = "bar".to_owned();
  //   let b_type = Type::Int;
  //   b.insert(&b_ident, b_type.clone());

  //   assert_eq!(b.get(&b_ident), Some(&b_type));

  //   a.insert(&("baz".to_owned()), Type::Int);
  // }

  #[test]
  fn test_table_lookup() {
    // let mut st = Context::new();

    // for i in 0..4 {
    //   let mut curr: HashMap<String, Type> = HashMap::new();

    //   let var1 = Ident(format!("{}{}", "x", i));
    //   let var2 = Ident(format!("{}{}", "y", i));
    //   let var3 = Ident(format!("{}{}", "z", i));

    //   st.insert(&var1, Type::Bool);
    //   st.insert(&var2, Type::Int);
    //   st.insert(&var3, Type::String);

    //   st.new_context();
    // }

    // assert_eq!(table_lookup(&vec, "x3"),
    // Ok(Type::Bool),);

    // assert_eq!(
    //   table_lookup(&vec, "z3"),
    //   Ok(Type::String),
    // );

    // assert_ne!(
    //   table_lookup(&vec, "v3"),
    //   Ok(Type::String),
    // );

    // assert_eq!(table_lookup(&vec, "random"), Err("not found".to_string()),);
  }

  #[test]
  fn test_table_update() {
    // let table: HashMap<String, Type> = HashMap::new();
    // let mut vec: Context = VecDeque::new();
    // vec.push_front(table);

    // for i in 0..4 {
    //   let mut curr: HashMap<String, Type> = HashMap::new();

    //   let var1 = format!("{}{}", "x", i);
    //   let var2 = format!("{}{}", "y", i);
    //   let var3 = format!("{}{}", "z", i);

    //   curr.insert(var1, Type::Bool);
    //   curr.insert(var2, Type::Int);
    //   curr.insert(var3, Type::String);

    //   vec.push_front(curr.clone());
    // }

    // // before tests = [x1: Bool, y1: Int, z1: String][x2: Bool, y2: Int, z2:
    // String] // [x3: Bool, y3: Int, z3: String]

    // assert_eq!(
    //   table_update(&mut vec, "x1", Type::Char, true),
    //   Ok(HashMap::from([
    //     ("x1".to_string(), Type::Char),
    //     ("y1".to_string(), Type::Int),
    //     ("z1".to_string(), Type::String),
    //   ]))
    // );

    // assert_ne!(table_lookup(&vec, "x1"), Ok(Type::Bool),)
  }
}
