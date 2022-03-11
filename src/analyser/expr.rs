use super::*;
use crate::ast::*;

impl HasType for Expr {
  fn get_type(&mut self, scope: &ScopeBuilder) -> AResult<Type> {
    Ok(match self {
      Expr::IntLiter(_) => Type::Int,
      Expr::BoolLiter(_) => Type::Bool,
      Expr::CharLiter(_) => Type::Char,
      Expr::StrLiter(_) => Type::String,
      Expr::PairLiter => Type::Pair(Box::new(Type::Any), Box::new(Type::Any)),
      Expr::Ident(id) => id.get_type(scope)?,
      Expr::ArrayElem(elem) => elem.get_type(scope)?,
      Expr::StructElem(elem) => elem.get_type(scope)?,
      Expr::UnaryApp(op, exp) => match op {
        UnaryOper::Bang => expected_type(scope, &Type::Bool, exp)?.clone(),
        UnaryOper::Neg => expected_type(scope, &Type::Int, exp)?.clone(),
        UnaryOper::Len => match exp.get_type(scope)? {
          Type::Array(_) => Type::Int,
          t => {
            return Err(SemanticError::Normal(format!(
              "TYPE ERROR: Attempt to find length of non array\n\tExpected: Array\n\tActual: {:?}",
              t
            )))
          }
        },
        UnaryOper::Ord => {
          expected_type(scope, &Type::Char, exp)?;
          Type::Int
        }
        UnaryOper::Chr => {
          expected_type(scope, &Type::Int, exp)?;
          Type::Char
        }
      },

      Expr::BinaryApp(exp1, op, exp2) => {
        /* Every binary application requires both expressions to have the same type. */
        let expr_type = equal_types(scope, exp1, exp2)?;

        match op {
          /* Maths can only be done on ints. */
          BinaryOper::Mul
          | BinaryOper::Div
          | BinaryOper::Mod
          | BinaryOper::Add
          | BinaryOper::Sub => match expr_type {
            Type::Int => Type::Int,
            t => {
              return Err(SemanticError::Normal(format!(
                "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
                op, t
              )))
            }
          },
          /* Any types can be compared. */
          BinaryOper::Gt | BinaryOper::Gte | BinaryOper::Lt | BinaryOper::Lte => match expr_type {
            Type::Int | Type::Char => Type::Bool,
            t => {
              return Err(SemanticError::Normal(format!(
                "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
                op, t
              )))
            }
          },
          BinaryOper::Eq | BinaryOper::Neq => Type::Bool,
          /* Boolean operators can only be applied to booleans. */
          BinaryOper::And | BinaryOper::Or => match expr_type {
            Type::Bool => Type::Bool,
            t => {
              return Err(SemanticError::Normal(format!(
                "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
                op, t
              )))
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
    let type_defs = TypeDefs::default();
    let scope = &ScopeBuilder::new(&mut symbol_table, &type_defs);

    assert_eq!(Expr::IntLiter(5).get_type(scope), Ok(Type::Int));
    assert_eq!(Expr::BoolLiter(false).get_type(scope), Ok(Type::Bool));
    assert_eq!(Expr::CharLiter('a').get_type(scope), Ok(Type::Char));
    assert_eq!(
      Expr::PairLiter.get_type(scope),
      Ok(Type::Pair(Box::new(Type::Any), Box::new(Type::Any))),
    );
  }

  #[test]
  fn idents() {
    let mut symbol_table = SymbolTable::default();
    let type_defs = TypeDefs::default();
    let mut scope = ScopeBuilder::new(&mut symbol_table, &type_defs);
    populate_scope(&mut scope, "var");

    assert_eq!(
      Expr::Ident(String::from("var1")).get_type(&scope),
      Ok(Type::Int),
    );
  }

  #[test]
  fn array_elems() {
    let x = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));

    let mut symbol_table = SymbolTable::default();
    let type_defs = TypeDefs::default();
    let mut scope = ScopeBuilder::new(&mut symbol_table, &type_defs);
    scope.insert(&x, x_type);

    assert_eq!(
      Expr::ArrayElem(ArrayElem(x, vec!(Expr::IntLiter(5)))).get_type(&scope),
      Ok(Type::Int)
    );
  }

  #[test]
  fn unary_apps() {
    /* Symbol Table */
    let mut symbol_table = SymbolTable::default();
    let type_defs = TypeDefs::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table, &type_defs);

    /* BANG */
    /* !false: Bool */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Bang, Box::new(Expr::BoolLiter(false)))
        .get_type(scope),
      Ok(Type::Bool)
    );

    /* !'a': ERROR */
    assert!(
      Expr::UnaryApp(UnaryOper::Bang, Box::new(Expr::CharLiter('a')))
        .get_type(scope)
        .is_err()
    );

    /* NEG */
    /* -5: Int */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Neg, Box::new(Expr::IntLiter(5)))
        .get_type(scope),
      Ok(Type::Int)
    );

