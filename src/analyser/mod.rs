pub mod context;
mod expr;
mod program;
mod stat;
mod unify;

use std::fmt::Display;

use context::ScopeBuilder;
use unify::Unifiable;

use crate::ast::*;

use self::context::*;

/* Represents the result of a semantic analyse. */
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SemanticError {
  Normal(String),
  Syntax(String),
  Join(Box<SemanticError>, Box<SemanticError>),
}

// impl<I> From<I> for SemanticError
// where
//   I: Iterator<Item = SemanticError>,
// {
//   fn from(iter: I) -> Self {
//     let mut err = match iter.next() {
//       Some(e) => e,
//       None =>
//     };
//   }
// }

impl SemanticError {
  fn join_iter<T>(iter: impl Iterator<Item = AResult<T>>) -> AResult<()> {
    let mut result = Ok(());

    for i in iter {
      if let Err(e2) = i {
        result = match result {
          Ok(()) => Err(e2),
          Err(e1) => Err(SemanticError::Join(Box::new(e1), Box::new(e2))),
        }
      }
    }

    result
  }
}

/* Result of a semantic analysis. */
type AResult<T> = Result<T, SemanticError>;

/* Because AResult is a type alias, I cannot add methods to it directly,
so I add the join method via this trait, which is only implemented by AResult. */
trait Joinable {
  type T;

  fn join<U>(self, other: AResult<U>) -> AResult<(Self::T, U)>;
}

impl<T> Joinable for AResult<T> {
  type T = T;

  /* Used to join the analysis results, concatenating their errors.
  let a: AResult<T>;
  let b: AResult<U>;

  a.join(b) = {
    if (both ok) { Ok((a.unwrap(), b.unwrap())) }
    else { LinkedList containing all of both of their accumulated errors }
  }
  */
  fn join<U>(self, other: AResult<U>) -> AResult<(Self::T, U)> {
    match (self, other) {
      /* Both results failed, append the errors of the second to the first. */
      (Err(e1), Err(e2)) => {
        Err(SemanticError::Join(Box::new(e1), Box::new(e2)))
      }

      /* Only one failed, return their errors. */
      (Err(e), _) | (_, Err(e)) => Err(e),

      /* Both Ok, join the results into a pair. */
      (Ok(o1), Ok(o2)) => Ok((o1, o2)),
    }
  }
}

impl Display for SemanticError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SemanticError::Normal(s) | SemanticError::Syntax(s) => {
        write!(f, "ERROR: {}", s)
      }
      SemanticError::Join(e1, e2) => write!(f, "{}\n{}", e1, e2),
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
fn equal_types<
  L: Analysable<Input = (), Output = Type>,
  R: Analysable<Input = (), Output = Type>,
>(
  scope: &mut ScopeBuilder,
  lhs: &mut L,
  rhs: &mut R,
) -> AResult<Type> {
  let (lhs_type, rhs_type) =
    lhs.analyse(scope, ()).join(rhs.analyse(scope, ()))?;

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
fn expected_type<'a, A: Analysable<Input = (), Output = Type>>(
  scope: &mut ScopeBuilder,
  expected_type: &'a Type,
  actual: &mut A,
) -> AResult<&'a Type> {
  let actual_type = actual.analyse(scope, ())?;

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

/* ======== MAIN ANALYSABLE TRAIT ======= */
trait Analysable {
  type Input;
  type Output;

  fn analyse(
    &mut self,
    scope: &mut ScopeBuilder,
    aux: Self::Input,
  ) -> AResult<Self::Output>;
}

impl<T: Analysable<Output = Type>> Analysable for &mut T {
  type Input = <T as Analysable>::Input;
  type Output = Type;

  fn analyse(
    &mut self,
    scope: &mut ScopeBuilder,
    input: Self::Input,
  ) -> AResult<Type> {
    (**self).analyse(scope, input)
  }
}

impl<T: Analysable<Output = Type>> Analysable for Box<T> {
  type Input = <T as Analysable>::Input;
  type Output = Type;

  fn analyse(
    &mut self,
    scope: &mut ScopeBuilder,
    input: Self::Input,
  ) -> AResult<Type> {
    (**self).analyse(scope, input)
  }
}

impl Analysable for Ident {
  type Input = ();
  type Output = Type;

  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    use IdentInfo::*;

    match scope.get(self) {
      Some(LocalVar(t, _) | Label(t, _)) => Ok(t.clone()),
      _ => Err(SemanticError::Normal(format!(
        "Use of undeclared variable: {:#?}",
        self
      ))),
    }
  }
}

impl Analysable for ArrayElem {
  type Input = ();
  type Output = Type;
  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    let ArrayElem(id, indexes) = self;

    /* If any indexes aren't Int, return errors. */
    SemanticError::join_iter(
      indexes
        .iter_mut()
        .map(|index| expected_type(scope, &Type::Int, index)),
    )?;

    /* Gets type of the array being looked up. */
    let mut curr_type = id.analyse(scope, ())?;

