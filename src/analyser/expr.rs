use super::{context::ScopeBuilder, equal_types, expected_type, HasType, SemanticError};
use crate::ast::*;

impl HasType for Expr {
  fn get_type(&mut self, scope: &ScopeBuilder, errors: &mut Vec<SemanticError>) -> Option<Type> {
    Some(match self {
      Expr::IntLiter(_) => Type::Int,
      Expr::BoolLiter(_) => Type::Bool,
      Expr::CharLiter(_) => Type::Char,
      Expr::StrLiter(_) => Type::String,
      Expr::PairLiter => Type::Pair(Box::new(Type::Any), Box::new(Type::Any)),
      Expr::Ident(id) => id.get_type(scope, errors)?,
      Expr::ArrayElem(elem) => elem.get_type(scope, errors)?,
      Expr::UnaryApp(op, exp) => match op {
        UnaryOper::Bang => expected_type(scope, errors, &Type::Bool, exp)?.clone(),
        UnaryOper::Neg => expected_type(scope, errors, &Type::Int, exp)?.clone(),
        UnaryOper::Len => match exp.get_type(scope, errors)? {
          Type::Array(_) => Type::Int,
          t => {
            scope.add_error(
              errors,
              SemanticError::Normal(format!(
              "TYPE ERROR: Attempt to find length of non array\n\tExpected: Array\n\tActual: {:?}",
              t
            )),
            );
            return None;
          }
        },
        UnaryOper::Ord => {
          expected_type(scope, errors, &Type::Char, exp)?;
          Type::Int
        }
        UnaryOper::Chr => {
          expected_type(scope, errors, &Type::Int, exp)?;
          Type::Char
        }
      },

      Expr::BinaryApp(exp1, op, exp2) => {
        /* Every binary application requires both expressions to have the same type. */
        let expr_type = equal_types(scope, errors, exp1, exp2)?;

        match op {
          /* Maths can only be done on ints. */
          BinaryOper::Mul
          | BinaryOper::Div
          | BinaryOper::Mod
          | BinaryOper::Add
          | BinaryOper::Sub => match expr_type {
            Type::Int => Type::Int,
            t => {
              scope.add_error(
                errors,
                SemanticError::Normal(format!(
                  "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
                  op, t
                )),
              );
              return None;
            }
          },
          /* Any types can be compared. */
          BinaryOper::Gt | BinaryOper::Gte | BinaryOper::Lt | BinaryOper::Lte => match expr_type {
            Type::Int | Type::Char => Type::Bool,
            t => {
              scope.add_error(
                errors,
                SemanticError::Normal(format!(
                  "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
                  op, t
                )),
              );
              return None;
            }
          },
          BinaryOper::Eq | BinaryOper::Neq => Type::Bool,
          /* Boolean operators can only be applied to booleans. */
          BinaryOper::And | BinaryOper::Or => match expr_type {
            Type::Bool => Type::Bool,
            t => {
              scope.add_error(
                errors,
                SemanticError::Normal(format!(
                  "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
                  op, t
                )),
              );
              return None;
            }
          },
        }
      }
    })
  }
}

#[cfg(test)]
mod tests {

  use crate::analyser::context::SymbolTable;

  use super::*;

  /* Defines a scope with 10 variables, each starting with prefix and ending
   * with 0..10 */
  fn populate_scope(scope: &mut ScopeBuilder, prefix: &str) {
    for i in 0..10 {
      let ident = format!("{}{}", prefix, i);
      let t = Type::Int;
      scope.insert(&ident, t);
    }
  }

  #[test]
  fn literals() {
    let mut symbol_table = SymbolTable::default();
    let scope = &ScopeBuilder::new(&mut symbol_table);

    assert_eq!(
      Expr::IntLiter(5).get_type(scope, &mut vec!()),
      Some(Type::Int)
    );
    assert_eq!(
      Expr::BoolLiter(false).get_type(scope, &mut vec!()),
      Some(Type::Bool)
    );
    assert_eq!(
      Expr::CharLiter('a').get_type(scope, &mut vec!()),
      Some(Type::Char)
    );
    assert_eq!(
      Expr::PairLiter.get_type(scope, &mut vec!()),
      Some(Type::Pair(Box::new(Type::Any), Box::new(Type::Any))),
    );
  }

  #[test]
  fn idents() {
    let mut symbol_table = SymbolTable::default();
    let mut scope = ScopeBuilder::new(&mut symbol_table);
    populate_scope(&mut scope, "var");

    assert_eq!(
      Expr::Ident(String::from("var1")).get_type(&scope, &mut vec![]),
      Some(Type::Int),
    );
  }