    /* -false: ERROR */
    assert!(
      Expr::UnaryApp(UnaryOper::Neg, Box::new(Expr::BoolLiter(false)))
        .get_type(scope)
        .is_err()
    );

    /* LEN */
    /* len [1,2,3]: Int */
    let x = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));
    scope.insert(&x, x_type);
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Len, Box::new(Expr::Ident(x))).get_type(scope),
      Ok(Type::Int)
    );

    /* len 5: ERROR */
    assert!(Expr::UnaryApp(UnaryOper::Len, Box::new(Expr::IntLiter(5)))
      .get_type(scope)
      .is_err());

    /* ORD */
    /* ord 'a': Int */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Ord, Box::new(Expr::CharLiter('a')))
        .get_type(scope),
      Ok(Type::Int)
    );

    /* ord 5: ERROR */
    assert!(Expr::UnaryApp(UnaryOper::Ord, Box::new(Expr::IntLiter(5)))
      .get_type(scope)
      .is_err());

    /* CHR */
    /* chr 5: Char */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Chr, Box::new(Expr::IntLiter(5)))
        .get_type(scope),
      Ok(Type::Char)
    );

    /* chr 'a': ERROR */
    assert!(
      Expr::UnaryApp(UnaryOper::Chr, Box::new(Expr::CharLiter('a')))
        .get_type(scope)
        .is_err()
    );
  }

  #[test]
  fn binary_apps() {
    let mut symbol_table = SymbolTable::default();
    let type_defs = TypeDefs::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table, &type_defs);

    /* 5 + false: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::IntLiter(5)),
      BinaryOper::Add,
      Box::new(Expr::BoolLiter(false))
    )
    .get_type(scope)
    .is_err());

    /* 5 * 'a': ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::IntLiter(5)),
      BinaryOper::Mul,
      Box::new(Expr::CharLiter('a'))
    )
    .get_type(scope)
    .is_err());

    /* false / "hello": ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::BoolLiter(false)),
      BinaryOper::Div,
      Box::new(Expr::StrLiter(String::from("hello")))
    )
    .get_type(scope)
    .is_err());

    /* false && 6: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::BoolLiter(false)),
      BinaryOper::And,
      Box::new(Expr::IntLiter(6))
    )
    .get_type(scope)
    .is_err());

    /* MATH CAN BE DONE ON INTS. */
    /* 5 * 5: Int */
    assert_eq!(
      Expr::BinaryApp(
        Box::new(Expr::IntLiter(5)),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(5)),
      )
      .get_type(scope),
      Ok(Type::Int),
    );

    /* 5 + 5: Int */
    assert_eq!(
      Expr::BinaryApp(
        Box::new(Expr::IntLiter(5)),
        BinaryOper::Add,
        Box::new(Expr::IntLiter(5)),
      )
      .get_type(scope),
      Ok(Type::Int),
    );

    /* MATH CANT BE DONE ON ANYTHING ELSE */
    /* 'a' + 'b': ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::CharLiter('a')),
      BinaryOper::Add,
      Box::new(Expr::CharLiter('b'))
    )
    .get_type(scope)
    .is_err());

    /* false + false: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::BoolLiter(false)),
      BinaryOper::Add,
      Box::new(Expr::BoolLiter(false))
    )
    .get_type(scope)
    .is_err());

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
            .get_type(scope),
          Ok(Type::Bool)
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
            .get_type(scope),
          Ok(Type::Bool)
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
    .get_type(scope)
    .is_err());

    /* 'a' || 'a': ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::CharLiter('a')),
      BinaryOper::Or,
      Box::new(Expr::CharLiter('a')),
    )
    .get_type(scope)
    .is_err());

    /* true && true: bool */
    assert_eq!(
      Expr::BinaryApp(
        Box::new(Expr::BoolLiter(true)),
        BinaryOper::And,
        Box::new(Expr::BoolLiter(true)),
      )
      .get_type(scope),
      Ok(Type::Bool)
    );
  }
}