    /* For each index, unwrap the type by one array. */
    for _ in indexes {
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

impl Analysable for StructElem {
  type Input = ();
  type Output = Type;
  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    let StructElem(struct_elem_id, expr, field_name) = self;

    /* Expression should have this type. */
    let expr_type = expr.analyse(scope, ())?;

    /* Get the struct's identifier. */
    let mut struct_id = match expr_type {
      Type::Custom(id) => id,
      _ => {
        return Err(SemanticError::Normal(format!(
          "Field lookup can only be done on structs."
        )))
      }
    };

    /* Put struct's identifier on StructElem because StructElem
    is generic over structs. */
    *struct_elem_id = struct_id.clone();

    /* Get struct definition. */
    let struct_def =
      scope
        .get_def(&mut struct_id)
        .ok_or(SemanticError::Normal(format!(
          "Custom type not found: {}",
          struct_id
        )))?;

    /* Look up field type. */
    let (field_type, _) =
      struct_def
        .fields
        .get(field_name)
        .ok_or(SemanticError::Normal(format!(
          "Struct {} has no field {}",
          struct_id, field_name
        )))?;

    /* This is the type of {struct_id}.{field_name}. */
    Ok(field_type.clone())
  }
}

pub fn analyse(program: &mut Program) -> AResult<()> {
  /* Makes fake ScopeBuilder so all analysis has the same signature. */
  program.analyse(&mut ScopeBuilder::new(&mut SymbolTable::default()), ())
}

/* ======== Type Checkers ======== */
/* These functions just checked that an AST is well typed. */

#[cfg(test)]

mod tests {
  use std::collections::HashMap;

  use crate::analyser::context::SymbolTable;

  use super::*;

  #[test]
  fn test_struct_elem() {
    let mut symbol_table = SymbolTable::default();
    let box_id = format!("box");
    let mut scope = ScopeBuilder::new(&mut symbol_table);
    /* Add custom struct to scope. */
    scope
      .insert(
        &format!("IntBox"),
        IdentInfo::TypeDef(Struct {
          fields: HashMap::from([(format!("y"), (Type::Bool, 0))]),
          size: 4,
        }),
      )
      .unwrap();

    /* Add variable to scope. */
    scope
      .insert_var(&mut box_id.clone(), Type::Custom(format!("IntBox")))
      .unwrap();

    let mut elem = StructElem(
      format!("IntBox"),
      Box::new(Expr::Ident(box_id)),
      format!("y"),
    );

    assert_eq!(elem.analyse(&mut scope, ()), Ok(Type::Bool));
  }

  #[test]
  fn test_array_elems() {
    let mut id = String::from("x");

    let mut symbol_table = SymbolTable::default();
    let mut scope = ScopeBuilder::new(&mut symbol_table);

    /* x: Array(Array(Int)) */
    scope
      .insert_var(
        &mut id,
        Type::Array(Box::new(Type::Array(Box::new(Type::Int)))),
      )
      .unwrap();

    /* x[5]['a'] is error */
    assert!(ArrayElem(
      id.clone(),
      vec![Expr::IntLiter(5), Expr::CharLiter('a')]
    )
    .analyse(&mut scope, ())
    .is_err());
  }

  #[test]
  fn idents() {
    let x = String::from("x");
    let x_type = Type::Int;
    let mut symbol_table = SymbolTable::default();
    let mut scope = ScopeBuilder::new(&mut symbol_table);

    /* x: BaseType(Int) */
    scope.insert_var(&mut x.clone(), x_type.clone()).unwrap();

    assert_eq!(x.clone().analyse(&mut scope, ()), Ok(x_type));
    assert!(String::from("hello").analyse(&mut scope, ()).is_err());
  }

  #[test]
  fn array_elems() {
    let id = String::from("x");

    let mut symbol_table = SymbolTable::default();
    let mut scope = ScopeBuilder::new(&mut symbol_table);

    /* x: Array(Array(Int)) */
    scope.insert_var(
      &mut id.clone(),
      Type::Array(Box::new(Type::Array(Box::new(Type::Int)))),
    );

    /* x[5][2]: Int */
    assert_eq!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5), Expr::IntLiter(2)])
        .analyse(&mut scope, ()),
      Ok(Type::Int),
    );

    /* x[5]['a'] is error */
    assert!(ArrayElem(
      id.clone(),
      vec![Expr::IntLiter(5), Expr::CharLiter('a')]
    )
    .analyse(&mut scope, ())
    .is_err());

    /* x[5]: Array(Int) */
    assert_eq!(
      ArrayElem(id.clone(), vec![Expr::IntLiter(5)]).analyse(&mut scope, ()),
      Ok(Type::Array(Box::new(Type::Int))),
    );

    /* x[5][2][1] is error */
    assert!(ArrayElem(
      id.clone(),
      vec![Expr::IntLiter(5), Expr::IntLiter(2), Expr::IntLiter(1)]
    )
    .analyse(&mut scope, ())
    .is_err());
  }
}
