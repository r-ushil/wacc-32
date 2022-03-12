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
use crate::analyser::context::{IdentInfo, SymbolTable};
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

  /* Convert from vector of type defs to hashmap of typedefs. */
  let mut symbol_table = SymbolTable::default();
  for (struct_name, struct_def) in type_defs_vec {
    symbol_table
      .table
      .insert(struct_name, IdentInfo::TypeDef(struct_def));
  }

  Ok((
    input,
    Program {
      funcs,
      statement: ScopedStat::new(statement),
      symbol_table,
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

  let (param_types, param_ids): (Vec<Type>, Vec<String>) =
    params.into_iter().unzip();

  Ok((
    input,
    Func {
      ident,
      signature: FuncSig {
        param_types,
        return_type,
      },
      body,
      params_st: SymbolTable::default(),
      body_st: SymbolTable::default(),
      param_ids,
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
    assert_eq!(
      program(
        "begin int foo(int x) is return x end int y = call foo(5 + 1) end"
      )
      .unwrap()
      .1,
      Program {
        funcs: vec!(Func {
          signature: FuncSig {
            param_types: vec!(Type::Int),
            return_type: Type::Int
          },
          param_ids: vec!("x".to_string()),
          ident: "foo".to_string(),
          body: Stat::Return(Expr::Ident("x".to_string())),
          params_st: SymbolTable::default(),
          body_st: SymbolTable::default(),
        }),
        statement: ScopedStat::new(Stat::Declaration(
          Type::Int,
          "y".to_string(),
          AssignRhs::Expr(Expr::Call(
            Type::default(),
            Box::new(Expr::Ident("foo".to_string())),
            vec!(Expr::BinaryApp(
              Box::new(Expr::IntLiter(5)),
              BinaryOper::Add,
              Box::new(Expr::IntLiter(1)),
            )),
          ))
        )),
        symbol_table: SymbolTable::default(),
      }
    );
  }

  #[test]
  fn test_structs() {
    let p = program("begin struct foo { int x, char y } skip end")
      .unwrap()
      .1;

    assert_eq!(p.symbol_table.table.len(), 1);

    assert_eq!(
      p.symbol_table.table.get("foo").unwrap(),
      &IdentInfo::TypeDef(Struct {
        fields: HashMap::from([
          (format!("x"), (Type::Int, 0)),
          (format!("y"), (Type::Char, 4))
        ]),
        size: 5
      })
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
      ast)) if ast == Func {ident:"firstFunc".to_string(),signature:FuncSig{param_types:vec!(Type::Int,Type::Int),return_type:Type::Int,},body:Stat::Return(Expr::BinaryApp(Box::new(Expr::Ident("x".to_string())),BinaryOper::Add,Box::new(Expr::Ident("y".to_string())))),params_st:SymbolTable::default(),body_st:SymbolTable::default(),
                                                                            param_ids
                                                                          : vec!("x".to_string(),"y".to_string()) }
    ));

    assert!(matches!(
    func("int exitThree () is exit 3 end"),
    Ok((
      "",
      ast)) if ast == Func {signature:FuncSig{param_types:vec!(),return_type:Type::Int},ident:"exitThree".to_string(),body:Stat::Exit(Expr::IntLiter(3)),params_st:SymbolTable::default(),body_st:SymbolTable::default(), param_ids: vec!() }
    ));
  }

  #[test]
  fn test_func_with_func_in_args() {
    assert_eq!(
      func("int funcWithFunc (int(int, int) foo, int x) is int y = call foo(x); return y end").unwrap().1,
      Func {
        ident:"funcWithFunc".to_string(),
        signature:FuncSig{
          param_types:vec!(Type::Func(Box::new(FuncSig {param_types:vec!(Type::Int, Type::Int), return_type:Type::Int})), Type::Int),
          return_type:Type::Int,
        },
        body:Stat::Sequence(
          Box::new(Stat::Declaration(Type::Int, "y".to_string(), AssignRhs::Expr(Expr::Call(Type::default(), Box::new(Expr::Ident("foo".to_string())), vec!(Expr::Ident("x".to_string())))))),
          Box::new(Stat::Return(Expr::Ident("y".to_string()))),
        ),
        params_st:SymbolTable::default(),
        body_st:SymbolTable::default(),
        param_ids:vec!("foo".to_string(), "x".to_string()),
      }
    );
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
