extern crate nom;
use nom::{
  IResult,
  bytes::complete::{tag, take_while},
  branch::alt,
  combinator::{
    value,
    map, recognize,
  },
  sequence::{
    pair,
    tuple,
    terminated,
    preceded, delimited
  },
  multi::many0,
  character::{is_space, complete::{multispace0, alpha1, alphanumeric1}}, error::{ParseError, Error, ErrorKind}, Parser,
};

use crate::ast::*;

/* ======= HELPER FUNCTIONS ======= */

/* Consumes leading and trailing whitespace, then applies a parser
   to the inner content. */
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Parser<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

pub fn tok<'a>(t: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
  delimited(
    multispace0,
    tag(t),
    multispace0
  )
}

/* ======= PARSERS ======= */
/* The parser which parses a string into a AST node of type
ExampleNode, will have the name example_node. */
/* If names conflict with Rust keywords, an underscore is appended. */
/* All parsers will consume all leading whitespace before and after parsing. */

/* program ::= 'begin' <func>* <stat> 'end' */
pub fn program(input: &str) -> IResult<&str, Program> {
  todo!();
}

/* func ::= <type> <ident> '(' <param-list>? ') 'is' <stat> 'end' */
/* param-list ::= <param> ( ',' <param> )* */
fn func(input: &str) -> IResult<&str, Func> {
  todo!();
}

/* param ::= <type> <ident> */
fn param(input: &str) -> IResult<&str, Param> {
  let (input, (t, id)) = pair(type_, ident)(input)?;

  Ok((input, Param(t, id)))
}

/* stat ::= 'skip'
          | <type> <ident> '=' <assign-rhs>
          | <assign-lhs> '=' <assign-rhs>
          | 'read' <assign-lhs>
          | 'free' <expr>
          | 'return' <expr>
          | 'exit' <expr>
          | 'print' <expr>
          | 'println' <expr>
          | 'if' <expr> 'then' <stat> 'else' <stat> 'fi'
          | 'while' <expr> 'do' <stat> 'done'
          | 'begin' <stat> 'end'
          | <stat> ';' <stat> */
fn stat(input: &str) -> IResult<&str, Stat> {
  todo!();
}

/* assign-lhs ::= <ident> | <array-elem> | <pair-elem> */
fn assign_lhs(input: &str) -> IResult<&str, AssignLhs> {
  todo!();
}

/* assign-rhs ::= <expr>
                | <array-liter>
                | 'newpair' '(' <expr> ',' <expr> ')'
                | <pair-elem>
                | 'call' <ident> '(' <arg-list>? ')' */
/* arg-list ::= <expr> ( ',' <expr> )* */
fn assign_rhs(input: &str) -> IResult<&str, AssignRhs> {
  todo!();
}

/* pair-elem ::= 'fst' <expr> | 'snd' <expr> */
fn pair_elem(input: &str) -> IResult<&str, PairElem> {
  todo!();
}

/* type ::= <base-type> | <array-type> | <pair-type> */
fn type_(input: &str) -> IResult<&str, Type> {
  /* Parses everything apart from the trailing array notes. */
  let (input, mut t) = alt((
    map(base_type, |bt| Type::BaseType(bt)),
    map(
      tuple((
        tok("pair("), pair_elem_type, tok(","), pair_elem_type, tok(")"),
      )),
      |(_, l, _, r, _)| Type::Pair(l, r),
    ),
  ))(input)?;

  /* Counts how many '[]' trail. */
  let (input, arrs) = many0(
    pair(tok("["), tok("]"))
  )(input)?;

  /* Nests t in Type::Array's that amount of times. */
  for _ in arrs {
    t = Type::Array(Box::new(t));
  }

  Ok((input, t))
}

/* 'int' | 'bool' | 'char' | 'string' */
fn base_type(input: &str) -> IResult<&str, BaseType> {
  alt((
    value(BaseType::Int, tok("int")),
    value(BaseType::Bool, tok("bool")),
    value(BaseType::Char, tok("char")),
    value(BaseType::String, tok("string")),
  ))(input)
}

/* pair-elem-type ::= <base-type> | <array-type> | 'pair' */
fn pair_elem_type(input: &str) -> IResult<&str, PairElemType> {
  /* Type logic reused for base types and arrays, because pairs
  are different we have to handle that edge case. */
  match type_(input) {
    Ok((input, Type::BaseType(it))) => Ok((input, PairElemType::BaseType(it))),
    Ok((input, Type::Array(it))) => Ok((input, PairElemType::Array(it))),
    _ => value(PairElemType::Pair, tok("pair"))(input)
  }
}

