use super::{equal_types, expected_type, symbol_table::SymbolTable, AResult, HasType};
use crate::ast::*;

/*

pair(int, int) x = null

is type(null) == pair(int, int)

*/

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
          expected_type(symbol_table, &Type::BaseType(BaseType::Char), exp)?.clone()
        },
        UnaryOper::Chr => expected_type(symbol_table, &Type::BaseType(BaseType::Int), exp)?.clone(),
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
  use super::*;

  /* Defines a scope with 10 variables, each starting with prefix and ending with 0..10 */
  fn populate_symbol_table(symbol_table: &mut SymbolTable, prefix: &str) {
    for i in 0..10 {
      let ident = Ident(format!("{}{}", prefix, i));
      let t = Type::BaseType(BaseType::Int);
      symbol_table.insert(&ident, t);
    }
  }

  #[test]
  fn test_literals() {
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
  fn test_ident() {
    let mut outer = SymbolTable::new();
    populate_symbol_table(&mut outer, "outer");
    let _inner = outer.new_scope();
  }
}
