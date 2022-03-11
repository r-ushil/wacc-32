extern crate nom;
use nom::{
  multi::many0,
  sequence::{delimited, pair, preceded, tuple},
  IResult,
};
use nom_supreme::{error::ErrorTree, final_parser};

use super::shared::*;
use super::stat::*;
use super::type_::*;
use crate::analyser::context::SymbolTable;
use crate::ast::*;

pub fn final_program_parser(input: &str) -> Result<Program, ErrorTree<&str>> {
  final_parser::final_parser(program)(input)
}

/* program ::= 'begin' <func>* <stat> 'end' */
pub fn program(input: &str) -> IResult<&str, Program, ErrorTree<&str>> {
  let (input, (type_defs_vec, funcs, statement)) = delimited(
    preceded(comment_or_ws, tok("begin")),
    tuple((many0(type_def), many0(func), stat)),
    tok("end"),
  )(input)?;

  Ok((
    input,
    Program {
      /* Convert from vector of type defs to hashmap of typedefs. */
      type_defs: type_defs_vec.into_iter().collect(),
      funcs,
      statement: ScopedStat::new(statement),
      symbol_table: SymbolTable::default(),
    },
  ))
}

/* type-def ::= 'struct' <ident> '{' <param-list>? '}' */
fn type_def(input: &str) -> IResult<&str, (Ident, Struct), ErrorTree<&str>> {
  let (input, (id, fields)) = pair(
    /* 'struct' <ident> */
    preceded(tok("struct"), ident),
    /* '{' <param-list> '}' */
    delimited(tok("{"), param_list, tok("}")),
  )(input)?;

  /* Adds all fields to a struct definition. */
  let mut s = Struct::new();
  for (t, id) in fields {
    s.add_field(t, id);
  }

  Ok((input, (id, s)))
}

/* func ::= <type> <ident> '(' <param-list>? ')' 'is' <stat> 'end' */
/* param-list ::= <param> ( ',' <param> )* */
fn func(input: &str) -> IResult<&str, Func, ErrorTree<&str>> {
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
      params_st: SymbolTable::default(),
      body_st: SymbolTable::default(),
    },
  ))
}

/* param ::= <type> <ident> */
fn param(input: &str) -> IResult<&str, (Type, Ident), ErrorTree<&str>> {
  pair(type_, ident)(input)
}

fn param_list(
  input: &str,
) -> IResult<&str, Vec<(Type, Ident)>, ErrorTree<&str>> {
  many0_delimited(param, tok(","))(input)
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use super::*;

  #[test]
  fn test_program() {
    assert!(matches!(
    program("begin int foo(int x) is return x end int y = call foo(5 + 1) end"),
    Ok((
      "",
      ast)) if ast == Program {
        type_defs: HashMap::default(),
        funcs: vec!(Func {
          signature: FuncSig {
            params: vec!((Type::Int, "x".to_string())),
            return_type: Type::Int
          },
          ident: "foo".to_string(),
          body: Stat::Return(Expr::Ident("x".to_string())),
          params_st: SymbolTable::default(),
          body_st: SymbolTable::default(),
        }),
        statement: ScopedStat::new(Stat::Declaration(
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
        )),
        symbol_table: SymbolTable::default(),
      }
    ));
  }

  #[test]
  fn test_structs() {
    let p = program("begin struct foo { int x, char y } skip end")
      .unwrap()
      .1;

    assert_eq!(p.type_defs.len(), 1);

    assert_eq!(
      p.type_defs.get("foo").unwrap(),
      &Struct {
        fields: HashMap::from([
          (format!("x"), (Type::Int, 0)),
          (format!("y"), (Type::Char, 4))
        ]),
        size: 5
      }
    );
  }

  #[test]
  fn test_structs2() {
    program(
      "begin
      struct IntBox {
        int x
      }
      IntBox f = IntBox { x: 5 }
    end",
    )
    .unwrap()
    .1;
  }

  #[test]
  fn test_func() {
    assert!(matches!(
    func("int firstFunc (int x, int y) is return x + y end"),
    Ok((
      "",
      ast)) if ast == Func {
        ident: "firstFunc".to_string(),
        signature: FuncSig {
          params: vec!((Type::Int, "x".to_string()), (Type::Int, "y".to_string())),
          return_type: Type::Int,
        },
        body: Stat::Return(Expr::BinaryApp(
          Box::new(Expr::Ident("x".to_string())),
          BinaryOper::Add,
          Box::new(Expr::Ident("y".to_string()))
        )),
        params_st: SymbolTable::default(),
        body_st: SymbolTable::default(),
      }
    ));

    assert!(matches!(
    func("int exitThree () is exit 3 end"),
    Ok((
      "",
      ast)) if ast == Func {
        signature: FuncSig {
          params: vec!(),
          return_type: Type::Int
        },
        ident: "exitThree".to_string(),
        body: Stat::Exit(Expr::IntLiter(3)),
        params_st: SymbolTable::default(),
        body_st: SymbolTable::default(),
      }
    ));
  }

  #[test]
  fn test_param() {
    assert!(
      matches!(param("int x"), Ok(("", ast)) if ast == (Type::Int, "x".to_string()))
    );
    assert!(matches!(
    param("int [ ][ ] x"),
    Ok((
      "",
      ast)) if ast == (
        Type::Array(Box::new(Type::Array(Box::new(Type::Int)))),
        "x".to_string()
      )
    ));
  }
}
