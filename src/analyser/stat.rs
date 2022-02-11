use super::{
  equal_types, expected_type, symbol_table::SymbolTable, unify::Unifiable, AResult, HasType,
};
use crate::ast::*;

pub enum ReturnBehaviour {
  Never,        /* Statement never returns. */
  MidWay(Type), /* Statement returns at least sometimes, but not at the end. */
  AtEnd(Type),  /* Statement always returns at the end. */
}

impl ReturnBehaviour {
  fn same_return(&self, other: &ReturnBehaviour) -> bool {
    use ReturnBehaviour::*;

    !matches!((self, other), (MidWay(true_type) | AtEnd(true_type), MidWay(false_type) | AtEnd(false_type)) if true_type.clone().unify(false_type.clone()) == None)
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
      AssignRhs::Pair(e1, e2) => Type::Pair(
        Box::new(e1.get_type(symbol_table)?),
        Box::new(e2.get_type(symbol_table)?),
      ),
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
        },
        t => {
          return Err(format!(
            "TYPE ERROR:\n\tExpected: Function\n\tActual: {:?}",
            t
          ))
        },
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
    let ArrayLiter(exprs) = self;

    Ok(Type::Array(Box::new(match exprs.first() {
      None => Type::Any,
      Some(first) => {
        /* Take first element as source of truth. */
        let first_type = first.get_type(symbol_table)?;

        /* Ensure every other element has same type. */
        for expr in &exprs[1..] {
          let expr_type = expr.get_type(symbol_table)?;
          if first_type != expr_type {
            return Err(format!("Array literal value has wrong type\n\tValue: {:?}\n\tExpected Type: {:?}\n\tActual Type: {:?}", expr, first_type, expr_type));
          }
        }

        first_type
      },
    })))
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
      expected_type(symbol_table, &expected, val)?;

      /* Adds identifier to symbol table. */
      symbol_table.insert(id, expected.clone())?;

      Ok(Never) /* Declarations never return. */
    },
    Stat::Assignment(lhs, rhs) => {
      equal_types(symbol_table, lhs, rhs)?;
      Ok(Never) /* Assignments never return. */
    },
    Stat::Read(dest) => {
      /* Any type can be read. */
      dest.get_type(symbol_table)?;
      /* Reads never return. */
      Ok(Never)
    },
    Stat::Free(expr) => match expr.get_type(symbol_table)? {
      Type::Pair(_, _) | Type::Array(_) => Ok(Never), /* Frees never return. */
      actual_type => Err(format!(
        "TYPE ERROR: Expected Type\n\tExpected: Pair or Array\n\tActual:{:?}",
        actual_type
      )),
    },
    Stat::Return(expr) => Ok(AtEnd(expr.get_type(symbol_table)?)), /* Returns always return. */
    Stat::Exit(expr) => {
      /* Exit codes must be integers. */
      expected_type(symbol_table, &Type::Int, expr)?;
      /* Exits can be concidered to return because they will never return the
      wrong type, by using any it won't collide with another type. */
      Ok(AtEnd(Type::Any))
    },
    Stat::Print(expr) | Stat::Println(expr) => {
      /* Any type can be printed. */
      expr.get_type(symbol_table)?;

      /* Prints never return. */
      Ok(Never)
    },
    Stat::If(cond, if_stat, else_stat) => {
      expected_type(symbol_table, &Type::Bool, cond)?;

      /* If both branches return the same type, the if statement can
      be relied on to return that type. */
      let true_behaviour = stat(&mut symbol_table.new_scope(), if_stat)?;
      let false_behaviour = stat(&mut symbol_table.new_scope(), else_stat)?;

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
        (MidWay(t) | AtEnd(t), _) | (_, MidWay(t) | AtEnd(t)) => t,
      };

      /* Determine how often that return type is returned. */
      if let (AtEnd(_), AtEnd(_)) = (&true_behaviour, &false_behaviour) {
        /* If both branches end in returns, the if statement ends in a return. */
        Ok(AtEnd(return_type.clone()))
      } else {
        /* Otherwise, the if statement doesn't end with a return. */
        Ok(MidWay(return_type.clone()))
      }
    },
    Stat::While(cond, body) => {
      expected_type(symbol_table, &Type::Bool, cond)?;

      Ok(match stat(&mut symbol_table.new_scope(), body)? {
        /* If the body always returns, while loop might still not return
        because the cond might always be false and the body never run. */
        AtEnd(t) => MidWay(t),
        /* Otherwise white loop returns the same way it's body does. */
        b => b,
      })
    },
    Stat::Scope(body) => return stat(&mut symbol_table.new_scope(), body),
    Stat::Sequence(fst, snd) => {
      /* CHECK: no definite returns before last line. */
      let lhs = stat(symbol_table, fst)?;
      let rhs = stat(symbol_table, snd)?;

      /* Even if RHS never returns, the statement overall might still return
      if the LHS returns some or all of the time. */
      Ok(if let (Never, MidWay(t) | AtEnd(t)) = (&rhs, lhs) {
        MidWay(t)
      } else {
        rhs
      })
    },
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
