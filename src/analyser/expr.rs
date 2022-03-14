use super::*;
use crate::ast::*;
use Expr::*;

impl Analysable for TypedExpr {
  type Input = ();
  type Output = Type;
  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    let TypedExpr(t, expr) = self;
    *t = expr.analyse(scope, ())?;
    Ok(t.clone())
  }
}

impl Analysable for Expr {
  type Input = ();
  type Output = Type;
  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<Type> {
    match self {
      IntLiter(_) => Ok(Type::Int),
      BoolLiter(_) => Ok(Type::Bool),
      CharLiter(_) => Ok(Type::Char),
      StrLiter(_) => Ok(Type::String),
      NullPairLiter => Ok(Type::Pair(Box::new(Type::Any), Box::new(Type::Any))),
      PairLiter(e1, e2) => {
        let (lhs_type, rhs_type) =
          e1.analyse(scope, ()).join(e2.analyse(scope, ()))?;

        Ok(Type::Pair(Box::new(lhs_type), Box::new(rhs_type)))
      }
      Expr::ArrayLiter(lit) => lit.analyse(scope, ()),
      StructLiter(s) => s.analyse(scope, ()),
      PairElem(elem) => elem.analyse(scope, ()),
      Ident(id) => id.analyse(scope, ()),
      Expr::ArrayElem(elem) => elem.analyse(scope, ()),
      Expr::StructElem(elem) => elem.analyse(scope, ()),
      Call(t, func_expr, args) => analyse_call(scope, t, func_expr, args),
      UnaryApp(op, exp) => analyse_unary(scope, op, exp),
      BinaryApp(exp1, op, exp2) => analyse_binary(scope, exp1, op, exp2),
      Expr::AnonFunc(func) => {
        (**func).analyse(scope, ())?;
        Ok(Type::Func(Box::new(func.signature.clone())))
      }
    }
  }
}

