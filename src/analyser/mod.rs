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
  fn get_type(&self, _symbol_table: &SymbolTable) -> AResult<Type> {
    todo!();
  }
}

/* ======== Type Checkers ======== */
/* These functions just checked that an AST is well typed. */

#[cfg(test)]

mod tests {}
