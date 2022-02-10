extern crate nom;
use nom::{
  branch::alt,
  combinator::{map, value},
  multi::many0,
  sequence::{pair, tuple},
  IResult,
};

use super::shared::*;
use crate::ast::*;

/* type ::= <base-type> | <array-type> | <pair-type> */
pub fn type_(input: &str) -> IResult<&str, Type> {
  /* Parses everything apart from the trailing array notes. */
  let (input, mut t) = alt((
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
  ))(input)?; // int [] [][][][]

  /* Counts how many '[]' trail. */
  let (input, arrs) = many0(pair(tok("["), tok("]")))(input)?;

  /* Nests t in Type::Array's that amount of times. */
  for _ in arrs {
    t = Type::Array(Box::new(t));
  }

  Ok((input, t))
}

/* base-type ::= 'int' | 'bool' | 'char' | 'string' */
fn base_type(input: &str) -> IResult<&str, Type> {
  alt((
    value(Type::Int, tok("int")),
    value(Type::Bool, tok("bool")),
    value(Type::Char, tok("char")),
    value(Type::String, tok("string")),
  ))(input)
}

/* pair-elem-type ::= <base-type> | <array-type> | 'pair' */
fn pair_elem_type(input: &str) -> IResult<&str, Type> {
  /* Type logic reused for base types and arrays, because pairs
  are different we have to handle that edge case. */
  match type_(input) {
    /* pair(int, int) is allowed as a regular type, but not as a pair_elem_type */
    Ok((input, Type::Pair(_, _))) => Err(nom::Err::Error(nom::error::Error::new(
      input,
      nom::error::ErrorKind::Alpha,
    ))),
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
  fn test_type_() {
    assert_eq!(type_("int"), Ok(("", Type::Int)),);
    assert_eq!(
      type_("pair (int [], int)[ ]"),
      Ok((
        "",
        Type::Array(Box::new(Type::Pair(
          Box::new(Type::Array(Box::new(Type::Int))),
          Box::new(Type::Int)
        )))
      ))
    );
    assert_eq!(
      type_("pair (pair , string)"),
      Ok((
        "",
        Type::Pair(
          Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any))),
          Box::new(Type::String),
        )
      ))
    );
    assert_eq!(
      type_("pair(int, int)[]"),
      Ok((
        "",
        Type::Array(Box::new(Type::Pair(
          Box::new(Type::Int),
          Box::new(Type::Int),
        )))
      ))
    );
    println!("{:?}", type_("pair(pair(int, int), string)"));
    assert!(type_("pair(pair(int, int), string)").is_err());
  }

  #[test]
  fn test_pair_elem_type() {
    assert_eq!(pair_elem_type("int"), Ok(("", Type::Int)),);
    assert_eq!(
      pair_elem_type("char[ ]"),
      Ok(("", Type::Array(Box::new(Type::Char)))),
    );
    assert_eq!(
      pair_elem_type("pair"),
      Ok(("", Type::Pair(Box::new(Type::Any), Box::new(Type::Any))))
    );
    // assert_eq!(
    //   pair_elem_type("pair(int, int)"),
    //   Ok(("(int, int)", Type::Any))
    // ); //unneeded test? the pair_elem parser should never deal with pair with
    // brackets...
  }

  #[test]
  fn test_base_type() {
    assert_eq!(base_type("int"), Ok(("", Type::Int)));
    assert_eq!(base_type("bool"), Ok(("", Type::Bool)));
    assert_eq!(base_type("char"), Ok(("", Type::Char)));
    assert_eq!(base_type("string"), Ok(("", Type::String)));
  }
}
