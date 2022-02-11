mod context;
mod expr;
mod program;
mod stat;
mod unify;

use std::{collections::HashMap, fmt::Display};

use context::{Context, ContextLocation};
use unify::Unifiable;

use crate::ast::*;

/* Represents the result of a semantic analyse. */
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SemanticError {
  Normal(String),
  Syntax(String),
  Nested(ContextLocation, Box<SemanticError>),
}

impl Display for SemanticError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SemanticError::Normal(s) | SemanticError::Syntax(s) => s.fmt(f),
      SemanticError::Nested(location, err) => {
        write!(f, "{:?} in {:?}", err, location)
      }
    }
  }
}

/* The semantic analyser has ONE jobs:
1. check the program is correctly typed */

/* The semantic analyser, like the parser, is made up of functions which type
check things, and when an AST represents a value, returns their type. */

/* ======== Helpers ======== */

/* If types are the same, return that type.
Otherwise, error. */
fn equal_types<L: HasType, R: HasType>(
  context: &Context,
  errors: &mut Vec<SemanticError>,
  lhs: L,
  rhs: R,
) -> Option<Type> {
  if let (Some(lhs_type), Some(rhs_type)) =
    (lhs.get_type(context, errors), rhs.get_type(context, errors))
  {
    if let Some(t) = lhs_type.clone().unify(rhs_type.clone()) {
      Some(t)
    } else {
      context.add_error(
        errors,
        SemanticError::Normal(format!(
          "TYPE ERROR: Type mismatch between.\n\tType 1: {:?}Type 2:\n\t{:?}",
          lhs_type, rhs_type
        )),
      );

      None
    }
  } else {
    None
  }
}

/* Errors if AST node does not have expected type. */
fn expected_type<'a, A: HasType>(
  context: &Context,
  errors: &mut Vec<SemanticError>,
  expected_type: &'a Type,
  actual: A,
) -> Option<&'a Type> {
  let actual_type = actual.get_type(context, errors)?;

  if expected_type.clone().unify(actual_type.clone()).is_some() {
    Some(expected_type)
  } else {
    context.add_error(
      errors,
      SemanticError::Normal(format!(
        "TYPE ERROR: Unexpected type.\n\tExpected: {:?}\n\tActual: {:?}",
        expected_type, actual_type
      )),
    );

    None
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
  fn get_type(&self, context: &Context, errors: &mut Vec<SemanticError>) -> Option<Type>;
}

impl<T: HasType> HasType for &T {
  fn get_type(&self, context: &Context, errors: &mut Vec<SemanticError>) -> Option<Type> {
    (**self).get_type(context, errors)
  }
}

impl<T: HasType> HasType for Box<T> {
  fn get_type(&self, context: &Context, errors: &mut Vec<SemanticError>) -> Option<Type> {
    (**self).get_type(context, errors)
  }
}

impl HasType for Ident {
  fn get_type(&self, context: &Context, errors: &mut Vec<SemanticError>) -> Option<Type> {
    match context.get(self) {
      Some(t) => Some(t.clone()),
      None => {
        context.add_error(
          errors,
          SemanticError::Normal(format!("Use of undeclared variable: {:?}", self)),
        );
        None
      }
    }
  }
}

impl HasType for ArrayElem {
  fn get_type(&self, context: &Context, errors: &mut Vec<SemanticError>) -> Option<Type> {
    let ArrayElem(id, indexes) = self;
    let mut errored = false;

    /* Ensure all indexes are ints */
    for index in indexes {
      if index.get_type(context, errors) != Some(Type::Int) {
        errored = true;
      }
    }

    if errored {
      return None;
    }

    /* Gets type of the array being looked up. */
    let mut curr_type = id.get_type(context, errors)?;

    /* For each index, unwrap the type by one array. */
    for _ in indexes {
      curr_type = match curr_type {
        Type::Array(t) => *t,
        t => {
          context.add_error(
            errors,
            SemanticError::Normal(format!("Expected array, found {:?}", t)),
          );
          return None;
        }
      };
    }

    Some(curr_type)
  }
}

pub fn analyse(program: &Program) -> Result<(), Vec<SemanticError>> {
  let mut errors = Vec::new();
  let mut context = Context::new();

  if program::program(&mut context, &mut errors, program).is_some() {
    Ok(())
  } else {
    Err(errors)
  }
}

/* ======== Type Checkers ======== */
/* These functions just checked that an AST is well typed. */

#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn charlie_test() {
    let id = String::from("x");

    let mut context = Context::new();

    /* x: Array(Array(Int)) */
    context.insert(&id, Type::Array(Box::new(Type::Array(Box::new(Type::Int)))));

    /* x[5]['a'] is error */
    println!("{:?}", id.clone().get_type(&context, &mut vec!()));
    println!(
      "{:?}",
      ArrayElem(id.clone(), vec![Expr::IntLiter(5), Expr::CharLiter('a')])
        .get_type(&context, &mut vec![])
    );
    assert!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5), Expr::CharLiter('a')])
        .get_type(&context, &mut vec![])
        .is_none()
    );
  }

  #[test]
  fn idents() {
    let x = String::from("x");
    let x_type = Type::Int;
    let mut context = Context::new();

    /* x: BaseType(Int) */
    context.insert(&x, x_type.clone()).unwrap();

    assert_eq!(x.get_type(&context, &mut vec![]), Some(x_type));
    assert!(String::from("hello")
      .get_type(&context, &mut vec![])
      .is_none());
  }

  #[test]
  fn array_elems() {
    let id = String::from("x");

    let mut context = Context::new();

    /* x: Array(Array(Int)) */
    context.insert(&id, Type::Array(Box::new(Type::Array(Box::new(Type::Int)))));

    /* x[5][2]: Int */
    assert_eq!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5), Expr::IntLiter(2)])
        .get_type(&context, &mut vec![]),
      Some(Type::Int),
    );

    /* x[5]['a'] is error */
    assert!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5), Expr::CharLiter('a')])
        .get_type(&context, &mut vec![])
        .is_none()
    );

    /* x[5]: Array(Int) */
    assert_eq!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5)]).get_type(&context, &mut vec![]),
      Some(Type::Array(Box::new(Type::Int))),
    );

    /* x[5][2][1] is error */
    assert!(ArrayElem(
      id.clone(),
      vec![Expr::IntLiter(5), Expr::IntLiter(2), Expr::IntLiter(1)]
    )
    .get_type(&context, &mut vec![])
    .is_none());
  }
}
