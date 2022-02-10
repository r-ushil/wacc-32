use super::{equal_types, expected_type, symbol_table::SymbolTable, AResult, HasType};
use crate::ast::*;

impl HasType for Expr {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    Ok(match self {
      Expr::IntLiter(_) => Type::BaseType(BaseType::Int),
      Expr::BoolLiter(_) => Type::BaseType(BaseType::Bool),
      Expr::CharLiter(_) => Type::BaseType(BaseType::Char),
      Expr::StrLiter(_) => Type::BaseType(BaseType::String),
      Expr::PairLiter => Type::Pair(
        Box::new(Type::BaseType(BaseType::Any)),
        Box::new(Type::BaseType(BaseType::Any)),
      ),
      Expr::Ident(id) => id.get_type(symbol_table)?,
      Expr::ArrayElem(elem) => elem.get_type(symbol_table)?,
      Expr::UnaryApp(op, exp) => match op {
        UnaryOper::Bang => {
          expected_type(symbol_table, &Type::BaseType(BaseType::Bool), exp)?.clone()
        },
        UnaryOper::Neg => expected_type(symbol_table, &Type::BaseType(BaseType::Int), exp)?.clone(),
        UnaryOper::Len => match exp.get_type(symbol_table)? {
          Type::Array(_) => Type::BaseType(BaseType::Int),
          t => {
            return Err(format!(
              "TYPE ERROR: Attempt to find length of non array\n\tExpected: Array\n\tActual: {:?}",
              t
            ))
          },
        },
        UnaryOper::Ord => {
          expected_type(symbol_table, &Type::BaseType(BaseType::Char), exp)?;
          Type::BaseType(BaseType::Int)
        },
        UnaryOper::Chr => {
          expected_type(symbol_table, &Type::BaseType(BaseType::Int), exp)?;
          Type::BaseType(BaseType::Char)
        },
      },

      Expr::BinaryApp(exp1, op, exp2) => {
        /* Every binary application requires both expressions to have the same type. */
        let expr_type = equal_types(symbol_table, exp1, exp2)?;

        match op {
          /* Maths can only be done on ints. */
          BinaryOper::Mul
          | BinaryOper::Div
          | BinaryOper::Mod
          | BinaryOper::Add
          | BinaryOper::Sub => match expr_type {
            Type::BaseType(BaseType::Int) => Type::BaseType(BaseType::Int),
            t => {
              return Err(format!(
                "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
                op, t
              ))
            },
          },
          /* Any types can be compared. */
          BinaryOper::Gt
          | BinaryOper::Gte
          | BinaryOper::Lt
          | BinaryOper::Lte
          | BinaryOper::Eq
          | BinaryOper::Neq => Type::BaseType(BaseType::Bool),
          /* Boolean operators can only be applied to booleans. */
          BinaryOper::And | BinaryOper::Or => match expr_type {
            Type::BaseType(BaseType::Bool) => Type::BaseType(BaseType::Bool),
            t => {
              return Err(format!(
                "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
                op, t
              ))
            },
          },
        }
      },
    })
  }
}

#[cfg(test)]
mod tests {
  use std::fmt::Binary;

  use super::*;
  use crate::analyser::symbol_table;

  /* Defines a scope with 10 variables, each starting with prefix and ending
   * with 0..10 */
  fn populate_symbol_table(symbol_table: &mut SymbolTable, prefix: &str) {
    for i in 0..10 {
      let ident = Ident(format!("{}{}", prefix, i));
      let t = Type::BaseType(BaseType::Int);
      symbol_table.insert(&ident, t);
    }
  }

