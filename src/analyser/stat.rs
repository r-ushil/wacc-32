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

impl Analysable for AssignLhs {
  type Input = ();
  type Output = Type;

  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    match self {
      AssignLhs::Ident(id) => id.analyse(scope, ()),
      AssignLhs::ArrayElem(elem) => elem.analyse(scope, ()),
      AssignLhs::PairElem(elem) => elem.analyse(scope, ()),
      AssignLhs::StructElem(elem) => elem.analyse(scope, ()),
    }
  }
}

impl Analysable for AssignRhs {
  type Input = ();
  type Output = Type;

  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    match self {
      AssignRhs::Expr(exp) => exp.analyse(scope, ()),
    }
  }
}

impl Analysable for StructLiter {
  type Input = ();
  type Output = Type;
  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    let StructLiter { id, fields } = self;

    /* Fetch struct definition. */
    let def = match scope.get_def(id) {
      Some(def) => def,
      None => {
        return Err(SemanticError::Normal(format!(
          "Cannot find type: {:?}",
          id
        )))
      }
    };

    /* Make sure defition and usage have same number of fields. */
    if fields.len() != def.fields.len() {
      return Err(SemanticError::Normal(format!(
        "Struct literal has different amount of errors to definition."
      )));
    }

    /* Check all fields evaluate to correct type. */
    for (field_name, (field_type, _)) in def.fields.iter() {
      /* Get expression this field is to be set to. */
      let liter_expr = match self.fields.get_mut(field_name) {
        Some(expr) => expr,
        None => {
          return Err(SemanticError::Normal(format!(
            "Struct literal missing expression for field: {}",
            field_name
          )))
        }
      };

      /* Assert it's type is the same as the field is expecting. */
      expected_type(scope, field_type, liter_expr)?;
    }

    Ok(Type::Custom(id.clone()))
  }
}

impl Analysable for PairElem {
  type Input = ();
  type Output = Type;
  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    use PairElem::*;

    /* Gets type of thing being accessed, and stored type. */
    let pair_type = match self {
      Fst(p) | Snd(p) => p.analyse(scope, ())?,
    };

    /* Gets type of left and right element of pair. */
    let (left_type, right_type) = match pair_type {
      Type::Pair(lt, rt) => (*lt, *rt),
      t => {
        return Err(SemanticError::Normal(format!(
          "TYPE ERROR:\n\tExpected: Pair\n\tActual:{:?}",
          t
        )))
      }
    };

    /* EDGE CASE: compiler must reject "fst null", "snd null".
    (Which makes no sense because in the rest of the compiler there is
    no static checking for nulls) >:( */
    if left_type == Type::Any && right_type == Type::Any {
      return Err(SemanticError::Normal(format!(
        "Cannot access element of null-literal"
      )));
    }

    /* Gets the type of the element being accessed. */
    let elem_type = match self {
      Fst(_) => left_type,
      Snd(_) => right_type,
    };

    let stored_type = match self {
      Fst(TypedExpr(st, _)) | Snd(TypedExpr(st, _)) => st,
    };

    /* Stores this on the AST. */
    *stored_type = elem_type.clone();

    Ok(elem_type.clone())
  }
}

impl Analysable for ArrayLiter {
  type Input = ();
  type Output = Type;

  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    let ArrayLiter(stored_type, exprs) = self;

    /* Take first element as source of truth. */
    let mut array_type = Some(Type::Any);

    /* Ensure every other element has same type. */
    for expr in exprs {
      if let Ok(expr_type) = expr.analyse(scope, ()) {
        if let Some(t) = array_type {
          array_type = t.unify(expr_type)
        }
      }
    }

    match array_type {
      Some(t) => {
        *stored_type = t.clone();
        Ok(Type::Array(Box::new(t)))
      }
      None => Err(SemanticError::Normal(format!(
        "Expressions making up array literal have inconsistent types."
      ))),
    }
  }
}

impl Analysable for ScopedStat {
  type Input = ();

  type Output = ReturnBehaviour;

  fn analyse(
    &mut self,
    scope: &mut ScopeBuilder,
    _: (),
  ) -> AResult<ReturnBehaviour> {
    let ScopedStat(new_symbol_table, statement) = self;

    /* Create a new scope, so declarations in {statement} don't bleed into
    surrounding scope. */
    let mut new_scope = scope.new_scope(new_symbol_table);

    /* Analyse statement. */
    statement.analyse(&mut new_scope, ())
  }
}

