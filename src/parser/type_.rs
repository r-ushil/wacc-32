extern crate nom;
use nom::{
  branch::alt,
  combinator::{map, value},
  multi::many0,
  sequence::{delimited, pair, tuple},
  IResult,
};
use nom_supreme::{
  error::{ErrorTree, Expectation},
  ParserExt,
};

use super::shared::*;
use crate::ast::*;

pub fn func_param_types(
  input: &str,
) -> IResult<&str, Vec<Type>, ErrorTree<&str>> {
  delimited(tok("("), many0_delimited(type_, tok(",")), tok(")"))(input)
}

/* type ::= <base-type> | <array-type> | <pair-type> */
pub fn type_(input: &str) -> IResult<&str, Type, ErrorTree<&str>> {
  /* Parses everything apart from the trailing array notes. */
  let (mut input, mut t) = alt((
    base_type,
    map(
      tuple((
        tok("pair"),
        tok("("),
        pair_elem_type,
        tok(","),
        pair_elem_type,
        tok(")"),
      )),
      |(_, _, l, _, r, _)| Type::Pair(Box::new(l), Box::new(r)),
    ),
    map(upper_ident, Type::Custom),
  ))(input)?;

  loop {
    if let Ok((i, _)) =
      pair::<_, _, _, ErrorTree<&str>, _, _>(tok("["), tok("]"))(input)
    {
      /* Consume input. */
      input = i;

      /* Array type. */
      t = Type::Array(Box::new(t));
    } else if let Ok((i, param_types)) = func_param_types(input) {
      /* Consume input. */
      input = i;

      /* Function type. */
      t = Type::Func(Box::new(FuncSig {
        param_types,
        return_type: t,
      }));
    } else {
      /* No trailing parsers, finished with suffixes. */
      break;
    }
  }

  /* Counts how many '[]' trail. */
  let (input, arrs) = many0(pair(tok("["), tok("]")))(input)?;

  /* Nests t in Type::Array's that amount of times. */
  for _ in arrs {
    t = Type::Array(Box::new(t));
  }

  Ok((input, t))
}

/* upper-ident ::= ident but first character is upper case. */
fn upper_ident(input: &str) -> IResult<&str, Ident, ErrorTree<&str>> {
  let (input, id) = ident(input)?;

  if !id.chars().nth(0).unwrap().is_uppercase() {
    return Err(nom::Err::Error(nom_supreme::error::ErrorTree::Base {
      location: input,
      kind: nom_supreme::error::BaseErrorKind::Expected(Expectation::Tag(
        "Upper case ident",
      )),
    }));
  }

  Ok((input, id))
}

/* base-type ::= 'int' | 'bool' | 'char' | 'string' */
fn base_type(input: &str) -> IResult<&str, Type, ErrorTree<&str>> {
  alt((
    value(Type::Int, key("int")).context("expected int"),
    value(Type::Bool, key("bool")).context("expected bool"),
    value(Type::Char, key("char")).context("expected char"),
    value(Type::String, key("string")).context("expected string"),
  ))(input)
}

/* pair-elem-type ::= <base-type> | <array-type> | 'pair' */
fn pair_elem_type(input: &str) -> IResult<&str, Type, ErrorTree<&str>> {
  use nom_supreme::error::Expectation;

  /* Type logic reused for base types and arrays, because pairs
  are different we have to handle that edge case. */
  match type_(input) {
    /* pair(int, int) is allowed as a regular type, but not as a pair_elem_type */
    Ok((input, Type::Pair(_, _))) => {
      Err(nom::Err::Error(nom_supreme::error::ErrorTree::Base {
        location: input,
        kind: nom_supreme::error::BaseErrorKind::Expected(Expectation::Tag(
          "cannot have strongly defined nested pair types.",
        )),
      }))
    }

    /* Everything else the regular type parser can deal with is also a pair_elem_type */
    Ok(result) => Ok(result),
    /* But pair_elem_type can also 'pair' */
    _ => value(
      Type::Pair(Box::new(Type::Any), Box::new(Type::Any)),
      tok("pair"),
    )(input),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_upper_ident() {
    assert!(type_("y").is_err());
  }

  #[test]
  fn test_custom() {
    assert_eq!(type_("foobar").unwrap().1, Type::Custom(format!("foobar")));
  }

  #[test]
  fn test_type_() {
    assert!(matches!(type_("int"), Ok(("", Type::Int))));
    assert!(matches!(
      type_("pair (int [], int)[ ]"),
      Ok((
        "",
        ast)) if ast == Type::Array(Box::new(Type::Pair(
          Box::new(Type::Array(Box::new(Type::Int))),
          Box::new(Type::Int)
        )))
    ));
    assert!(matches!(
      type_("pair (pair , string)"),
      Ok((
        "",
        ast)) if ast == Type::Pair(
          Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any))),
          Box::new(Type::String),
        )
    ));
    assert!(matches!(
      type_("pair(int, int)[]"),
      Ok((
        "",
        ast)) if ast == Type::Array(Box::new(Type::Pair(
          Box::new(Type::Int),
          Box::new(Type::Int),
        )))
    ));
    println!("{:?}", type_("pair(pair(int, int), string)"));
    assert!(type_("pair(pair(int, int), string)").is_err());
  }

  #[test]
  fn test_pair_elem_type() {
    assert!(matches!(pair_elem_type("int"), Ok(("", Type::Int))));
    assert!(matches!(
      pair_elem_type("char[ ]"),
      Ok(("", ast)) if ast == Type::Array(Box::new(Type::Char)),
    ));
    assert!(matches!(
      pair_elem_type("pair"),
      Ok(("", ast)) if ast == Type::Pair(Box::new(Type::Any), Box::new(Type::Any))
    ));
    // assert!(matches!(
    //   pair_elem_type("pair(int, int)"),
    //   Ok(("(int, int)", Type::Any))
    // ); //unneeded test? the pair_elem parser should never deal with pair with
    // brackets...
  }

  #[test]
  fn test_base_type() {
    assert!(matches!(base_type("int"), Ok(("", Type::Int))));
    assert!(matches!(base_type("bool"), Ok(("", Type::Bool))));
    assert!(matches!(base_type("char"), Ok(("", Type::Char))));
    assert!(matches!(base_type("string"), Ok(("", Type::String))));
  }
}
