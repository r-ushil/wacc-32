extern crate nom;
use nom::{
  multi::many0,
  sequence::{delimited, pair, preceded, tuple},
  IResult,
};

use super::shared::*;
use super::stat::*;
use super::type_::*;
use crate::ast::*;

/* program ::= 'begin' <func>* <stat> 'end' */
pub fn program(input: &str) -> IResult<&str, Program> {
  let (input, (funcs, statement)) = delimited(
    preceded(comment_or_ws, tok("begin")),
    pair(many0(func), stat),
    tok("end"),
  )(input)?;

  Ok((input, Program { funcs, statement }))
}

/* func ::= <type> <ident> '(' <param-list>? ')' 'is' <stat> 'end' */
/* param-list ::= <param> ( ',' <param> )* */
fn func(input: &str) -> IResult<&str, Func> {
  let param_list = many0_delimited(param, tok(","));

  let (input, (return_type, ident, _, params, _, _, body, _)) = tuple((
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
      ident,
      signature: FuncSig {
        params,
        return_type,
      },
      body,
    },
  ))
}

/* param ::= <type> <ident> */
fn param(input: &str) -> IResult<&str, (Type, Ident)> {
  pair(type_, ident)(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_program() {
    assert_eq!(
      program("begin int foo(int x) is return x end int y = call foo(5 + 1) end"),
      Ok((
        "",
        Program {
          funcs: vec!(Func {
            signature: FuncSig {
              params: vec!((Type::Int, "x".to_string())),
              return_type: Type::Int
            },
            ident: "foo".to_string(),
            body: Stat::Return(Expr::Ident("x".to_string())),
          }),
          statement: Stat::Declaration(
            Type::Int,
            "y".to_string(),
            AssignRhs::Call(
              "foo".to_string(),
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
          ident: "firstFunc".to_string(),
          signature: FuncSig {
            params: vec!((Type::Int, "x".to_string()), (Type::Int, "y".to_string())),
            return_type: Type::Int,
          },
          body: Stat::Return(Expr::BinaryApp(
            Box::new(Expr::Ident("x".to_string())),
            BinaryOper::Add,
            Box::new(Expr::Ident("y".to_string()))
          ))
        }
      ))
    );

    assert_eq!(
      func("int exitThree () is exit 3 end"),
      Ok((
        "",
        Func {
          signature: FuncSig {
            params: vec!(),
            return_type: Type::Int
          },
          ident: "exitThree".to_string(),
          body: Stat::Exit(Expr::IntLiter(3))
        }
      ))
    );
  }

  #[test]
  fn test_param() {
    assert_eq!(param("int x"), Ok(("", (Type::Int, "x".to_string()))));
    assert_eq!(
      param("int [ ][ ] x"),
      Ok((
        "",
        (
          Type::Array(Box::new(Type::Array(Box::new(Type::Int)))),
          "x".to_string()
        )
      ))
    );
  }
}