impl Analysable for Stat {
  type Input = ();
  type Output = ReturnBehaviour;

  /* Type checks a statement.
  Declarations will add to the symbol table.
  Scopes will make a new scope within the symbol table.
  If the statment ALWAYS returns with the same type, returns that type. */
  fn analyse(
    &mut self,
    scope: &mut ScopeBuilder,
    _: (),
  ) -> AResult<ReturnBehaviour> {
    use ReturnBehaviour::*;

    /* Returns error if there is any. */
    match self {
      Stat::Skip => Ok(Never), /* Skips never return. */
      Stat::Declaration(expected, id, val) => {
        expected_type(scope, expected, val)
          .join(scope.insert_var(id, expected.clone()))?;

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
        match dest.analyse(scope, ())? {
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
      Stat::Free(expr) => match expr.analyse(scope, ())? {
        Type::Pair(_, _) | Type::Array(_) => Ok(Never), /* Frees never return. */
        actual_type => Err(SemanticError::Normal(format!(
          "TYPE ERROR: Expected Type\n\tExpected: Pair or Array\n\tActual:{:?}",
          actual_type
        ))),
      },
      Stat::Return(expr) => Ok(AtEnd(expr.analyse(scope, ())?)), /* Returns always return. */
      Stat::Exit(expr) => {
        /* Exit codes must be integers. */
        expected_type(scope, &Type::Int, expr)?;
        /* Exits can be concidered to return because they will never return the
        wrong type, by using any it won't collide with another type. */
        Ok(AtEnd(Type::Any))
      }
      Stat::Print(expr) | Stat::Println(expr) => {
        /* Any type can be printed. */
        expr.analyse(scope, ())?;

        /* Prints never return. */
        Ok(Never)
      }
      Stat::If(cond, if_stat, else_stat) => {
        let ((_, true_behaviour), false_behaviour) =
          expected_type(scope, &Type::Bool, cond)
            .join(if_stat.analyse(scope, ()))
            .join(else_stat.analyse(scope, ()))?;

        /* If both branches return the same type, the if statement can
        be relied on to return that type. */

        /* If branches return with different types, if statement is error. */
        if !true_behaviour.same_return(&false_behaviour) {
          return Err(SemanticError::Normal(
            "Branches of if statement return values of different types."
              .to_string(),
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
        let (_, statement_result) = expected_type(scope, &Type::Bool, cond)
          .join(body.analyse(scope, ()))?;

        Ok(match statement_result {
          /* If the body always returns, while loop might still not return
          because the cond might always be false and the body never run. */
          AtEnd(t) => MidWay(t),
          /* Otherwise white loop returns the same way it's body does. */
          b => b,
        })
      }
      Stat::Scope(body) => body.analyse(scope, ()),
      Stat::Sequence(fst, snd) => {
        /* CHECK: no definite returns before last line. */
        let (lhs, rhs) = fst.analyse(scope, ()).join(snd.analyse(scope, ()))?;

        /* Even if RHS never returns, the statement overall might still return
        if the LHS returns some or all of the time. */
        Ok(if let (Never, MidWay(t) | AtEnd(t)) = (&rhs, lhs) {
          MidWay(t)
        } else {
          rhs
        })
      }
      Stat::For(decl, cond, body, assign) => {
        //match decl is a declaration and assign is an assignment, ** for the boxes
        //need to add decl variable to scope

        match (**decl).clone() {
          Stat::Skip => (),
          Stat::Declaration(t, id, _rhs) => {
            decl.analyse(scope, ())?;
            scope.insert_var(&mut id.clone(), t.clone());
          }
          _ => panic!("First part of for loop not a declaration or a skip"),
        };

        match **assign {
          Stat::Assignment(_, _, _) => {
            //check everything type checks (recursively using analyse)
            let (_, stat_res) = expected_type(scope, &Type::Bool, cond)
              .join(body.analyse(scope, ()))
              .join(assign.analyse(scope, ()))?;

            //similar reasoning to while loop
            Ok(match stat_res {
              AtEnd(t) => MidWay(t),
              b => b,
            })
          }
          _ => panic!("Last part of for loop not an assignment"),
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::analyser::context::SymbolTable;

  use super::*;

  #[test]
  fn struct_liter() {
    let mut symbol_table = SymbolTable::default();
    symbol_table.table.insert(
      format!("IntBox"),
      IdentInfo::TypeDef(Struct {
        fields: HashMap::from([(format!("x"), (Type::Int, 0))]),
        size: 4,
      }),
    );
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    /* Correct usage. */
    assert!((StructLiter {
      id: format!("IntBox"),
      fields: HashMap::from([(format!("x"), Expr::IntLiter(5))]),
    })
    .analyse(scope, ())
    .is_ok());

    /* Wrong amount of fields. */
    assert!((StructLiter {
      id: format!("IntBox"),
      fields: HashMap::from([
        (format!("x"), Expr::IntLiter(5)),
        (format!("y"), Expr::IntLiter(6)),
      ])
    })
    .analyse(scope, ())
    .is_err());

    /* Field has wrong type. */
    assert!((StructLiter {
      id: format!("IntBox"),
      fields: HashMap::from([(format!("x"), Expr::BoolLiter(true)),])
    })
    .analyse(scope, ())
    .is_err());
  }

  #[test]
  fn assign_lhs() {
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    /* Identifiers cause */
    let x_id = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));
    scope.insert_var(&mut x_id.clone(), x_type.clone()).unwrap();
    assert_eq!(
      AssignLhs::Ident(x_id.clone()).analyse(scope, ()),
      Ok(x_type)
    );

    assert_eq!(
      AssignLhs::ArrayElem(ArrayElem(x_id.clone(), vec!(Expr::IntLiter(5))))
        .analyse(scope, ()),
      Ok(Type::Int)
    );
  }

  #[test]
  fn declare_adds_symbol_table() {
    /*
    int x = 5
    */
    let x = || String::from("x");
    let mut intx5 =
      Stat::Declaration(Type::Int, x(), AssignRhs::Expr(Expr::IntLiter(5)));

    let mut outer_symbol_table = SymbolTable::default();
    let mut outer_scope = ScopeBuilder::new(&mut outer_symbol_table);

    intx5.analyse(&mut outer_scope, ()).unwrap();

    assert!(matches!(
      outer_scope.get(&mut x()),
      Some(IdentInfo::LocalVar(Type::Int, _))
    ));
  }

  #[test]
  fn scoping() {
    use IdentInfo::*;
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
    let intx5 =
      Stat::Declaration(Type::Int, x(), AssignRhs::Expr(Expr::IntLiter(5)));
    let intyx =
      Stat::Declaration(Type::Int, y(), AssignRhs::Expr(Expr::Ident(x())));
    let intz7 =
      Stat::Declaration(Type::Int, z(), AssignRhs::Expr(Expr::IntLiter(7)));
    let mut statement = Stat::Scope(ScopedStat::new(Stat::Sequence(
      Box::new(intx5),
      Box::new(Stat::Sequence(
        Box::new(Stat::Scope(ScopedStat::new(intyx))),
        Box::new(intz7),
      )),
    )));

    let mut outer_symbol_table = SymbolTable::default();
    let mut global_scope = ScopeBuilder::new(&mut outer_symbol_table);

    statement.analyse(&mut global_scope, ()).unwrap();
    /* x and z should now be in outer scope */

    /* Retrieve inner and outer st from statement ast. */
    let (mut st, mut inner_st) =
      if let Stat::Scope(ScopedStat(st, statement)) = statement {
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
    assert!(matches!(
      outer_scope.get(&mut x()),
      Some(LocalVar(Type::Int, _))
    ));
    assert!(matches!(
      outer_scope.get(&mut z()),
      Some(LocalVar(Type::Int, _))
    ));

    /* Check offsets are correct from outer scope. */
    assert!(matches!(outer_scope.get(&mut x()), Some(LocalVar(_, 4))));
    assert!(matches!(outer_scope.get(&mut z()), Some(LocalVar(_, 0))));

    /* When in inner scope, x, y, and z should be ints. */
    let inner_scope = outer_scope.new_scope(&mut inner_st);
    assert!(matches!(
      inner_scope.get(&mut x()),
      Some(LocalVar(Type::Int, _))
    ));
    assert!(matches!(
      inner_scope.get(&mut y()),
      Some(LocalVar(Type::Int, _))
    ));
    assert!(matches!(
      inner_scope.get(&mut z()),
      Some(LocalVar(Type::Int, _))
    ));

    /* x and z's offsets should be offset by 4 more now because y is using 4 bytes. */
    assert!(matches!(inner_scope.get(&mut x()), Some(LocalVar(_, 8))));
    assert!(matches!(inner_scope.get(&mut z()), Some(LocalVar(_, 4))));

    /* y should now have +0 offset */
    assert!(matches!(inner_scope.get(&mut y()), Some(LocalVar(_, 0))));
  }
}
