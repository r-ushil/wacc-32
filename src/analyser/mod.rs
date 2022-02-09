mod expr;
mod program;
mod stat;
mod symbol_table;

use symbol_table::SymbolTable;

use crate::ast::*;

/* Represents the result of a semantic analyse. */
type SemanticError = String;
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

  if lhs_type != rhs_type {
    Err(format!(
      "TYPE ERROR: Type mismatch between.\n\tType 1: {:?}Type 2:\n\t{:?}",
      lhs_type, rhs_type
    ))
  } else {
    Ok(lhs_type)
  }
}

/* Errors if AST node does not have expected type. */
fn expected_type<'a, A: HasType>(
  symbol_table: &SymbolTable,
  expected_type: &'a Type,
  actual: A,
) -> AResult<&'a Type> {
  let actual_type = actual.get_type(symbol_table)?;

  if expected_type != &actual_type {
    Err(format!(
      "TYPE ERROR: Unexpected type.\n\tExpected: {:?}Actual:\n\t{:?}",
      expected_type, actual_type
    ))
  } else {
    Ok(expected_type)
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
      None => Err(format!("Use of undeclared variable: {:?}", self)),
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
      expected_type(symbol_table, &Type::BaseType(BaseType::Int), index)?;
      curr_type = match curr_type {
        Type::Array(t) => *t,
        t => return Err(format!("Expected array, found {:?}", t)),
      };
    }

    Ok(curr_type)
  }
}

/* ======== Type Checkers ======== */
/* These functions just checked that an AST is well typed. */

#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn idents() {
    let x = Ident(String::from("x"));
    let x_type = Type::BaseType(BaseType::Int);
    let mut symbol_table = SymbolTable::new();

    /* x: BaseType(Int) */
    symbol_table.insert(&x, x_type.clone());

    assert_eq!(x.get_type(&symbol_table), Ok(x_type));
    assert!(Ident(String::from("hello"))
      .get_type(&symbol_table)
      .is_err());
  }

  #[test]
  fn array_elems() {
    let id = Ident(String::from("x"));

    let mut symbol_table = SymbolTable::new();

    /* x: Array(Array(Int)) */
    symbol_table.insert(
      &id,
      Type::Array(Box::new(Type::Array(Box::new(Type::BaseType(
        BaseType::Int,
      ))))),
    );

    /* x[5][2]: Int */
    assert_eq!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5), Expr::IntLiter(2)]).get_type(&symbol_table),
      Ok(Type::BaseType(BaseType::Int)),
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
      Ok(Type::Array(Box::new(Type::BaseType(BaseType::Int)))),
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