fn analyse_call(
  scope: &mut ScopeBuilder,
  t: &mut Type,
  func_expr: &mut Box<Expr>,
  args: &mut Vec<Expr>,
) -> AResult<Type> {
  match func_expr.analyse(scope, ())? {
    Type::Func(bx) => {
      /* Populate the type in call. */
      *t = Type::Func(bx.clone());

      let FuncSig {
        param_types,
        return_type,
      } = *bx;

      /* Types must be pairwise the same. */
      SemanticError::join_iter(args.iter_mut().zip(param_types.iter()).map(
        |(arg, param_type)| expected_type(scope, &mut param_type.clone(), arg),
      ))?;

      /* Must be same amount of args as parameters */
      if param_types.len() != args.len() {
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

fn analyse_unary(
  scope: &mut ScopeBuilder,
  op: &mut UnaryOper,
  exp: &mut Box<Expr>,
) -> AResult<Type> {
  match op {
    UnaryOper::Bang => Ok(expected_type(scope, &mut Type::Bool, exp)?.clone()),
    UnaryOper::Neg => Ok(expected_type(scope, &mut Type::Int, exp)?.clone()),
    UnaryOper::Len => match exp.analyse(scope, ())? {
      Type::Array(_) => Ok(Type::Int),
      t => {
        Err(SemanticError::Normal(format!(
          "TYPE ERROR: Attempt to find length of non array\n\tExpected: Array\n\tActual: {:?}",
          t
        )))
      }
    },
    UnaryOper::Ord => {
      expected_type(scope, &mut Type::Char, exp)?;
      Ok(Type::Int)
    }
    UnaryOper::Chr => {
      expected_type (scope, &mut Type::Int, exp)?;
      Ok(Type::Char)
    }
  }
}

fn analyse_binary(
  scope: &mut ScopeBuilder,
  exp1: &mut Box<Expr>,
  op: &mut BinaryOper,
  exp2: &mut Box<Expr>,
) -> AResult<Type> {
  /* Every binary application requires both expressions to have the same type. */
  let expr_type = equal_types(scope, exp1, exp2)?;

  match op {
    /* Maths can only be done on ints. */
    BinaryOper::Mul
    | BinaryOper::Div
    | BinaryOper::Mod
    | BinaryOper::Add
    | BinaryOper::Sub => match expr_type {
      Type::Int => Ok(Type::Int),
      t => {
        return Err(SemanticError::Normal(format!(
          "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
          op, t
        )))
      }
    },
    /* Any types can be compared. */
    BinaryOper::Gt | BinaryOper::Gte | BinaryOper::Lt | BinaryOper::Lte => match expr_type {
      Type::Int | Type::Char => Ok(Type::Bool),
      t => {
        return Err(SemanticError::Normal(format!(
          "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
          op, t
        )))
      }
    },
    BinaryOper::Eq | BinaryOper::Neq => Ok(Type::Bool),
    /* Boolean operators can only be applied to booleans. */
    BinaryOper::And | BinaryOper::Or => match expr_type {
      Type::Bool => Ok(Type::Bool),
      t => {
        return Err(SemanticError::Normal(format!(
          "TYPE ERROR: Unsupported type for {:?}\n\tExpected: Int\n\tActual: {:?}",
          op, t
        )))
      }
    },
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
      let mut ident = format!("{}{}", prefix, i);
      let t = Type::Int;
      scope.insert_var(&mut ident, t).unwrap();
    }
  }

  #[test]
  fn pair_elems() {
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    assert!(Expr::PairElem(Box::new(PairElem::Fst(TypedExpr::new(
      Expr::NullPairLiter
    ))))
    .analyse(scope, ())
    .is_err());

    assert!(Expr::PairElem(Box::new(PairElem::Fst(TypedExpr::new(
      Expr::NullPairLiter
    ))))
    .analyse(scope, ())
    .is_err());
  }

  #[test]
  fn literals() {
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    assert_eq!(IntLiter(5).analyse(scope, ()), Ok(Type::Int));
    assert_eq!(BoolLiter(false).analyse(scope, ()), Ok(Type::Bool));
    assert_eq!(CharLiter('a').analyse(scope, ()), Ok(Type::Char));
    assert_eq!(
      NullPairLiter.analyse(scope, ()),
      Ok(Type::Pair(Box::new(Type::Any), Box::new(Type::Any))),
    );
  }

  #[test]
  fn idents() {
    let mut symbol_table = SymbolTable::default();
    let mut scope = ScopeBuilder::new(&mut symbol_table);
    populate_scope(&mut scope, "var");

    assert_eq!(
      Ident(String::from("var1")).analyse(&mut scope, ()),
      Ok(Type::Int),
    );
  }

  #[test]
  fn array_elems() {
    let x = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));

    let mut symbol_table = SymbolTable::default();
    let mut scope = ScopeBuilder::new(&mut symbol_table);
    scope.insert_var(&mut x.clone(), x_type).unwrap();

    assert_eq!(
      Expr::ArrayElem(ArrayElem(x, vec!(Expr::IntLiter(5))))
        .analyse(&mut scope, ()),
      Ok(Type::Int)
    );
  }

  #[test]
  fn unary_apps() {
    /* Symbol Table */
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    /* BANG */
    /* !false: Bool */
    assert_eq!(
      UnaryApp(UnaryOper::Bang, Box::new(BoolLiter(false))).analyse(scope, ()),
      Ok(Type::Bool)
    );

    /* !'a': ERROR */
    assert!(UnaryApp(UnaryOper::Bang, Box::new(CharLiter('a')))
      .analyse(scope, ())
      .is_err());

    /* NEG */
    /* -5: Int */
    assert_eq!(
      UnaryApp(UnaryOper::Neg, Box::new(IntLiter(5))).analyse(scope, ()),
      Ok(Type::Int)
    );

    /* -false: ERROR */
    assert!(UnaryApp(UnaryOper::Neg, Box::new(BoolLiter(false)))
      .analyse(scope, ())
      .is_err());

    /* LEN */
    /* len [1,2,3]: Int */
    let x = String::from("x");
    let x_type = Type::Array(Box::new(Type::Int));
    scope.insert_var(&mut x.clone(), x_type).unwrap();
    assert_eq!(
      UnaryApp(UnaryOper::Len, Box::new(Ident(x))).analyse(scope, ()),
      Ok(Type::Int)
    );

    /* len 5: ERROR */
    assert!(UnaryApp(UnaryOper::Len, Box::new(IntLiter(5)))
      .analyse(scope, ())
      .is_err());

    /* ORD */
    /* ord 'a': Int */
    assert_eq!(
      UnaryApp(UnaryOper::Ord, Box::new(CharLiter('a'))).analyse(scope, ()),
      Ok(Type::Int)
    );

    /* ord 5: ERROR */
    assert!(UnaryApp(UnaryOper::Ord, Box::new(IntLiter(5)))
      .analyse(scope, ())
      .is_err());

    /* CHR */
    /* chr 5: Char */
    assert_eq!(
      UnaryApp(UnaryOper::Chr, Box::new(IntLiter(5))).analyse(scope, ()),
      Ok(Type::Char)
    );

    /* chr 'a': ERROR */
    assert!(UnaryApp(UnaryOper::Chr, Box::new(CharLiter('a')))
      .analyse(scope, ())
      .is_err());
  }

  #[test]
  fn binary_apps() {
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    /* 5 + false: ERROR */
    assert!(BinaryApp(
      Box::new(IntLiter(5)),
      BinaryOper::Add,
      Box::new(BoolLiter(false))
    )
    .analyse(scope, ())
    .is_err());

    /* 5 * 'a': ERROR */
    assert!(BinaryApp(
      Box::new(IntLiter(5)),
      BinaryOper::Mul,
      Box::new(CharLiter('a'))
    )
    .analyse(scope, ())
    .is_err());

    /* false / "hello": ERROR */
    assert!(BinaryApp(
      Box::new(BoolLiter(false)),
      BinaryOper::Div,
      Box::new(StrLiter(String::from("hello")))
    )
    .analyse(scope, ())
    .is_err());

    /* false && 6: ERROR */
    assert!(BinaryApp(
      Box::new(BoolLiter(false)),
      BinaryOper::And,
      Box::new(IntLiter(6))
    )
    .analyse(scope, ())
    .is_err());

    /* MATH CAN BE DONE ON INTS. */
    /* 5 * 5: Int */
    assert_eq!(
      BinaryApp(
        Box::new(IntLiter(5)),
        BinaryOper::Mul,
        Box::new(IntLiter(5)),
      )
      .analyse(scope, ()),
      Ok(Type::Int),
    );

    /* 5 + 5: Int */
    assert_eq!(
      BinaryApp(
        Box::new(IntLiter(5)),
        BinaryOper::Add,
        Box::new(IntLiter(5)),
      )
      .analyse(scope, ()),
      Ok(Type::Int),
    );

    /* MATH CANT BE DONE ON ANYTHING ELSE */
    /* 'a' + 'b': ERROR */
    assert!(BinaryApp(
      Box::new(CharLiter('a')),
      BinaryOper::Add,
      Box::new(CharLiter('b'))
    )
    .analyse(scope, ())
    .is_err());

    /* false + false: ERROR */
    assert!(BinaryApp(
      Box::new(BoolLiter(false)),
      BinaryOper::Add,
      Box::new(BoolLiter(false))
    )
    .analyse(scope, ())
    .is_err());

    /* Any type can be comapred to itself. */
    for expr in vec![
      IntLiter(5),
      BoolLiter(false),
      StrLiter(String::from("hello")),
      NullPairLiter,
      UnaryApp(UnaryOper::Neg, Box::new(IntLiter(5))),
      BinaryApp(
        Box::new(StrLiter(String::from("hello world"))),
        BinaryOper::Eq,
        Box::new(StrLiter(String::from("hello steve"))),
      ),
    ] {
      for oper in vec![BinaryOper::Eq, BinaryOper::Neq] {
        assert_eq!(
          BinaryApp(Box::new(expr.clone()), oper, Box::new(expr.clone()))
            .analyse(scope, ()),
          Ok(Type::Bool)
        );
      }
    }

    for expr in vec![IntLiter(5), CharLiter('a')] {
      for oper in vec![
        BinaryOper::Gt,
        BinaryOper::Gte,
        BinaryOper::Lt,
        BinaryOper::Lte,
      ] {
        assert_eq!(
          BinaryApp(Box::new(expr.clone()), oper, Box::new(expr.clone()))
            .analyse(scope, ()),
          Ok(Type::Bool)
        );
      }
    }

    /* Boolean logic can only be applied to booleans */
    /* 5 && 5: ERROR */
    assert!(BinaryApp(
      Box::new(IntLiter(5)),
      BinaryOper::And,
      Box::new(IntLiter(5)),
    )
    .analyse(scope, ())
    .is_err());

    /* 'a' || 'a': ERROR */
    assert!(BinaryApp(
      Box::new(CharLiter('a')),
      BinaryOper::Or,
      Box::new(CharLiter('a')),
    )
    .analyse(scope, ())
    .is_err());

    /* true && true: bool */
    assert_eq!(
      BinaryApp(
        Box::new(BoolLiter(true)),
        BinaryOper::And,
        Box::new(BoolLiter(true)),
      )
      .analyse(scope, ()),
      Ok(Type::Bool)
    );
  }
}
