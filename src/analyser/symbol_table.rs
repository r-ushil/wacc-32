use std::collections::HashMap;

use super::AResult;
use crate::ast::*;

type Scope = HashMap<Ident, Type>;

pub struct SymbolTable<'a> {
  current_scope: Scope,
  above_scopes: Option<&'a SymbolTable<'a>>,
}

#[allow(dead_code)]
impl<'a> SymbolTable<'a> {
  /* Makes new Symbol table with initial global scope. */
  pub fn new() -> Self {
    Self {
      current_scope: HashMap::new(),
      above_scopes: None,
    }
  }

  /* Returns type of given ident */
  pub fn get(&self, ident: &Ident) -> Option<&Type> {
    match self.current_scope.get(ident) {
      Some(type_) => Some(type_),
      None => match self.above_scopes {
        Some(symbol_table) => symbol_table.get(ident),
        None => None,
      },
    }
  }

  /* Sets type of ident to val, if ident already exists, updates it and
  returns old value. */
  pub fn insert(&mut self, ident: &Ident, val: Type) -> AResult<()> {
    match self.current_scope.insert(ident.clone(), val) {
      Some(_) => Err(format!(
        "Attempt to change type of variable in current scope."
      )),
      None => Ok(()),
    }
  }

  pub fn new_scope(&'a self) -> Self {
    Self {
      current_scope: HashMap::new(),
      above_scopes: Some(self),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn foo() {
    let mut a = SymbolTable::new();
    a.insert(&("foo".to_owned()), Type::Int);

    let mut b = a.new_scope();
    let b_ident = "bar".to_owned();
    let b_type = Type::Int;
    b.insert(&b_ident, b_type.clone());

    assert_eq!(b.get(&b_ident), Some(&b_type));

    a.insert(&("baz".to_owned()), Type::Int);
  }

  #[test]
  fn test_table_lookup() {
    // let mut st = SymbolTable::new();

    // for i in 0..4 {
    //   let mut curr: HashMap<String, Type> = HashMap::new();

    //   let var1 = Ident(format!("{}{}", "x", i));
    //   let var2 = Ident(format!("{}{}", "y", i));
    //   let var3 = Ident(format!("{}{}", "z", i));

    //   st.insert(&var1, Type::Bool);
    //   st.insert(&var2, Type::Int);
    //   st.insert(&var3, Type::String);

    //   st.new_scope();
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
    // let mut vec: SymbolTable = VecDeque::new();
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
