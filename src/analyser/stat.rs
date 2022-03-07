use super::*;
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
  fn get_type(&mut self, scope: &ScopeBuilder) -> AResult<Type> {
    match self {
      AssignLhs::Ident(id) => id.get_type(scope),
      AssignLhs::ArrayElem(elem) => elem.get_type(scope),
      AssignLhs::PairElem(elem) => elem.get_type(scope),
    }
  }
}

#[allow(unused_variables)]
impl HasType for AssignRhs {
  fn get_type(&mut self, scope: &ScopeBuilder) -> AResult<Type> {
    match self {
      AssignRhs::Expr(exp) => exp.get_type(scope),
      AssignRhs::ArrayLiter(lit) => lit.get_type(scope),
      AssignRhs::Pair(e1, e2) => {
        let (lhs_type, rhs_type) = e1.get_type(scope).join(e2.get_type(scope))?;

        Ok(Type::Pair(Box::new(lhs_type), Box::new(rhs_type)))
      }
      AssignRhs::PairElem(elem) => elem.get_type(scope),
      AssignRhs::Call(id, args) => {
        /* TODO: make it never replace id in the first place,
        so we don't have to set it back again after. */
        let old_id = id.clone();

        match id.get_type(scope)? {
          Type::Func(bx) => {
            /* Function idents shouldn't be renamed. */
            *id = old_id;

            let FuncSig {
              params,
              return_type,
            } = *bx;

            /* Types must be pairwise the same. */
            SemanticError::join_iter(
              args
                .iter_mut()
                .zip(params.iter())
                .map(|(arg, (param_type, param_id))| expected_type(scope, param_type, arg)),
            )?;

            /* Must be same amount of args as parameters */
            if params.len() != args.len() {
              Err(SemanticError::Normal(
                "Function called with wrong amount of arguments.".to_string(),
              ))
            } else {
              Ok(return_type)
            }
          }
          t => Err(SemanticError::Normal(format!(
            "TYPE ERROR:\n\tExpected: Function\n\tActual: {:?}",
            t
          ))),
        }
      }
    }
  }
}

impl HasType for PairElem {
  fn get_type(&mut self, scope: &ScopeBuilder) -> AResult<Type> {
    match self {
      PairElem::Fst(t, p) => match p.get_type(scope)? {
        Type::Pair(left, _) => match *left {
          Type::Any => Err(SemanticError::Normal(
            "TYPE ERROR:\n\tExpected: BaseType\n\tActual: Null".to_string(),
          )),
          new_t => {
            *t = new_t.clone();
            Ok(new_t)
          }
        },
        t => Err(SemanticError::Normal(format!(
          "TYPE ERROR:\n\tExpected: Pair\n\tActual:{:?}",
          t
        ))),
      },
      PairElem::Snd(t, p) => match p.get_type(scope)? {
        Type::Pair(_, right) => match *right {
          Type::Any => Err(SemanticError::Normal(
            "TYPE ERROR:\n\tExpected: BaseType\n\tActual: Null".to_string(),
          )),
          new_t => {
            *t = new_t.clone();
            Ok(new_t)
          }
        },
        t => Err(SemanticError::Normal(format!(
          "TYPE ERROR:\n\tExpected: Pair\n\tActual:{:?}",
          t
        ))),
      },
    }
  }
}

impl HasType for ArrayLiter {
  fn get_type(&mut self, scope: &ScopeBuilder) -> AResult<Type> {
    let ArrayLiter(exprs) = self;

    /* Take first element as source of truth. */
    let mut array_type = Some(Type::Any);

    /* Ensure every other element has same type. */
    for expr in exprs {
      if let Ok(expr_type) = expr.get_type(scope) {
        if let Some(t) = array_type {
          array_type = t.unify(expr_type)
        }
      }
    }

    match array_type {
      Some(t) => Ok(Type::Array(Box::new(t))),
      None => Err(SemanticError::Normal(format!(
        "Expressions making up array literal have inconsistent types."
      ))),
    }
  }
}

pub fn scoped_stat(
  scope: &ScopeBuilder,
  ScopedStat(new_symbol_table, statement): &mut ScopedStat,
) -> AResult<ReturnBehaviour> {
  /* Create a new scope, so declarations in {statement} don't bleed into
  surrounding scope. */
  let mut new_scope = scope.new_scope(new_symbol_table);

  /* Analyse statement. */
  stat(&mut new_scope, statement)
}

