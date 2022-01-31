extern crate nom;
use nom::IResult;

use crate::ast::*;

pub fn program(input: &str) -> IResult<&str, Program> {
  todo!();
}

fn func(input: &str) -> IResult<&str, Func> {
  todo!();
}

fn param(input: &str) -> IResult<&str, Param> {
  todo!();
}

fn stat(input: &str) -> IResult<&str, Stat> {
  todo!();
}

fn assign_lhs(input: &str) -> IResult<&str, AssignLhs> {
  todo!();
}

fn assign_rhs(input: &str) -> IResult<&str, AssignRhs> {
  todo!();
}

fn pair_elem(input: &str) -> IResult<&str, PairElem> {
  todo!();
}

fn type_(input: &str) -> IResult<&str, Type> {
  todo!();
}

fn pair_elem_type(input: &str) -> IResult<&str, PairElemType> {
  todo!();
}

fn base_type(input: &str) -> IResult<&str, BaseType> {
  todo!();
}

fn expr(input: &str) -> IResult<&str, Expr> {
  todo!();
}

fn unary_oper(input: &str) -> IResult<&str, UnaryOper> {
  todo!();
}

fn binary_oper(input: &str) -> IResult<&str, BinaryOper> {
  todo!();
}

fn ident(input: &str) -> IResult<&str, Ident> {
  todo!();
}

fn array_elem(input: &str) -> IResult<&str, ArrayElem> {
  todo!();
}

fn array_liter(input: &str) -> IResult<&str, ArrayLiter> {
  todo!();
}

pub fn main() {
  println!("Hello, World!");
}

#[cfg(tests)]
mod tests {
  use super::*;

  #[test]
  fn test_program() {}

  #[test]
  fn test_func() {}

  #[test]
  fn test_param() {}

  #[test]
  fn test_stat() {}

  #[test]
  fn test_assign_lhs() {}

  #[test]
  fn test_assign_rhs() {}

  #[test]
  fn test_pair_elem() {}

  #[test]
  fn test_type_() {}

  #[test]
  fn test_pair_elem_type() {}

  #[test]
  fn test_base_type() {}

  #[test]
  fn test_expr() {}

  #[test]
  fn test_unary_oper() {}

  #[test]
  fn test_binary_oper() {}

  #[test]
  fn test_ident() {}

  #[test]
  fn test_array_elem() {}

  #[test]
  fn test_array_liter() {}
}
