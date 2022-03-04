use super::{
  context::{ScopeBuilder, SymbolTable},
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
  fn get_type(&mut self, scope: &ScopeBuilder, errors: &mut Vec<SemanticError>) -> Option<Type> {
    match self {
      AssignLhs::Ident(id) => id.get_type(scope, errors),
      AssignLhs::ArrayElem(elem) => elem.get_type(scope, errors),
      AssignLhs::PairElem(elem) => elem.get_type(scope, errors),
    }
  }
}

#[allow(unused_variables)]
impl HasType for AssignRhs {
  fn get_type(&mut self, scope: &ScopeBuilder, errors: &mut Vec<SemanticError>) -> Option<Type> {
    match self {
      AssignRhs::Expr(exp) => exp.get_type(scope, errors),
      AssignRhs::ArrayLiter(lit) => lit.get_type(scope, errors),
      AssignRhs::Pair(e1, e2) => {
        if let (Some(lhs_type), Some(rhs_type)) =
          (e1.get_type(scope, errors), e2.get_type(scope, errors))
        {
          Some(Type::Pair(
            Box::new(lhs_type.clone()),
            Box::new(rhs_type.clone()),
          ))
        } else {
          None
        }
      }
      AssignRhs::PairElem(elem) => elem.get_type(scope, errors),
      AssignRhs::Call(id, args) => {
        /* TODO: make it never replace id in the first place,
        so we don't have to set it back again after. */
        let old_id = id.clone();

        match id.get_type(scope, errors)? {
          Type::Func(bx) => {
            /* Function idents shouldn't be renamed. */
            *id = old_id;

            let FuncSig {
              params,
              return_type,
            } = *bx;
            let mut errored = false;

            /* Must be same amount of args as parameters */
            if params.len() != args.len() {
              scope.add_error(
                errors,
                SemanticError::Normal(format!("Function called with wrong amount of arguments.")),
              );
              errored = true;
            }

            /* Types must be pairwise the same. */
            for (arg, (param_type, param_id)) in args.iter_mut().zip(params.iter()) {
              if arg
                .get_type(scope, errors)?
                .unify(param_type.clone())
                .is_none()
              {
                scope.add_error(
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
            scope.add_error(
              errors,
              SemanticError::Normal(format!(
                "TYPE ERROR:\n\tExpected: Function\n\tActual: {:?}",
                t
              )),
            );
            None
          }
        }
      }
    }
  }
}

impl HasType for PairElem {
  fn get_type(&mut self, scope: &ScopeBuilder, errors: &mut Vec<SemanticError>) -> Option<Type> {
    match self {
      PairElem::Fst(t, p) => match p.get_type(scope, errors)? {
        Type::Pair(left, _) => match *left {
          Type::Any => {
            scope.add_error(
              errors,
              SemanticError::Normal(format!("TYPE ERROR:\n\tExpected: BaseType\n\tActual: Null")),
            );
            None
          }
          new_t => {
            *t = new_t.clone();
            Some(new_t)
          }
        },
        t => {
          scope.add_error(
            errors,
            SemanticError::Normal(format!("TYPE ERROR:\n\tExpected: Pair\n\tActual:{:?}", t)),
          );
          None
        }
      },
      PairElem::Snd(t, p) => match p.get_type(scope, errors)? {
        Type::Pair(_, right) => match *right {
          Type::Any => {
            scope.add_error(
              errors,
              SemanticError::Normal(format!("TYPE ERROR:\n\tExpected: BaseType\n\tActual: Null")),
            );
            None
          }
          new_t => {
            *t = new_t.clone();
            Some(new_t)
          }
        },
        t => {
          scope.add_error(
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
  fn get_type(&mut self, scope: &ScopeBuilder, errors: &mut Vec<SemanticError>) -> Option<Type> {
    let ArrayLiter(exprs) = self;

    /* Take first element as source of truth. */
    let mut array_type = Some(Type::Any);

    /* Ensure every other element has same type. */
    for expr in exprs {
      if let Some(expr_type) = expr.get_type(scope, errors) {
        if let Some(t) = array_type {
          array_type = t.unify(expr_type)
        }
      }
    }

    Some(Type::Array(Box::new(array_type?)))
  }
}

pub fn scoped_stat(
  scope: &ScopeBuilder,
  errors: &mut Vec<SemanticError>,
  ScopedStat(new_symbol_table, statement): &mut ScopedStat,
) -> Option<ReturnBehaviour> {
  /* Create a new scope, so declarations in {statement} don't bleed into
  surrounding scope. */
  let mut new_scope = scope.new_scope(new_symbol_table);

  /* Analyse statement. */
  stat(&mut new_scope, errors, statement)
}

/* Type checks a statement.
Declarations will add to the symbol table.
Scopes will make a new scope within the symbol table.
If the statment ALWAYS returns with the same type, returns that type. */
#[allow(dead_code)]
pub fn stat(
  scope: &mut ScopeBuilder,
  errors: &mut Vec<SemanticError>,
  statement: &mut Stat,
) -> Option<ReturnBehaviour> {
  use ReturnBehaviour::*;

  /* Returns error if there is any. */
  match statement {
    Stat::Skip => Some(Never), /* Skips never return. */
    Stat::Declaration(expected, id, val) => {
      if let (Some(_), Some(new_id)) = (
        expected_type(scope, errors, &expected, val),
        /* Adds identifier to symbol table. */
        scope.insert(id, expected.clone()),
      ) {
        /* Rename ident. (global ident) */
        *id = new_id;
        Some(Never)
      } else {
        None
      }
    }
    Stat::Assignment(lhs, t, rhs) => {
      *t = equal_types(scope, errors, lhs, rhs)?;
      Some(Never) /* Assignments never return. */
    }
    Stat::Read(t, dest) => {
      /* Any type can be read. */
      match dest.get_type(scope, errors)? {
        /* Reads never return. */
        new_t @ (Type::Int | Type::Char) => {
          *t = new_t;
          Some(Never)
        }
        _ => {
          scope.add_error(
            errors,
            SemanticError::Normal(format!("Read statements must read char or int.")),
          );
          None
        } /*  */
      }
    }
    Stat::Free(t, expr) => match expr.get_type(scope, errors)? {
      new_t @ (Type::Pair(_, _) | Type::Array(_)) => {
        *t = new_t;
        Some(Never)
      } /* Frees never return. */
      actual_type => {
        scope.add_error(
          errors,
          SemanticError::Normal(format!(
            "TYPE ERROR: Expected Type\n\tExpected: Pair or Array\n\tActual:{:?}",
            actual_type
          )),
        );
        None
      }
    },
    Stat::Return(expr) => Some(AtEnd(expr.get_type(scope, errors)?)), /* Returns always return. */
    Stat::Exit(expr) => {
      /* Exit codes must be integers. */
      expected_type(scope, errors, &Type::Int, expr)?;
      /* Exits can be concidered to return because they will never return the
      wrong type, by using any it won't collide with another type. */
      Some(AtEnd(Type::Any))
    }
    Stat::Print(t, expr) | Stat::Println(t, expr) => {
      /* Any type can be printed. */
      /* Store type on print so codegen knows which print to use. */
      *t = expr.get_type(scope, errors)?;

      /* Prints never return. */
      Some(Never)
    }
    Stat::If(cond, if_stat, else_stat) => {
      let cond_fine = cond.get_type(scope, errors) == Some(Type::Bool);

      /* If both branches return the same type, the if statement can
      be relied on to return that type. */
      if let (Some(true_behaviour), Some(false_behaviour)) = (
        scoped_stat(scope, errors, if_stat),
        scoped_stat(scope, errors, else_stat),
      ) {
        if !cond_fine {
          return None;
        }

        /* If branches return with different types, if statement is error. */
        if !true_behaviour.same_return(&false_behaviour) {
          scope.add_error(
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
      let cond_fine = cond.get_type(scope, errors) == Some(Type::Bool);

      let statement_result = scoped_stat(scope, errors, body)?;

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
    Stat::Scope(body) => scoped_stat(scope, errors, body),
    Stat::Sequence(fst, snd) => {
      /* CHECK: no definite returns before last line. */
      if let (Some(lhs), Some(rhs)) = (stat(scope, errors, fst), stat(scope, errors, snd)) {
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
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    /* Identifiers cause */
    let x_id = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));
    scope.insert(&x_id, x_type.clone()).unwrap();
    assert_eq!(
      AssignLhs::Ident(x_id.clone()).get_type(scope, &mut vec![]),
      Some(x_type.clone())
    );

    assert!(
      AssignRhs::PairElem(PairElem::Fst(Type::default(), Expr::PairLiter))
        .get_type(scope, &mut vec![])
        .is_none()
    );

    assert!(
      AssignRhs::PairElem(PairElem::Fst(Type::default(), Expr::PairLiter))
        .get_type(scope, &mut vec![])
        .is_none()
    );

    assert_eq!(
      AssignLhs::ArrayElem(ArrayElem(x_id.clone(), vec!(Expr::IntLiter(5))))
        .get_type(scope, &mut vec![]),
      Some(Type::Int)
    );
  }

  #[test]
  fn declare_adds_symbol_table() {
    /*
    int x = 5
    */
    let x = || String::from("x");
    let mut intx5 = Stat::Declaration(Type::Int, x(), AssignRhs::Expr(Expr::IntLiter(5)));

    let mut outer_symbol_table = SymbolTable::default();
    let mut outer_scope = ScopeBuilder::new(&mut outer_symbol_table);

    stat(&mut outer_scope, &mut vec![], &mut intx5);

    assert!(matches!(outer_scope.get_type(&x()), Some((&Type::Int, _))));
  }

  #[test]
  fn scoping() {
    /*
    int y should be able to access the symbol table about to type check x,
    but must also allow z to be defined in that symbol table afterwards.

    begin
      int x = 5;
      begin
        int y = x
      end;
      int z = 7
    end
    */
    let x = || String::from("x");
    let y = || String::from("y");
    let z = || String::from("z");
    let intx5 = Stat::Declaration(Type::Int, x(), AssignRhs::Expr(Expr::IntLiter(5)));
    let intyx = Stat::Declaration(Type::Int, y(), AssignRhs::Expr(Expr::Ident(x())));
    let intz7 = Stat::Declaration(Type::Int, z(), AssignRhs::Expr(Expr::IntLiter(7)));
    let mut statement = Stat::Scope(ScopedStat::new(Stat::Sequence(
      Box::new(intx5),
      Box::new(Stat::Sequence(
        Box::new(Stat::Scope(ScopedStat::new(intyx))),
        Box::new(intz7),
      )),
    )));

    let mut outer_symbol_table = SymbolTable::default();
    let mut global_scope = ScopeBuilder::new(&mut outer_symbol_table);

    stat(&mut global_scope, &mut vec![], &mut statement);
    /* x and z should now be in outer scope */

    /* Retrieve inner and outer st from statement ast. */
    let (mut st, mut inner_st) = if let Stat::Scope(ScopedStat(st, statement)) = statement {
      if let Stat::Sequence(_intx5, everything_else) = *statement {
        if let Stat::Sequence(inner_scope_stat, _intz7) = *everything_else {
          if let Stat::Scope(ScopedStat(inner_st, _)) = *inner_scope_stat {
            (st, inner_st)
          } else {
            panic!("inner statement isnt a scope!")
          }
        } else {
          panic!("only two statements!");
        }
      } else {
        panic!("inner statement structure has been changed!")
      }
    } else {
      panic!("outer statement structure has been changed!")
    };

    /* When in outer scope, x and z should be ints. */
    let outer_scope = ScopeBuilder::new(&mut st);
    assert!(matches!(outer_scope.get_type(&x()), Some((&Type::Int, _))));
    assert!(matches!(outer_scope.get_type(&z()), Some((&Type::Int, _))));

    /* Check offsets are correct from outer scope. */
    assert_eq!(outer_scope.get_offset(&x()), Some(4));
    assert_eq!(outer_scope.get_offset(&z()), Some(0));

    /* When in inner scope, x, y, and z should be ints. */
    let inner_scope = outer_scope.new_scope(&mut inner_st);
    assert!(matches!(inner_scope.get_type(&x()), Some((&Type::Int, _))));
    assert!(matches!(inner_scope.get_type(&y()), Some((&Type::Int, _))));
    assert!(matches!(inner_scope.get_type(&z()), Some((&Type::Int, _))));

    /* x and z's offsets should be offset by 4 more now because y is using 4 bytes. */
    assert_eq!(inner_scope.get_offset(&x()), Some(8));
    assert_eq!(inner_scope.get_offset(&z()), Some(4));

    /* y should now have +0 offset */
    assert_eq!(inner_scope.get_offset(&y()), Some(0));
  }
}