/* Type checks a statement.
Declarations will add to the symbol table.
Scopes will make a new scope within the symbol table.
If the statment ALWAYS returns with the same type, returns that type. */
#[allow(dead_code)]
pub fn stat(scope: &mut ScopeBuilder, statement: &mut Stat) -> AResult<ReturnBehaviour> {
  use ReturnBehaviour::*;

  /* Returns error if there is any. */
  match statement {
    Stat::Skip => Ok(Never), /* Skips never return. */
    Stat::Declaration(expected, id, val) => {
      let (_, new_id) =
        expected_type(scope, expected, val).join(scope.insert(id, expected.clone()))?;

      /* Rename ident. (global ident) */
      *id = new_id;

      /* Declarations never return. */
      Ok(Never)
    }
    Stat::Assignment(lhs, t, rhs) => {
      /* LHS and RHS must have same type. */
      *t = equal_types(scope, lhs, rhs)?;

      /* Assignments never return. */
      Ok(Never)
    }
    Stat::Read(t, dest) => {
      /* Any type can be read. */
      match dest.get_type(scope)? {
        /* Reads never return. */
        new_t @ (Type::Int | Type::Char) => {
          *t = new_t;
          Ok(Never)
        }
        _ => Err(SemanticError::Normal(
          "Read statements must read char or int.".to_string(),
        )), /*  */
      }
    }
    Stat::Free(t, expr) => match expr.get_type(scope)? {
      new_t @ (Type::Pair(_, _) | Type::Array(_)) => {
        *t = new_t;
        Ok(Never)
      } /* Frees never return. */
      actual_type => Err(SemanticError::Normal(format!(
        "TYPE ERROR: Expected Type\n\tExpected: Pair or Array\n\tActual:{:?}",
        actual_type
      ))),
    },
    Stat::Return(expr) => Ok(AtEnd(expr.get_type(scope)?)), /* Returns always return. */
    Stat::Exit(expr) => {
      /* Exit codes must be integers. */
      expected_type(scope, &Type::Int, expr)?;
      /* Exits can be concidered to return because they will never return the
      wrong type, by using any it won't collide with another type. */
      Ok(AtEnd(Type::Any))
    }
    Stat::Print(t, expr) | Stat::Println(t, expr) => {
      /* Any type can be printed. */
      /* Store type on print so codegen knows which print to use. */
      *t = expr.get_type(scope)?;

      /* Prints never return. */
      Ok(Never)
    }
    Stat::If(cond, if_stat, else_stat) => {
      let ((_, true_behaviour), false_behaviour) = expected_type(scope, &Type::Bool, cond)
        .join(scoped_stat(scope, if_stat))
        .join(scoped_stat(scope, else_stat))?;

      /* If both branches return the same type, the if statement can
      be relied on to return that type. */

      /* If branches return with different types, if statement is error. */
      if !true_behaviour.same_return(&false_behaviour) {
        return Err(SemanticError::Normal(
          "Branches of if statement return values of different types.".to_string(),
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
    }
    Stat::While(cond, body) => {
      let (_, statement_result) =
        expected_type(scope, &Type::Bool, cond).join(scoped_stat(scope, body))?;

      Ok(match statement_result {
        /* If the body always returns, while loop might still not return
        because the cond might always be false and the body never run. */
        AtEnd(t) => MidWay(t),
        /* Otherwise white loop returns the same way it's body does. */
        b => b,
      })
    }
    Stat::Scope(body) => scoped_stat(scope, body),
    Stat::Sequence(fst, snd) => {
      /* CHECK: no definite returns before last line. */
      let (lhs, rhs) = stat(scope, fst).join(stat(scope, snd))?;

      /* Even if RHS never returns, the statement overall might still return
      if the LHS returns some or all of the time. */
      Ok(if let (Never, MidWay(t) | AtEnd(t)) = (&rhs, lhs) {
        MidWay(t)
      } else {
        rhs
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::analyser::context::SymbolTable;

  use super::*;

  #[test]
  fn assign_lhs() {
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    /* Identifiers cause */
    let x_id = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));
    scope.insert(&x_id, x_type.clone()).unwrap();
    assert_eq!(AssignLhs::Ident(x_id.clone()).get_type(scope), Ok(x_type));

    assert!(
      AssignRhs::PairElem(PairElem::Fst(Type::default(), Expr::PairLiter))
        .get_type(scope)
        .is_ok()
    );

    assert!(
      AssignRhs::PairElem(PairElem::Fst(Type::default(), Expr::PairLiter))
        .get_type(scope)
        .is_ok()
    );

    assert_eq!(
      AssignLhs::ArrayElem(ArrayElem(x_id.clone(), vec!(Expr::IntLiter(5)))).get_type(scope),
      Ok(Type::Int)
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

    stat(&mut outer_scope, &mut intx5);

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

    stat(&mut global_scope, &mut statement);
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
