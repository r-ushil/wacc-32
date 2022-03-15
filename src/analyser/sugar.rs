/* Place to put functions which desugar ast nodes into other ast nodes. */
use super::*;

pub fn array_assignment(
  scope: &mut ScopeBuilder,
  ArrayLiter(_, dst_exprs): &mut ArrayLiter,
  src: &mut Expr,
) -> AResult<Stat> {
  if dst_exprs.len() == 0 {
    return Err(SemanticError::Normal(format!(
      "Cannot destructure into empty array."
    )));
  }

  /* Use first element to determine the element type. */
  let elem_type = dst_exprs[0].clone().analyse(scope, ExprPerms::Nothing)?;

  let tmp_val = Expr::Ident(scope.get_unique());

  /* Store the whole array in a temporary variable. */
  let mut new_stat = Stat::Declaration(
    Type::Array(Box::new(elem_type.clone())),
    tmp_val.clone(),
    src.clone(),
  );

  /* Assign a value of the array to each destination expression. */
  for (i, dst_expr) in dst_exprs.iter().enumerate() {
    /* Writes the ith value of the temp value to this destination expression. */
    let assignment = Stat::Assignment(
      dst_expr.clone(),
      Type::default(),
      Expr::ArrayElem(
        Type::default(),
        Box::new(tmp_val.clone()),
        Box::new(Expr::IntLiter(i as i32)),
      ),
    );

    /* Put it after new stat. */
    new_stat = Stat::Sequence(Box::new(new_stat), Box::new(assignment));
  }

  Ok(new_stat)
}

/* Turns an assignment to a pair into a declaration */
pub fn pair_declaration(
  scope: &mut ScopeBuilder,
  lhs_expr: Expr,
  lhs_type: Type,
  rhs_expr: Expr,
  rhs_type: Type,
  src: Expr,
) -> AResult<Stat> {
  let tmp_val = Expr::Ident(scope.get_unique());

  Ok(Stat::Sequence(
    Box::new(Stat::Declaration(
      Type::Pair(Box::new(lhs_type.clone()), Box::new(rhs_type.clone())),
      tmp_val.clone(),
      src.clone(),
    )),
    Box::new(Stat::Sequence(
      Box::new(Stat::Declaration(
        lhs_type,
        lhs_expr,
        Expr::PairElem(Box::new(PairElem::Fst(TypedExpr(
          Type::default(),
          tmp_val.clone(),
        )))),
      )),
      Box::new(Stat::Declaration(
        rhs_type,
        rhs_expr,
        Expr::PairElem(Box::new(PairElem::Snd(TypedExpr(
          Type::default(),
          tmp_val.clone(),
        )))),
      )),
    )),
  ))
}

/* Turns an assignment to a pair into a declaration */
pub fn pair_assignment(
  scope: &mut ScopeBuilder,
  lhs_expr: Expr,
  rhs_expr: Expr,
  src: Expr,
) -> AResult<Stat> {
  let tmp_val = Expr::Ident(scope.get_unique());

  Ok(Stat::Sequence(
    /* pair(T, U) @_1 =  */
    Box::new(Stat::Declaration(
      Type::Pair(
        Box::new(lhs_expr.clone().analyse(scope, ExprPerms::Nothing)?),
        Box::new(rhs_expr.clone().analyse(scope, ExprPerms::Nothing)?),
      ),
      tmp_val.clone(),
      src.clone(),
    )),
    Box::new(Stat::Sequence(
      Box::new(Stat::Assignment(
        lhs_expr,
        Type::default(),
        Expr::PairElem(Box::new(PairElem::Fst(TypedExpr(
          Type::default(),
          tmp_val.clone(),
        )))),
      )),
      Box::new(Stat::Assignment(
        rhs_expr,
        Type::default(),
        Expr::PairElem(Box::new(PairElem::Snd(TypedExpr(
          Type::default(),
          tmp_val.clone(),
        )))),
      )),
    )),
  ))
}