  #[test]
  fn array_elems() {
    let x = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));

    let mut symbol_table = SymbolTable::default();
    let mut scope = ScopeBuilder::new(&mut symbol_table);
    scope.insert(&x, x_type);

    assert_eq!(
      Expr::ArrayElem(ArrayElem(x, vec!(Expr::IntLiter(5)))).get_type(&scope, &mut vec![]),
      Some(Type::Int)
    );
  }

  #[test]
  fn unary_apps() {
    /* Symbol Table */
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    /* BANG */
    /* !false: Bool */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Bang, Box::new(Expr::BoolLiter(false)))
        .get_type(scope, &mut vec!()),
      Some(Type::Bool)
    );

    /* !'a': ERROR */
    assert!(
      Expr::UnaryApp(UnaryOper::Bang, Box::new(Expr::CharLiter('a')))
        .get_type(scope, &mut vec!())
        .is_none()
    );

    /* NEG */
    /* -5: Int */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Neg, Box::new(Expr::IntLiter(5))).get_type(scope, &mut vec!()),
      Some(Type::Int)
    );

    /* -false: ERROR */
    assert!(
      Expr::UnaryApp(UnaryOper::Neg, Box::new(Expr::BoolLiter(false)))
        .get_type(scope, &mut vec!())
        .is_none()
    );

    /* LEN */
    /* len [1,2,3]: Int */
    let x = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));
    scope.insert(&x, x_type);
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Len, Box::new(Expr::Ident(x))).get_type(scope, &mut vec!()),
      Some(Type::Int)
    );

    /* len 5: ERROR */
    assert!(Expr::UnaryApp(UnaryOper::Len, Box::new(Expr::IntLiter(5)))
      .get_type(scope, &mut vec!())
      .is_none());

    /* ORD */
    /* ord 'a': Int */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Ord, Box::new(Expr::CharLiter('a'))).get_type(scope, &mut vec!()),
      Some(Type::Int)
    );

    /* ord 5: ERROR */
    assert!(Expr::UnaryApp(UnaryOper::Ord, Box::new(Expr::IntLiter(5)))
      .get_type(scope, &mut vec!())
      .is_none());

    /* CHR */
    /* chr 5: Char */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Chr, Box::new(Expr::IntLiter(5))).get_type(scope, &mut vec!()),
      Some(Type::Char)
    );

    /* chr 'a': ERROR */
    assert!(
      Expr::UnaryApp(UnaryOper::Chr, Box::new(Expr::CharLiter('a')))
        .get_type(scope, &mut vec!())
        .is_none()
    );
  }

  #[test]
  fn binary_apps() {
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    /* 5 + false: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::IntLiter(5)),
      BinaryOper::Add,
      Box::new(Expr::BoolLiter(false))
    )
    .get_type(scope, &mut vec![])
    .is_none());

    /* 5 * 'a': ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::IntLiter(5)),
      BinaryOper::Mul,
      Box::new(Expr::CharLiter('a'))
    )
    .get_type(scope, &mut vec![])
    .is_none());

    /* false / "hello": ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::BoolLiter(false)),
      BinaryOper::Div,
      Box::new(Expr::StrLiter(String::from("hello")))
    )
    .get_type(scope, &mut vec![])
    .is_none());

    /* false && 6: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::BoolLiter(false)),
      BinaryOper::And,
      Box::new(Expr::IntLiter(6))
    )
    .get_type(scope, &mut vec![])
    .is_none());

    /* MATH CAN BE DONE ON INTS. */
    /* 5 * 5: Int */
    assert_eq!(
      Expr::BinaryApp(
        Box::new(Expr::IntLiter(5)),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(5)),
      )
      .get_type(scope, &mut vec![]),
      Some(Type::Int),
    );

    /* 5 + 5: Int */
    assert_eq!(
      Expr::BinaryApp(
        Box::new(Expr::IntLiter(5)),
        BinaryOper::Add,
        Box::new(Expr::IntLiter(5)),
      )
      .get_type(scope, &mut vec![]),
      Some(Type::Int),
    );

    /* MATH CANT BE DONE ON ANYTHING ELSE */
    /* 'a' + 'b': ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::CharLiter('a')),
      BinaryOper::Add,
      Box::new(Expr::CharLiter('b'))
    )
    .get_type(scope, &mut vec![])
    .is_none());

    /* false + false: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::BoolLiter(false)),
      BinaryOper::Add,
      Box::new(Expr::BoolLiter(false))
    )
    .get_type(scope, &mut vec![])
    .is_none());

    /* Any type can be comapred to itself. */
    for expr in vec![
      Expr::IntLiter(5),
      Expr::BoolLiter(false),
      Expr::StrLiter(String::from("hello")),
      Expr::PairLiter,
      Expr::UnaryApp(UnaryOper::Neg, Box::new(Expr::IntLiter(5))),
      Expr::BinaryApp(
        Box::new(Expr::StrLiter(String::from("hello world"))),
        BinaryOper::Eq,
        Box::new(Expr::StrLiter(String::from("hello steve"))),
      ),
    ] {
      for oper in vec![BinaryOper::Eq, BinaryOper::Neq] {
        assert_eq!(
          Expr::BinaryApp(Box::new(expr.clone()), oper, Box::new(expr.clone()))
            .get_type(scope, &mut vec![]),
          Some(Type::Bool)
        );
      }
    }

    for expr in vec![Expr::IntLiter(5), Expr::CharLiter('a')] {
      for oper in vec![
        BinaryOper::Gt,
        BinaryOper::Gte,
        BinaryOper::Lt,
        BinaryOper::Lte,
      ] {
        assert_eq!(
          Expr::BinaryApp(Box::new(expr.clone()), oper, Box::new(expr.clone()))
            .get_type(scope, &mut vec!()),
          Some(Type::Bool)
        );
      }
    }

    /* Boolean logic can only be applied to booleans */
    /* 5 && 5: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::IntLiter(5)),
      BinaryOper::And,
      Box::new(Expr::IntLiter(5)),
    )
    .get_type(scope, &mut vec!())
    .is_none());

    /* 'a' || 'a': ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::CharLiter('a')),
      BinaryOper::Or,
      Box::new(Expr::CharLiter('a')),
    )
    .get_type(scope, &mut vec!())
    .is_none());

    /* true && true: bool */
    assert_eq!(
      Expr::BinaryApp(
        Box::new(Expr::BoolLiter(true)),
        BinaryOper::And,
        Box::new(Expr::BoolLiter(true)),
      )
      .get_type(scope, &mut vec!()),
      Some(Type::Bool)
    );
  }
}
