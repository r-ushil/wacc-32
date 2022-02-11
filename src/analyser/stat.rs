use std::array;

use super::{
  context::{Context, ContextLocation},
  equal_types, expected_type,
  unify::Unifiable,
  HasType, SemanticError,
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
  fn get_type(&self, context: &Context, errors: &mut Vec<SemanticError>) -> Option<Type> {
    match self {
      AssignLhs::Ident(id) => id.get_type(context, errors),
      AssignLhs::ArrayElem(elem) => elem.get_type(context, errors),
      AssignLhs::PairElem(elem) => elem.get_type(context, errors),
    }
  }
}

#[allow(unused_variables)]
impl HasType for AssignRhs {
  fn get_type(&self, context: &Context, errors: &mut Vec<SemanticError>) -> Option<Type> {
    match self {
      AssignRhs::Expr(exp) => exp.get_type(context, errors),
      AssignRhs::ArrayLiter(lit) => lit.get_type(context, errors),
      AssignRhs::Pair(e1, e2) => {
        if let (Some(lhs_type), Some(rhs_type)) =
          (e1.get_type(context, errors), e2.get_type(context, errors))
        {
          Some(Type::Pair(
            Box::new(lhs_type.clone()),
            Box::new(rhs_type.clone()),
          ))
        } else {
          None
        }
      }
      AssignRhs::PairElem(elem) => elem.get_type(context, errors),
      AssignRhs::Call(id, args) => match id.get_type(context, errors)? {
        Type::Func(bx) => {
          let FuncSig {
            params,
            return_type,
          } = *bx;
          let mut errored = false;

          /* Must be same amount of args as parameters */
          if params.len() != args.len() {
            context.add_error(
              errors,
              SemanticError::Normal(format!("Function called with wrong amount of arguments.")),
            );
            errored = true;
          }

          /* Types must be pairwise the same. */
          for (arg, (param_type, param_id)) in args.iter().zip(params.iter()) {
            if arg
              .clone()
              .get_type(context, errors)?
              .unify(param_type.clone())
              .is_none()
            {
              context.add_error(
                errors,
                SemanticError::Normal(format!("Incorrect type passed to function.")),
              );
              errored = true;
            }
          }

          if errored {
            None
          } else {
            Some(return_type)
          }
        }
        t => {
          context.add_error(
            errors,
            SemanticError::Normal(format!(
              "TYPE ERROR:\n\tExpected: Function\n\tActual: {:?}",
              t
            )),
          );
          None
        }
      },
    }
  }
}

impl HasType for PairElem {
  fn get_type(&self, context: &Context, errors: &mut Vec<SemanticError>) -> Option<Type> {
    match self {
      PairElem::Fst(p) => match p.get_type(context, errors)? {
        Type::Pair(left, _) => match *left {
          Type::Any => {
            context.add_error(
              errors,
              SemanticError::Normal(format!("TYPE ERROR:\n\tExpected: BaseType\n\tActual: Null")),
            );
            None
          }
          t => Some(t),
        },
        t => {
          context.add_error(
            errors,
            SemanticError::Normal(format!("TYPE ERROR:\n\tExpected: Pair\n\tActual:{:?}", t)),
          );
          None
        }
      },
      PairElem::Snd(p) => match p.get_type(context, errors)? {
        Type::Pair(_, right) => match *right {
          Type::Any => {
            context.add_error(
              errors,
              SemanticError::Normal(format!("TYPE ERROR:\n\tExpected: BaseType\n\tActual: Null")),
            );
            None
          }
          t => Some(t),
        },
        t => {
          context.add_error(
            errors,
            SemanticError::Normal(format!("TYPE ERROR:\n\tExpected: Pair\n\tActual:{:?}", t)),
          );
          None
        }
      },
    }
  }
}

impl HasType for ArrayLiter {
  fn get_type(&self, context: &Context, errors: &mut Vec<SemanticError>) -> Option<Type> {
    let ArrayLiter(exprs) = self;

    /* Take first element as source of truth. */
    let mut array_type = Some(Type::Any);

    /* Ensure every other element has same type. */
    for expr in exprs {
      if let Some(expr_type) = expr.get_type(context, errors) {
        if let Some(t) = array_type {
          array_type = t.unify(expr_type)
        }
      }
    }

    Some(Type::Array(Box::new(array_type?)))
  }
}

