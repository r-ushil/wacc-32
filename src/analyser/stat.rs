use super::{equal_types, expected_type, symbol_table::SymbolTable, AResult, HasType};
use crate::ast::*;

/* -1 => Never returns */
/* 0 => Sometimes returns */
/* 1 => Always returns */
pub enum ReturnBehaviour {
  Never,
  Sometimes(Type),
  Always(Type),
}

impl ReturnBehaviour {
  fn same_return(&self, other: &ReturnBehaviour) -> bool {
    use ReturnBehaviour::*;

    if let (Sometimes(true_type) | Always(true_type), Sometimes(false_type) | Always(false_type)) =
      (self, other)
    {
      if true_type != false_type {
        return false;
      }
    }

    true
  }
}

impl HasType for AssignLhs {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    Ok(match self {
      AssignLhs::Ident(id) => id.get_type(symbol_table)?,
      AssignLhs::ArrayElem(elem) => elem.get_type(symbol_table)?,
      AssignLhs::PairElem(elem) => elem.get_type(symbol_table)?,
    })
  }
}

#[allow(unused_variables)]
impl HasType for AssignRhs {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    Ok(match self {
      AssignRhs::Expr(exp) => exp.get_type(symbol_table)?,
      AssignRhs::ArrayLiter(lit) => lit.get_type(symbol_table)?,
      AssignRhs::Pair(e1, e2) => todo!(),
      AssignRhs::PairElem(elem) => elem.get_type(symbol_table)?,
      AssignRhs::Call(id, args) => match id.get_type(symbol_table)? {
        Type::Func(bx) => {
          let FuncSig {
            params,
            return_type,
          } = *bx;

          /* Must be same amount of args as parameters */
          if params.len() != args.len() {
            return Err(format!("Function called with wrong amount of arguments."));
          }

          /* Types must be pairwise the same. */
          for (arg, (param_type, param_id)) in args.iter().zip(params.iter()) {
            if &arg.get_type(symbol_table)? != param_type {
              return Err(format!("Incorrect type passed to function."));
            }
          }

          return_type
        }
        t => {
          return Err(format!(
            "TYPE ERROR:\n\tExpected: Function\n\tActual: {:?}",
            t
          ))
        }
      },
    })
  }
}

impl HasType for PairElem {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    Ok(match self {
      PairElem::Fst(p) => match p.get_type(symbol_table)? {
        Type::Pair(left, _) => *left,
        t => return Err(format!("TYPE ERROR:\n\tExpected: Pair\n\tActual:{:?}", t)),
      },
      PairElem::Snd(p) => match p.get_type(symbol_table)? {
        Type::Pair(_, right) => *right,
        t => return Err(format!("TYPE ERROR:\n\tExpected: Pair\n\tActual:{:?}", t)),
      },
    })
  }
}

impl HasType for ArrayLiter {
  fn get_type(&self, symbol_table: &SymbolTable) -> AResult<Type> {
    if self.0.is_empty() {
      todo!("Handle empty array");
    } else {
      let first = self.0.first().unwrap().get_type(symbol_table)?;
      for expr in &self.0[1..] {
        if !(expr.get_type(symbol_table)? == first) {
          break;
        }
      }
    }

    Err("Mismatched type".to_string())
  }
}

