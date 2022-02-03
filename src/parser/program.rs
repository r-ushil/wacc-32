extern crate nom;
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{alpha1, alphanumeric1, char as char_, digit1, multispace0, none_of},
  combinator::{map, opt, recognize, value},
  error::ParseError,
  multi::{many0, many1},
  sequence::{delimited, pair, preceded, terminated, tuple},
  IResult, Parser,
};

use super::shared::*;
use super::stat::*;
use super::type_::*;
use crate::ast::*;

/* program ::= 'begin' <func>* <stat> 'end' */
pub fn program(input: &str) -> IResult<&str, Program> {
  let (input, (funcs, statement)) =
    delimited(tok("begin"), pair(many0(func), stat), tok("end"))(input)?;

  Ok((input, Program { funcs, statement }))
}

/* func ::= <type> <ident> '(' <param-list>? ')' 'is' <stat> 'end' */
/* param-list ::= <param> ( ',' <param> )* */
fn func(input: &str) -> IResult<&str, Func> {
  let param_list = many0_delimited(param, tok(","));

  let (input, (return_type, ident, _, param_list, _, _, body, _)) = tuple((
    type_,
    ident,
    tok("("),
    param_list,
    tok(")"),
    tok("is"),
    stat,
    tok("end"),
  ))(input)?;

  Ok((
    input,
    Func {
      return_type,
      ident,
      param_list,
      body,
    },
  ))
}

/* param ::= <type> <ident> */
fn param(input: &str) -> IResult<&str, Param> {
  let (input, (t, id)) = pair(type_, ident)(input)?;

  Ok((input, Param(t, id)))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_program() {
    assert_eq!(
      program("begin int foo(int x) is return x end int y = call foo(5 + 1) end",),
      Ok((
        "",
        Program {
          funcs: vec!(Func {
            return_type: Type::BaseType(BaseType::Int),
            ident: Ident("foo".to_string()),
            param_list: vec!(Param(Type::BaseType(BaseType::Int), Ident("x".to_string()),)),
            body: Stat::Return(Expr::Ident(Ident("x".to_string()))),
          }),
          statement: Stat::Declaration(
            Type::BaseType(BaseType::Int),
            Ident("y".to_string()),
            AssignRhs::Call(
              Ident("foo".to_string()),
              vec!(Expr::BinaryApp(
                Box::new(Expr::IntLiter(5)),
                BinaryOper::Add,
                Box::new(Expr::IntLiter(1)),
              )),
            )
          )
        }
      ))
    );
  }

  #[test]
  fn test_func() {
    assert_eq!(
      func("int firstFunc (int x, int y) is return x + y end"),
      Ok((
        "",
        Func {
          return_type: Type::BaseType(BaseType::Int),
          ident: Ident("firstFunc".to_string()),
          param_list: vec!(
            Param(Type::BaseType(BaseType::Int), Ident("x".to_string())),
            Param(Type::BaseType(BaseType::Int), Ident("y".to_string()))
          ),
          body: Stat::Return(Expr::BinaryApp(
            Box::new(Expr::Ident(Ident("x".to_string()))),
            BinaryOper::Add,
            Box::new(Expr::Ident(Ident("y".to_string())))
          ))
        }
      ))
    );

    assert_eq!(
      func("int exitThree () is exit 3 end"),
      Ok((
        "",
        Func {
          return_type: Type::BaseType(BaseType::Int),
          ident: Ident("exitThree".to_string()),
          param_list: vec!(),
          body: Stat::Exit(Expr::IntLiter(3))
        }
      ))
    );
  }

  #[test]
  fn test_param() {
    assert_eq!(
      param("int x"),
      Ok((
        "",
        Param(Type::BaseType(BaseType::Int), Ident("x".to_string()))
      ))
    );
    assert_eq!(
      param("int [ ][ ] x"),
      Ok((
        "",
        Param(
          Type::Array(Box::new(Type::Array(Box::new(Type::BaseType(
            BaseType::Int
          ))))),
          Ident("x".to_string())
        )
      ))
    );
  }
}