/* Type checks a statement.
Declarations will add to the symbol table.
Scopes will make a new scope within the symbol table.
If the statment ALWAYS returns with the same type, returns that type. */
#[allow(dead_code)]
pub fn stat(
  context: &mut Context,
  errors: &mut Vec<SemanticError>,
  statement: &Stat,
) -> Option<ReturnBehaviour> {
  use ReturnBehaviour::*;

  /* Returns error if there is any. */
  match statement {
    Stat::Skip => Some(Never), /* Skips never return. */
    Stat::Declaration(expected, id, val) => {
      if let (Some(_), Some(_)) = (
        expected_type(context, errors, &expected, val),
        /* Adds identifier to symbol table. */
        context.insert(id, expected.clone()),
      ) {
        Some(Never)
      } else {
        None
      }
    }
    Stat::Assignment(lhs, rhs) => {
      equal_types(context, errors, lhs, rhs)?;
      Some(Never) /* Assignments never return. */
    }
    Stat::Read(dest) => {
      /* Any type can be read. */
      match dest.get_type(context, errors)? {
        /* Reads never return. */
        Type::Int | Type::Char => Some(Never),
        _ => {
          context.add_error(
            errors,
            SemanticError::Normal(format!("Read statements must read char or int.")),
          );
          None
        } /*  */
      }
    }
    Stat::Free(expr) => match expr.get_type(context, errors)? {
      Type::Pair(_, _) | Type::Array(_) => Some(Never), /* Frees never return. */
      actual_type => {
        context.add_error(
          errors,
          SemanticError::Normal(format!(
            "TYPE ERROR: Expected Type\n\tExpected: Pair or Array\n\tActual:{:?}",
            actual_type
          )),
        );
        None
      }
    },
    Stat::Return(expr) => Some(AtEnd(expr.get_type(context, errors)?)), /* Returns always return. */
    Stat::Exit(expr) => {
      /* Exit codes must be integers. */
      expected_type(context, errors, &Type::Int, expr)?;
      /* Exits can be concidered to return because they will never return the
      wrong type, by using any it won't collide with another type. */
      Some(AtEnd(Type::Any))
    }
    Stat::Print(expr) | Stat::Println(expr) => {
      /* Any type can be printed. */
      expr.get_type(context, errors)?;

      /* Prints never return. */
      Some(Never)
    }
    Stat::If(cond, if_stat, else_stat) => {
      let cond_fine = cond.get_type(context, errors) == Some(Type::Bool);

      /* If both branches return the same type, the if statement can
      be relied on to return that type. */
      if let (Some(true_behaviour), Some(false_behaviour)) = (
        stat(
          &mut context.new_context(ContextLocation::If),
          errors,
          if_stat,
        ),
        stat(
          &mut context.new_context(ContextLocation::If),
          errors,
          else_stat,
        ),
      ) {
        if !cond_fine {
          return None;
        }

        /* If branches return with different types, if statement is error. */
        if !true_behaviour.same_return(&false_behaviour) {
          context.add_error(
            errors,
            SemanticError::Normal(format!(
              "Branches of if statement return values of different types."
            )),
          );
          return None;
        }

        /* Get return type. */
        let return_type = match (&true_behaviour, &false_behaviour) {
          /* If both branches never return, if statement never returns. */
          (Never, Never) => return Some(Never),
          /* Otherwise, if statement returns the same type as one of its branches. */
          (MidWay(t) | AtEnd(t), _) | (_, MidWay(t) | AtEnd(t)) => t,
        };

        /* Determine how often that return type is returned. */
        if let (AtEnd(_), AtEnd(_)) = (&true_behaviour, &false_behaviour) {
          /* If both branches end in returns, the if statement ends in a return. */
          Some(AtEnd(return_type.clone()))
        } else {
          /* Otherwise, the if statement doesn't end with a return. */
          Some(MidWay(return_type.clone()))
        }
      } else {
        None
      }
    }
    Stat::While(cond, body) => {
      let cond_fine = cond.get_type(context, errors) == Some(Type::Bool);

      let statement_result = stat(
        &mut context.new_context(ContextLocation::While),
        errors,
        body,
      )?;

      if !cond_fine {
        return None;
      }

      Some(match statement_result {
        /* If the body always returns, while loop might still not return
        because the cond might always be false and the body never run. */
        AtEnd(t) => MidWay(t),
        /* Otherwise white loop returns the same way it's body does. */
        b => b,
      })
    }
    Stat::Scope(body) => stat(
      &mut context.new_context(ContextLocation::Scope),
      errors,
      body,
    ),
    Stat::Sequence(fst, snd) => {
      /* CHECK: no definite returns before last line. */
      if let (Some(lhs), Some(rhs)) = (stat(context, errors, fst), stat(context, errors, snd)) {
        /* Even if RHS never returns, the statement overall might still return
        if the LHS returns some or all of the time. */
        Some(if let (Never, MidWay(t) | AtEnd(t)) = (&rhs, lhs) {
          MidWay(t)
        } else {
          rhs
        })
      } else {
        None
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn assign_lhs() {
    let context = &mut Context::new();

    /* Identifiers cause */
    let x_id = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));
    context.insert(&x_id, x_type.clone()).unwrap();
    assert_eq!(
      AssignLhs::Ident(x_id.clone()).get_type(context, &mut vec![]),
      Some(x_type.clone())
    );

    assert!(AssignRhs::PairElem(PairElem::Fst(Expr::PairLiter))
      .get_type(context, &mut vec![])
      .is_some());

    assert!(AssignRhs::PairElem(PairElem::Fst(Expr::PairLiter))
      .get_type(context, &mut vec![])
      .is_some());

    assert_eq!(
      AssignLhs::ArrayElem(ArrayElem(x_id.clone(), vec!(Expr::IntLiter(5))))
        .get_type(context, &mut vec![]),
      Some(x_type.clone())
    );
  }
}