  #[test]
  fn literals() {
    let symbol_table = &SymbolTable::new();

    assert_eq!(
      Expr::IntLiter(5).get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Int))
    );
    assert_eq!(
      Expr::BoolLiter(false).get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Bool))
    );
    assert_eq!(
      Expr::CharLiter('a').get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Char))
    );
    assert_eq!(
      Expr::PairLiter.get_type(symbol_table),
      Ok(Type::Pair(
        Box::new(Type::BaseType(BaseType::Any)),
        Box::new(Type::BaseType(BaseType::Any))
      )),
    );
  }

  #[test]
  fn idents() {
    let mut symbol_table = SymbolTable::new();
    populate_symbol_table(&mut symbol_table, "var");

    assert_eq!(
      Expr::Ident(Ident(String::from("var1"))).get_type(&symbol_table),
      Ok(Type::BaseType(BaseType::Int)),
    );
  }

  #[test]
  fn array_elems() {
    let x = Ident(String::from("x"));
    let x_type = Type::Array(Box::new(Type::BaseType(BaseType::Int)));

    let mut symbol_table = SymbolTable::new();
    symbol_table.insert(&x, x_type);

    assert_eq!(
      Expr::ArrayElem(ArrayElem(x, vec!(Expr::IntLiter(5)))).get_type(&symbol_table),
      Ok(Type::BaseType(BaseType::Int))
    );
  }

  #[test]
  fn unary_apps() {
    /* Symbol Table */
    let symbol_table = &mut SymbolTable::new();

    /* BANG */
    /* !false: Bool */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Bang, Box::new(Expr::BoolLiter(false))).get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Bool))
    );

    /* !'a': ERROR */
    assert!(
      Expr::UnaryApp(UnaryOper::Bang, Box::new(Expr::CharLiter('a')))
        .get_type(symbol_table)
        .is_err()
    );

    /* NEG */
    /* -5: Int */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Neg, Box::new(Expr::IntLiter(5))).get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Int))
    );

    /* -false: ERROR */
    assert!(
      Expr::UnaryApp(UnaryOper::Neg, Box::new(Expr::BoolLiter(false)))
        .get_type(symbol_table)
        .is_err()
    );

    /* LEN */
    /* len [1,2,3]: Int */
    let x = Ident(String::from("x"));
    let x_type = Type::Array(Box::new(Type::BaseType(BaseType::Int)));
    symbol_table.insert(&x, x_type);
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Len, Box::new(Expr::Ident(x))).get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Int))
    );

    /* len 5: ERROR */
    assert!(Expr::UnaryApp(UnaryOper::Len, Box::new(Expr::IntLiter(5)))
      .get_type(symbol_table)
      .is_err());

    /* ORD */
    /* ord 'a': Int */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Ord, Box::new(Expr::CharLiter('a'))).get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Int))
    );

    /* ord 5: ERROR */
    assert!(Expr::UnaryApp(UnaryOper::Ord, Box::new(Expr::IntLiter(5)))
      .get_type(symbol_table)
      .is_err());

    /* CHR */
    /* chr 5: Char */
    assert_eq!(
      Expr::UnaryApp(UnaryOper::Chr, Box::new(Expr::IntLiter(5))).get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Char))
    );

    /* chr 'a': ERROR */
    assert!(
      Expr::UnaryApp(UnaryOper::Chr, Box::new(Expr::CharLiter('a')))
        .get_type(symbol_table)
        .is_err()
    );
  }

  #[test]
  fn binary_apps() {
    let symbol_table = &mut SymbolTable::new();

    /* 5 + false: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::IntLiter(5)),
      BinaryOper::Add,
      Box::new(Expr::BoolLiter(false))
    )
    .get_type(symbol_table)
    .is_err());

    /* 5 * 'a': ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::IntLiter(5)),
      BinaryOper::Mul,
      Box::new(Expr::CharLiter('a'))
    )
    .get_type(symbol_table)
    .is_err());

    /* false / "hello": ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::BoolLiter(false)),
      BinaryOper::Div,
      Box::new(Expr::StrLiter(String::from("hello")))
    )
    .get_type(symbol_table)
    .is_err());

    /* false && 6: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::BoolLiter(false)),
      BinaryOper::And,
      Box::new(Expr::IntLiter(6))
    )
    .get_type(symbol_table)
    .is_err());

    /* MATH CAN BE DONE ON INTS. */
    /* 5 * 5: Int */
    assert_eq!(
      Expr::BinaryApp(
        Box::new(Expr::IntLiter(5)),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(5)),
      )
      .get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Int)),
    );

    /* 5 + 5: Int */
    assert_eq!(
      Expr::BinaryApp(
        Box::new(Expr::IntLiter(5)),
        BinaryOper::Add,
        Box::new(Expr::IntLiter(5)),
      )
      .get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Int)),
    );

    /* MATH CANT BE DONE ON ANYTHING ELSE */
    /* 'a' + 'b': ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::CharLiter('a')),
      BinaryOper::Add,
      Box::new(Expr::CharLiter('b'))
    )
    .get_type(symbol_table)
    .is_err());

    /* false + false: ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::BoolLiter(false)),
      BinaryOper::Add,
      Box::new(Expr::BoolLiter(false))
    )
    .get_type(symbol_table)
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
      for oper in vec![
        BinaryOper::Gt,
        BinaryOper::Gte,
        BinaryOper::Lt,
        BinaryOper::Lte,
        BinaryOper::Eq,
        BinaryOper::Neq,
      ] {
        assert_eq!(
          Expr::BinaryApp(Box::new(expr.clone()), oper, Box::new(expr.clone()))
            .get_type(symbol_table),
          Ok(Type::BaseType(BaseType::Bool))
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
    .get_type(symbol_table)
    .is_err());

    /* 'a' || 'a': ERROR */
    assert!(Expr::BinaryApp(
      Box::new(Expr::CharLiter('a')),
      BinaryOper::Or,
      Box::new(Expr::CharLiter('a')),
    )
    .get_type(symbol_table)
    .is_err());

    /* true && true: bool */
    assert_eq!(
      Expr::BinaryApp(
        Box::new(Expr::BoolLiter(true)),
        BinaryOper::And,
        Box::new(Expr::BoolLiter(true)),
      )
      .get_type(symbol_table),
      Ok(Type::BaseType(BaseType::Bool))
    );
  }
}
