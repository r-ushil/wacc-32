mod expr;
mod program;
mod stat;
mod symbol_table;
mod unify;

use std::fmt::Display;

use symbol_table::SymbolTable;
use unify::Unifiable;

use crate::ast::*;

/* Represents the result of a semantic analyse. */
#[derive(PartialEq, Eq, Debug)]
pub enum SemanticError {
  Normal(String),
  Syntax(String),
}

impl Display for SemanticError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SemanticError::Normal(s) | SemanticError::Syntax(s) => s.fmt(f),
    }
  }
}

type AResult<T> = Result<T, SemanticError>;

/* The semantic analyser has ONE jobs:
1. check the program is correctly typed */

/* The semantic analyser, like the parser, is made up of functions which type
check things, and when an AST represents a value, returns their type. */

/* ======== Helpers ======== */

/* If types are the same, return that type.
Otherwise, error. */
fn equal_types<L: HasType, R: HasType>(
  symbol_table: &SymbolTable,
  lhs: L,
  rhs: R,
) -> AResult<Type> {
  let lhs_type = lhs.get_type(symbol_table)?;
  let rhs_type = rhs.get_type(symbol_table)?;

  if let Some(t) = lhs_type.clone().unify(rhs_type.clone()) {
    Ok(t)
  } else {
    Err(SemanticError::Normal(format!(
      "TYPE ERROR: Type mismatch between.\n\tType 1: {:?}Type 2:\n\t{:?}",
      lhs_type, rhs_type
    )))
  }
}

/* Errors if AST node does not have expected type. */
fn expected_type<'a, A: HasType>(
  symbol_table: &SymbolTable,
  expected_type: &'a Type,
  actual: A,
) -> AResult<&'a Type> {
  let actual_type = actual.get_type(symbol_table)?;

  if expected_type.clone().unify(actual_type.clone()).is_some() {
    Ok(expected_type)
  } else {
    Err(SemanticError::Normal(format!(
      "TYPE ERROR: Unexpected type.\n\tExpected: {:?}\n\tActual: {:?}",
      expected_type, actual_type
    )))
  }
}

/* ======== Type Getters ======== */
/* These functions return the type of an AST node, and that they're well
 * typed. */

/* Represents AST nodes which have an associated type and allows you to
retrieve it without worrying what AST node it is. */
/* E.g: IntLiter(5).get_type(_) = Ok(BaseType(Int)) */
pub trait HasType {
  // TODO: make this return a reference to the type instead of a copy.
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type>;
}

impl<T: HasType> HasType for &T {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    (**self).get_type(symbol_table)
  }
}

impl<T: HasType> HasType for Box<T> {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    (**self).get_type(symbol_table)
  }
}

impl HasType for Ident {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    match symbol_table.get(self) {
      Some(t) => Ok(t.clone()),
      None => Err(SemanticError::Normal(format!(
        "Use of undeclared variable: {:?}",
        self
      ))),
    }
  }
}

impl HasType for ArrayElem {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    let ArrayElem(id, indexes) = self;

    /* Gets type of the array being looked up. */
    let mut curr_type = id.get_type(symbol_table)?;

    /* For each index, unwrap the type by one array. */
    /* x[1][2]: i32 where x: [[i32]] */
    for index in indexes {
      expected_type(symbol_table, &Type::Int, index)?;
      curr_type = match curr_type {
        Type::Array(t) => *t,
        t => {
          return Err(SemanticError::Normal(format!(
            "Expected array, found {:?}",
            t
          )))
        }
      };
    }

    Ok(curr_type)
  }
}

pub fn analyse(program: &Program) -> AResult<()> {
  let mut symbol_table = SymbolTable::new();
  program::program(&mut symbol_table, program)
}

/* ======== Type Checkers ======== */
/* These functions just checked that an AST is well typed. */

#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn idents() {
    let x = String::from("x");
    let x_type = Type::Int;
    let mut symbol_table = SymbolTable::new();

    /* x: BaseType(Int) */
    symbol_table.insert(&x, x_type.clone()).unwrap();

    assert_eq!(x.get_type(&symbol_table), Ok(x_type));
    assert!(String::from("hello").get_type(&symbol_table).is_err());
  }

  #[test]
  fn array_elems() {
    let id = String::from("x");

    let mut symbol_table = SymbolTable::new();

    /* x: Array(Array(Int)) */
    symbol_table.insert(&id, Type::Array(Box::new(Type::Array(Box::new(Type::Int)))));

    /* x[5][2]: Int */
    assert_eq!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5), Expr::IntLiter(2)]).get_type(&symbol_table),
      Ok(Type::Int),
    );

    /* x[5]['a'] is error */
    assert!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5), Expr::CharLiter('a')])
        .get_type(&symbol_table)
        .is_err()
    );

    /* x[5]: Array(Int) */
    assert_eq!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5)]).get_type(&symbol_table),
      Ok(Type::Array(Box::new(Type::Int))),
    );

    /* x[5][2][1] is error */
    assert!(ArrayElem(
      id.clone(),
      vec![Expr::IntLiter(5), Expr::IntLiter(2), Expr::IntLiter(1)]
    )
    .get_type(&symbol_table)
    .is_err());
  }
}