/*〈expr〉 ::= 〈int-liter〉  //〈int-liter〉::= (‘+’ | ‘-’) ? (‘0’-‘9’)
            | 〈bool-liter〉 //〈bool-liter〉::= ‘true’ | ‘false’
            | 〈char-liter〉 //〈char-liter〉::= ‘'’〈character〉‘'’
            | 〈str-liter〉  //〈str-liter〉::= ‘"’〈character〉* ‘"’
            | 〈pair-liter〉 //〈pair-liter〉::= ‘null’
            | 〈ident〉
            | 〈array-elem〉
            | 〈unary-oper〉〈expr〉          //〈unary-oper〉::= ‘!’ | ‘-’ | ‘len’ | ‘ord’ | ‘chr’
            | 〈expr〉〈binary-oper〉〈expr〉  //〈binary-oper〉::= ‘*’ | ‘/’ | ‘%’ | ‘+’ | ‘-’ | ‘>’ | ‘>=’ | ‘<’ | ‘<=’ | ‘==’ | ‘!=’ | ‘&&’ | ‘||’
            | ‘(’〈expr〉‘)’ */
fn expr(input: &str) -> IResult<&str, Expr> {
  todo!();
}

fn unary_oper(input: &str) -> IResult<&str, UnaryOper> {
  todo!();
}

fn binary_oper(input: &str) -> IResult<&str, BinaryOper> {
  todo!();
}

/*〈ident〉::= (‘_’ | ‘a’-‘z’ | ‘A’-‘Z’) (‘_’ | ‘a’-‘z’ | ‘A’-‘Z’ | ‘0’-‘9’)* */
fn ident(input: &str) -> IResult<&str, Ident> {
  map(recognize(
    pair(
      alt((alpha1, tag("_"))),
      many0(alt((alphanumeric1, tag("_"))))
    )
  ), |s: &str| Ident(s.to_string()))(input)
}

/*〈array-elem〉::=〈ident〉(‘[’〈expr〉‘]’)+ */
fn array_elem(input: &str) -> IResult<&str, ArrayElem> {
  todo!();
}

/*〈array-liter〉::= ‘[’ (〈expr〉 (‘,’〈expr〉)* )? ‘]’ */
fn array_liter(input: &str) -> IResult<&str, ArrayLiter> {
  todo!();
}

pub fn main() {
  println!("Hello, World!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_program() {}

  #[test]
  fn test_func() {}

  #[test]
  fn test_param() {
    assert_eq!(param("int x"),
      Ok(("", Param(Type::BaseType(BaseType::Int), Ident("x".to_string()))))
    );
    assert_eq!(param("int [ ][ ] x"),
      Ok(("", Param(
        Type::Array(Box::new(Type::Array(Box::new(Type::BaseType(BaseType::Int))))),
        Ident("x".to_string()))
      ))
    );
  }

  #[test]
  fn test_stat() {}

  #[test]
  fn test_assign_lhs() {}

  #[test]
  fn test_assign_rhs() {}

  #[test]
  fn test_pair_elem() {}

  #[test]
  fn test_type_() {
    assert_eq!(type_("int"),
      Ok(("", Type::BaseType(BaseType::Int))),
    );
    assert_eq!(type_("pair(int[], int)[]"),
      Ok(("", Type::Array(Box::new(Type::Pair(
        PairElemType::Array(Box::new(Type::BaseType(BaseType::Int))),
        PairElemType::BaseType(BaseType::Int))
      ))))
    );
    assert_eq!(type_("pair(pair, string)"),
      Ok(("", Type::Pair(
        PairElemType::Pair,
        PairElemType::BaseType(BaseType::String))
      ))
    );
    assert!(type_("pair(pair(int, int), string)").is_err());
  }

  #[test]
  fn test_pair_elem_type() {
    assert_eq!(pair_elem_type("int"),
      Ok(("", PairElemType::BaseType(BaseType::Int))),
    );
    assert_eq!(pair_elem_type("char[]"),
      Ok(("", PairElemType::Array(Box::new(Type::BaseType(BaseType::Char))))),
    );
    assert_eq!(pair_elem_type("pair"),
      Ok(("", PairElemType::Pair))
    );
    assert_eq!(pair_elem_type("pair(int, int)"),
      Ok(("(int, int)", PairElemType::Pair))
    );
    assert_eq!(pair_elem_type("pair(int, int)[]"),
      Ok(("", PairElemType::Array(Box::new(Type::Pair(
        PairElemType::BaseType(BaseType::Int),
        PairElemType::BaseType(BaseType::Int),
      ))))
    ));
  }

  #[test]
  fn test_base_type() {}

  #[test]
  fn test_expr() {}

  #[test]
  fn test_unary_oper() {}

  #[test]
  fn test_binary_oper() {}

  #[test]
  fn test_ident() {
    assert_eq!(ident("_hello123"), 
      Ok(("", Ident("_hello123".to_string())))
    );
    assert_eq!(ident("_hello123 test"), 
      Ok((" test", Ident("_hello123".to_string())))
    );
    assert!(ident("9test").is_err());
    assert_eq!(ident("te@st"), 
      Ok(("@st", Ident("te".to_string())))
    );
  }

  #[test]
  fn test_array_elem() {}

  #[test]
  fn test_array_liter() {}
}
