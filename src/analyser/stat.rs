use super::{equal_types, expected_type, symbol_table::SymbolTable, AResult, HasType};
use crate::ast::*;

impl HasType for AssignLhs {
  fn get_type(&self, _symbol_table: &SymbolTable) -> AResult<Type> {
    todo!();
  }
}

impl HasType for AssignRhs {
  fn get_type(&self, _symbol_table: &SymbolTable) -> AResult<Type> {
    todo!();
  }
}

impl HasType for PairElem {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    Ok(match self {
      PairElem::Fst(exp) | PairElem::Snd(exp) => exp.get_type(symbol_table)?,
    })
  }
}

impl HasType for ArrayLiter {
  fn get_type(&self, _symbol_table: &SymbolTable) -> AResult<Type> {
    todo!();
  }
}

/* Type checks a statement.
Declarations will add to the symbol table.
Scopes will make a new scope within the symbol table.
If the statment ALWAYS returns with the same type, returns that type. */
fn stat(symbol_table: &mut SymbolTable, statement: &Stat) -> AResult<Option<Type>> {
  /* Returns error if there is any. */
  match statement {
    Stat::Skip => (),
    Stat::Declaration(expected, id, val) => {
      /* Adds identifier to symbol table. */
      symbol_table.insert(id, expected.clone());

      expected_type(symbol_table, &expected, val)?;
    },
    Stat::Assignment(lhs, rhs) => {
      equal_types(symbol_table, lhs, rhs)?;
    }, // x = [1,2,3]
    Stat::Read(dest) => {
      expected_type(symbol_table, &Type::BaseType(BaseType::String), dest)?;
    },
    Stat::Free(expr) => match expr.get_type(symbol_table)? {
      Type::Pair(_, _) | Type::Array(_) => (),
      actual_type => {
        return Err(format!(
          "TYPE ERROR: Expected Type\n\tExpected: Pair or Array\n\tActual:{:?}",
          actual_type
        ))
      },
    },
    Stat::Return(expr) => {
      return Ok(Some(expr.get_type(symbol_table)?));
    },
    Stat::Exit(expr) => {
      expected_type(symbol_table, &Type::BaseType(BaseType::Int), expr)?;
    },
    Stat::Print(expr) | Stat::Println(expr) => {
      expected_type(symbol_table, &Type::BaseType(BaseType::String), expr)?;
    },
    Stat::If(cond, if_stat, else_stat) => {
      expected_type(symbol_table, &Type::BaseType(BaseType::Bool), cond)?;

      /* If both branches return the same type, the if statement can
      be relied on to return that type. */
      if let (Some(a), Some(b)) = (stat(symbol_table, if_stat)?, stat(symbol_table, else_stat)?) {
        if a == b {
          return Ok(Some(a));
        }
      }
    },
    Stat::While(cond, body) => {
      expected_type(symbol_table, &Type::BaseType(BaseType::Bool), cond)?;

      /* Even if body does return, statement can't be guarenteed to return
      because the body might not run. */
      stat(symbol_table, body)?;
    },
    Stat::Scope(body) => return stat(symbol_table, body),
    Stat::Sequence(fst, snd) => {
      /* NOTE: if x always returns in x;y, y will not get type checked. */
      return stat(symbol_table, fst).or(stat(symbol_table, snd));
    },
  }

  /* Returns with Ok otherwise. */
  Ok(None)
}

#[cfg(test)]
mod tests {

  #[test]
  fn test_pair_elem_has_type() {}
}
