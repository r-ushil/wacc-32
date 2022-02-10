extern crate nom;
use nom::{
  branch::alt,
  bytes::complete::{is_not, tag},
  character::complete::{alpha1, alphanumeric1, anychar, char as char_, multispace0},
  combinator::{map, not, opt, recognize, value, verify},
  error::ParseError,
  multi::{many0, many1},
  sequence::{delimited, pair, terminated},
  IResult, Parser,
};

use crate::ast::*;

/* ======= HELPER FUNCTIONS ======= */

/* https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#whitespace */
/* Consumes leading and trailing whitespace, then applies a parser
to the inner content. */
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
  inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
  F: Parser<&'a str, O, E>,
{
  terminated(inner, comment_or_ws)
}

fn comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
  value("", pair(char_('#'), is_not("\n\r")))(input)
}

pub fn comment_or_ws<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
  value(
    "",
    many0(alt((
      char_(' '),
      char_('\n'),
      char_('\r'),
      char_('\t'),
      value('a', comment),
    ))),
  )(input)
}

/* Consumes whitespace, matches tag, consumes whitespace.
Returns tag. */
pub fn tok<'a>(t: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
  ws(tag(t))
}

/* Like many0, but each of the elements are seperated by another parser,
the result of which is thrown away. */
pub fn many0_delimited<'a, O, O2, Ep: 'a, Dp: 'a, E>(
  element: Ep,
  delimeter: Dp,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
  E: ParseError<&'a str>,
  Ep: Parser<&'a str, O, E> + Copy,
  Dp: Parser<&'a str, O2, E>,
{
  map(
    pair(many0(terminated(element, delimeter)), opt(element)),
    |(mut elements, optlast)| {
      if let Some(last) = optlast {
        elements.push(last);
      }
      elements
    },
  )
}

/* ======= PARSERS ======= */

/* 〈ident〉::= (‘_’ | ‘a’-‘z’ | ‘A’-‘Z’) (‘_’ | ‘a’-‘z’ | ‘A’-‘Z’ |
 * ‘0’-‘9’)* */

pub fn is_keyword(ident: &str) -> bool {
  match ident {
    "begin" | "end" | "is" | "skip" | "read" | "free" | "return" | "exit" | "print" | "println"
    | "if" | "then" | "else" | "fi" | "while" | "do" | "done" | "newpair" | "call" | "fst"
    | "snd" | "int" | "bool" | "char" | "string" | "pair" | "len" | "ord" | "chr" | "true"
    | "false" | "null" => true,

    _ => false,
  }
}

pub fn ident(input: &str) -> IResult<&str, Ident> {
  let ident_parser = map(
    /* Then recognise will return the part of the input that got consumed. */
    recognize(pair(
      /* The parsers in here will match the whole identifier. */
      alt((alpha1, tag("_"))),
      many0(alt((alphanumeric1, tag("_")))),
    )),
    |s: &str| (s.to_string()), /* Copy string into identifier. */
  );

  ws(verify(ident_parser, |id| !is_keyword(id)))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ident() {
    assert_eq!(ident("_hello123"), Ok(("", ("_hello123".to_string()))));
    assert_eq!(
      ident("_hello123 test"),
      Ok(("test", ("_hello123".to_string())))
    );
    assert!(ident("9test").is_err());
    assert_eq!(ident("te@st"), Ok(("@st", ("te".to_string()))));

    assert!(ident("read").is_err());
    assert!(ident("begin").is_err());
    assert_eq!(ident("lenx"), Ok(("", ("lenx".to_string()))));
  }

  #[test]
  fn test_many0_multispace0() {
    let input = "#hello \n   #a;sdkjf;lakdsjf\n  #hi there\nsomething";
    let x: IResult<_, _> = many0(alt((
      char_(' '),
      char_('\n'),
      char_('\r'),
      char_('\t'),
      value('a', comment),
    )))(input);
    let (input, _) = x.unwrap();

    assert_eq!(input, "something");
  }
}