/* Type checks a statement.
Declarations will add to the symbol table.
Scopes will make a new scope within the symbol table.
If the statment ALWAYS returns with the same type, returns that type. */
#[allow(dead_code)]
pub fn stat(symbol_table: &mut SymbolTable, statement: &Stat) -> AResult<ReturnBehaviour> {
  use ReturnBehaviour::*;

  /* Returns error if there is any. */
  match statement {
    Stat::Skip => Ok(Never), /* Skips never return. */
    Stat::Declaration(expected, id, val) => {
      /* Adds identifier to symbol table. */
      symbol_table.insert(id, expected.clone())?;

      expected_type(symbol_table, &expected, val)?;

      Ok(Never) /* Declarations never return. */
    }
    Stat::Assignment(lhs, rhs) => {
      equal_types(symbol_table, lhs, rhs)?;
      Ok(Never) /* Assignments never return. */
    } // x = [1,2,3]
    Stat::Read(dest) => {
      expected_type(symbol_table, &Type::String, dest)?;
      Ok(Never) /* Reads never return. */
    }
    Stat::Free(expr) => match expr.get_type(symbol_table)? {
      Type::Pair(_, _) | Type::Array(_) => Ok(Never), /* Frees never return. */
      actual_type => Err(format!(
        "TYPE ERROR: Expected Type\n\tExpected: Pair or Array\n\tActual:{:?}",
        actual_type
      )),
    },
    Stat::Return(expr) => Ok(Always(expr.get_type(symbol_table)?)), /* Returns always return. */
    Stat::Exit(expr) => {
      expected_type(symbol_table, &Type::Int, expr)?;
      Ok(Always(Type::Any)) /* Exit never return. */
    }
    Stat::Print(expr) | Stat::Println(expr) => {
      expected_type(symbol_table, &Type::String, expr)?;
      Ok(Never) /* Prints never return. */
    }
    Stat::If(cond, if_stat, else_stat) => {
      expected_type(symbol_table, &Type::Bool, cond)?;

      /* If both branches return the same type, the if statement can
      be relied on to return that type. */
      let true_behaviour = stat(symbol_table, if_stat)?;
      let false_behaviour = stat(symbol_table, else_stat)?;

      /* If branches return with different types, if statement is error. */
      if !true_behaviour.same_return(&false_behaviour) {
        return Err(format!(
          "Branches of if statement return values of different types."
        ));
      }

      /* Get return type. */
      let return_type = match (&true_behaviour, &false_behaviour) {
        /* If both branches never return, if statement never returns. */
        (Never, Never) => return Ok(Never),
        /* Otherwise, if statement returns the same type as one of its branches. */
        (Sometimes(t) | Always(t), _) | (_, Sometimes(t) | Always(t)) => t,
      };

      /* Determine how often that return type is returned. */
      if let (Always(_), Always(_)) = (&true_behaviour, &false_behaviour) {
        /* If both branches always return, the if statement always returns. */
        Ok(Always(return_type.clone()))
      } else {
        /* Otherwise, the if statement can't be relied on to return. */
        Ok(Sometimes(return_type.clone()))
      }
    }
    Stat::While(cond, body) => {
      expected_type(symbol_table, &Type::Bool, cond)?;

      Ok(match stat(symbol_table, body)? {
        /* If the body always returns, while loop might still not return
        because the cond might always be false and the body never run. */
        Always(t) => Sometimes(t),
        /* Otherwise white loop returns the same way it's body does. */
        b => b,
      })
    }
    Stat::Scope(body) => return stat(symbol_table, body),
    Stat::Sequence(fst, snd) => {
      /* CHECK: no definite returns before last line. */
      let lhs = stat(symbol_table, fst)?;
      if let Always(_) = &lhs {
        return Err(format!("Return before end of function"));
      }

      let rhs = stat(symbol_table, snd)?;
      Ok(if let (Sometimes(_), Never) = (&lhs, &rhs) {
        /* If lhs sometimes returns the expression might still return
        even if the rhs never returns. */
        lhs
      } else {
        /* Otherwise the return behaviour is determined by the rhs. */
        rhs
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn assign_lhs() {
    let symbol_table = &mut SymbolTable::new();

    /* Identifiers cause */
    let x_id = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));
    symbol_table.insert(&x_id, x_type.clone()).unwrap();
    assert_eq!(
      AssignLhs::Ident(x_id.clone()).get_type(symbol_table),
      Ok(x_type.clone())
    );

    /*  */
    // assert_eq!(
    //   AssignLhs::ArrayElem(ArrayElem(x_id.clone(),
    // vec!(Expr::IntLiter(5)))).get_type(symbol_table),   Ok(x_type.
    // clone()) );
  }
}
