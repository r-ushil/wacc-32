extern crate nom;
use std::fs;

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::alphanumeric1,
  combinator::map,
  multi::{many0, many1},
  sequence::{delimited, pair, preceded, terminated, tuple},
  IResult,
};
use nom_supreme::{error::ErrorTree, final_parser};

use super::shared::*;
use super::stat::*;
use super::type_::*;
use crate::ast::*;
use crate::{
  analyser::context::{IdentInfo, SymbolTable},
  parse, read_file,
};

fn file_name(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
  alt((alphanumeric1, tag("/"), tag("-"), tag("_"), tag("../")))(input)
}

fn import_file(input: &str) -> IResult<&str, Vec<NamedFunc>, ErrorTree<&str>> {
  map(terminated(many1(file_name), tok(".wacc")), |filename| {
    let program_string =
      read_file(fs::File::open(format!("{}.wacc", filename.join(""))).unwrap());
    let program_str = program_string.as_str();

    parse(program_str).funcs
  })(input)
}

fn import_stat(input: &str) -> IResult<&str, Vec<NamedFunc>, ErrorTree<&str>> {
  preceded(tok("import"), import_file)(input)
}

pub fn final_program_parser(input: &str) -> Result<Program, ErrorTree<&str>> {
  final_parser::final_parser(program)(input)
}

/* program ::= 'begin' <func>* <stat> 'end' */
pub fn program(input: &str) -> IResult<&str, Program, ErrorTree<&str>> {
  let (input, _) = comment_or_ws(input)?;
  let (input, funcs) = many0(import_stat)(input)?;
  let mut funcs = funcs.into_iter().flatten().collect::<Vec<NamedFunc>>();

  let (input, (type_defs_vec, mut prog_funcs, statement)) = delimited(
    preceded(comment_or_ws, tok("begin")),
    tuple((many0(type_def), many0(named_func), stat)),
    tok("end"),
  )(input)?;

  funcs.append(&mut prog_funcs);

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

pub fn func(
  input: &str,
) -> IResult<&str, (Vec<(Type, String)>, Stat), ErrorTree<&str>> {
  map(
    tuple((tok("("), param_list, tok(")"), tok("is"), stat, tok("end"))),
    |(_, params, _, _, body, _)| (params, body),
  )(input)
}

/* func ::= <type> <ident> '(' <param-list>? ')' 'is' <stat> 'end' */
/* param-list ::= <param> ( ',' <param> )* */
fn named_func(input: &str) -> IResult<&str, NamedFunc, ErrorTree<&str>> {
  let (input, (return_type, ident, (params, body))) =
    tuple((type_, ident, func))(input)?;

  let (param_types, param_ids): (Vec<Type>, Vec<String>) =
    params.into_iter().unzip();

  Ok((
    input,
    (
      ident,
      Func {
        signature: FuncSig {
          param_types,
          return_type,
        },
        body,
        params_st: SymbolTable::default(),
        body_st: SymbolTable::default(),
        param_ids,
      },
    ),
  ))
}

/* param ::= <type> <ident> */
fn param(input: &str) -> IResult<&str, (Type, Ident), ErrorTree<&str>> {
  pair(type_, ident)(input)
}

pub fn param_list(
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
        funcs: vec!((
          "foo".to_string(),
          Func {
            signature: FuncSig {
              param_types: vec!(Type::Int),
              return_type: Type::Int
            },
            param_ids: vec!("x".to_string()),

            body: Stat::Return(Expr::Ident("x".to_string())),
            params_st: SymbolTable::default(),
            body_st: SymbolTable::default(),
          }
        )),
        statement: ScopedStat::new(Stat::Declaration(
          Type::Int,
          "y".to_string(),
          Expr::Call(
            Type::default(),
            Box::new(Expr::Ident("foo".to_string())),
            vec!(Expr::BinaryApp(
              Box::new(Expr::IntLiter(5)),
              BinaryOper::Add,
              Box::new(Expr::IntLiter(1)),
            )),
          )
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
    named_func("int firstFunc (int x, int y) is return x + y end"),
    Ok((
      "",
      ast)) if ast == ("firstFunc".to_string() , Func {signature:FuncSig{param_types:vec!(Type::Int,Type::Int),return_type:Type::Int,},body:Stat::Return(Expr::BinaryApp(Box::new(Expr::Ident("x".to_string())),BinaryOper::Add,Box::new(Expr::Ident("y".to_string())))),params_st:SymbolTable::default(),body_st:SymbolTable::default(),
                                                                            param_ids
                                                                          : vec!("x".to_string(),"y".to_string()) })
    ));

    assert!(matches!(
    named_func("int exitThree () is exit 3 end"),
    Ok((
      "",
      ast)) if ast == ("exitThree".to_string(), Func {signature:FuncSig{param_types:vec!(),return_type:Type::Int},body:Stat::Exit(Expr::IntLiter(3)),params_st:SymbolTable::default(),body_st:SymbolTable::default(), param_ids: vec!() })
    ));
  }

  #[test]
  fn test_func_with_func_in_args() {
    assert_eq!(
      named_func("int funcWithFunc (int(int, int) foo, int x) is int y = call foo(x); return y end").unwrap().1,
      ("funcWithFunc".to_string(),
      Func {
        signature:FuncSig{
          param_types:vec!(Type::Func(Box::new(FuncSig {param_types:vec!(Type::Int, Type::Int), return_type:Type::Int})), Type::Int),
          return_type:Type::Int,
        },
        body:Stat::Sequence(
          Box::new(Stat::Declaration(Type::Int, "y".to_string(), Expr::Call(Type::default(), Box::new(Expr::Ident("foo".to_string())), vec!(Expr::Ident("x".to_string()))))),
          Box::new(Stat::Return(Expr::Ident("y".to_string()))),
        ),
        params_st:SymbolTable::default(),
        body_st:SymbolTable::default(),
        param_ids:vec!("foo".to_string(), "x".to_string()),
      })
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

  // #[test]
  // fn test_import_statements() {
  //   let (_, ast) = program(
  //     "import test_integration/extension_executed/import-files/peano.wacc
  //     begin
  //     exit 0
  //     end",
  //   )
  //   .unwrap();

  //   assert!(ast.funcs.len() > 0);
  // }
}
