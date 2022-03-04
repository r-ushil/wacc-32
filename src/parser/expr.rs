extern crate nom;
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{char as char_, digit1, none_of},
  combinator::{map, opt, value},
  multi::{many0, many1},
  sequence::{delimited, pair, preceded},
  IResult,
};
use nom_supreme::error::ErrorTree;

use super::shared::*;
use crate::ast::*;

const BINARY_OP_MAX_PREC: u8 = 6;

/*〈expr〉 ::= 〈int-liter〉
| 〈bool-liter〉 //〈bool-liter〉::= ‘true’ | ‘false’
| 〈char-liter〉 //〈char-liter〉::= ‘'’〈character〉‘'’
| 〈str-liter〉  //〈str-liter〉::= ‘"’〈character〉* ‘"’
| 〈pair-liter〉 //〈pair-liter〉::= ‘null’
| 〈ident〉
| 〈array-elem〉
| 〈unary-oper〉〈expr〉          //〈unary-oper〉::= ‘!’ | ‘-’ | ‘len’ | ‘ord’ | ‘chr’
| 〈expr〉〈binary-oper〉〈expr〉  //〈binary-oper〉::= ‘*’ | ‘/’ | ‘%’ | ‘+’ | ‘-’ | ‘>’ | ‘>=’ | ‘<’ | ‘<=’ | ‘==’ | ‘!=’ | ‘&&’ | ‘||’
| ‘(’〈expr〉‘)’ */
pub fn expr(input: &str) -> IResult<&str, Expr, ErrorTree<&str>> {
  expr_binary_app(BINARY_OP_MAX_PREC, input)
}

fn expr_atom(input: &str) -> IResult<&str, Expr, ErrorTree<&str>> {
  let bool_liter = alt((
    value(Expr::BoolLiter(true), tok("true")),
    value(Expr::BoolLiter(false), tok("false")),
  ));

  let char_liter = ws(delimited(
    tag("'"),
    map(character, Expr::CharLiter),
    tag("'"),
  ));

  let str_liter = ws(delimited(
    tag("\""),
    map(many0(character), |cs| {
      Expr::StrLiter(cs.iter().collect::<String>())
    }),
    tag("\""),
  ));

  let unary_app = map(pair(unary_oper, expr), |(op, expr)| {
    Expr::UnaryApp(op, Box::new(expr))
  });

  alt((
    map(int_liter, Expr::IntLiter),
    bool_liter,
    char_liter,
    str_liter,
    value(Expr::PairLiter, tok("null")),
    map(array_elem, Expr::ArrayElem),
    unary_app,
    map(ident, Expr::Ident),
    delimited(tok("("), expr, tok(")")),
  ))(input)
}

fn expr_binary_app(prec: u8, input: &str) -> IResult<&str, Expr, ErrorTree<&str>> {
  if prec == 0 {
    return expr_atom(input);
  }

  let term = |i| expr_binary_app(prec - 1, i);

  let (mut input, mut lhs) = term(input)?;

  while let Ok((i, (op, rhs))) = pair(binary_oper_prec(prec), term)(input) {
    input = i;
    lhs = Expr::BinaryApp(Box::new(lhs), op, Box::new(rhs));
  }

  Ok((input, lhs))
}

//〈int-liter〉::= (‘+’ | ‘-’) ? (‘0’-‘9’)
fn int_liter(input: &str) -> IResult<&str, i32, ErrorTree<&str>> {
  use nom_supreme::error::{BaseErrorKind, Expectation};
  use std::convert::TryFrom;

  let (input, (sign, digits)) = pair(opt(ws(alt((char_('+'), char_('-'))))), ws(digit1))(input)?;

  /* Use builtin i32 parsing for digits. */
  let n = digits.parse::<i64>().map_err(|_| {
    nom::Err::Error(ErrorTree::Base {
      location: input,
      kind: BaseErrorKind::Expected(Expectation::Tag("could not parse digit")),
    })
  })?;

  /* Negate if negative sign present. */
  let n: i32 = i32::try_from(if sign == Some('-') { -n } else { n }).map_err(|_| {
    nom::Err::Error(ErrorTree::Base {
      location: input,
      kind: BaseErrorKind::Expected(Expectation::Tag("couldn't negate literal")),
    })
  })?;

  Ok((input, n))
}

/* 〈unary-oper〉::= ‘!’ | ‘-’ | ‘len’ | ‘ord’ | ‘chr’ */
fn unary_oper(input: &str) -> IResult<&str, UnaryOper, ErrorTree<&str>> {
  alt((
    value(UnaryOper::Bang, tok("!")),
    value(UnaryOper::Neg, tok("-")),
    value(UnaryOper::Len, key("len")),
    value(UnaryOper::Ord, key("ord")),
    value(UnaryOper::Chr, key("chr")),
  ))(input)
}

fn binary_oper_prec<'a>(
  prec: u8,
) -> impl FnMut(&'a str) -> IResult<&'a str, BinaryOper, ErrorTree<&str>> {
  use BinaryOper::*;

  move |input| match prec {
    1 => alt((
      value(Mul, tok("*")),
      value(Div, tok("/")),
      value(Mod, tok("%")),
    ))(input),
    2 => alt((value(Add, tok("+")), value(Sub, tok("-"))))(input),
    3 => alt((
      value(Gte, tok(">=")),
      value(Lte, tok("<=")),
      value(Gt, tok(">")),
      value(Lt, tok("<")),
    ))(input),
    4 => alt((value(Eq, tok("==")), value(Neq, tok("!="))))(input),
    5 => value(And, tok("&&"))(input),
    6 => value(Or, tok("||"))(input),
    _ => unreachable!("No binary"),
  }
}

/* 〈array-elem〉::=〈ident〉(‘[’〈expr〉‘]’)+ */
pub fn array_elem(input: &str) -> IResult<&str, ArrayElem, ErrorTree<&str>> {
  let (input, id) = ident(input)?;

  /* Gets the exprs to be indexed. */
  /* This matches many times because we might have arr[x][y][z], which has
  multiple expressions. (=> ArrayElem("arr", [x, y, z])) */
  let (input, exprs) = many1(delimited(tok("["), expr, tok("]")))(input)?;

  Ok((input, ArrayElem(id, exprs)))
}

/* 〈character〉 ::= any-ASCII-character-except-‘\’-‘'’-‘"’
| ‘\’ 〈escaped-char〉 */
fn character(input: &str) -> IResult<&str, char, ErrorTree<&str>> {
  /* 〈escaped-char〉::= ‘0’ | ‘b’ | ‘t’ | ‘n’ | ‘f’ | ‘r’ | ‘"’ | ‘'’ | ‘\’ */
  let escaped_char = alt((
    value('\0', tag("0")),
    value('\u{8}', tag("b")),
    value('\t', tag("t")),
    value('\n', tag("n")),
    value('\u{c}', tag("f")),
    value('\r', tag("r")),
    value('\"', tag("\"")),
    value('\'', tag("'")),
    value('\\', tag("\\")),
  ));

  alt((none_of("\\'\""), preceded(tag("\\"), escaped_char)))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_expr() {
    assert!(matches!(expr("true   "), Ok(("", Expr::BoolLiter(true)))));
    assert!(matches!(
      expr("( false)   "),
      Ok(("", Expr::BoolLiter(false)))
    ));
    assert!(matches!(expr("( - 5321)"), Ok(("", Expr::IntLiter(-5321)))));
    assert!(matches!(expr("+523"), Ok(("", Expr::IntLiter(523)))));
    assert!(matches!(expr("1"), Ok(("", Expr::IntLiter(1)))));
    assert!(matches!(expr("'a'"), Ok(("", Expr::CharLiter('a')))));
    assert!(matches!(expr("'\\n'"), Ok(("", Expr::CharLiter('\n')))));
    assert!(matches!(expr("'\\b'"), Ok(("", Expr::CharLiter('\u{8}')))));
    assert!(matches!(
      expr("\"hello\""),
      Ok(("", ast)) if ast == Expr::StrLiter(String::from("hello"))
    ));
    assert!(matches!(
      expr("\"hello\n\""),
      Ok(("", ast)) if ast == Expr::StrLiter(String::from("hello\n"))
    ));
    assert!(matches!(expr("\"\""), Ok(("", ast)) if ast == Expr::StrLiter(String::from(""))));
    assert!(matches!(expr("null"), Ok(("", ast)) if ast == Expr::PairLiter));
    assert!(matches!(expr("null  5"), Ok(("5", Expr::PairLiter))));
    assert!(matches!(expr("hello "), Ok(("", ast)) if ast == Expr::Ident(String::from("hello"))));
    assert!(matches!(
      expr("hello  5"),
      Ok(("5", ast)) if ast == Expr::Ident(String::from("hello"))
    ));
    assert!(matches!(
      expr("hello [ 2] "),
      Ok((
        "",
        ast)) if ast == Expr::ArrayElem(ArrayElem(String::from("hello"), vec!(Expr::IntLiter(2)),
      ))
    ));
    assert!(matches!(
      expr("- (5)"),
      Ok((
        "",
        ast)) if ast == Expr::UnaryApp(UnaryOper::Neg, Box::new(Expr::IntLiter(5)
      ))
    ));
    assert!(matches!(
      expr("ord 'a'"),
      Ok((
        "",
        ast)) if ast == Expr::UnaryApp(UnaryOper::Ord, Box::new(Expr::CharLiter('a')
      ))
    ));
    assert!(matches!(
      expr("5 + 5"),
      Ok((
        "",
        ast)) if ast == Expr::BinaryApp(
          Box::new(Expr::IntLiter(5)),
          BinaryOper::Add,
          Box::new(Expr::IntLiter(5)),
        )
    ));
    assert!(matches!(
      expr("5 && false"),
      Ok((
        "",
        ast)) if ast == Expr::BinaryApp(
          Box::new(Expr::IntLiter(5)),
          BinaryOper::And,
          Box::new(Expr::BoolLiter(false)),
        )
    ));

    let e1 = expr("1 * (2 + 3)");
    let e2 = expr("(1 * 2) + 3");
    assert!(e1.is_ok());
    assert!(e2.is_ok());
    assert_ne!(e1.unwrap(), e2.unwrap());

    assert!(matches!(
      expr("1 * (2 + 3)"),
      Ok((
        "",
        ast)) if ast == Expr::BinaryApp(
          Box::new(Expr::IntLiter(1)),
          BinaryOper::Mul,
          Box::new(Expr::BinaryApp(
            Box::new(Expr::IntLiter(2)),
            BinaryOper::Add,
            Box::new(Expr::IntLiter(3)),
          )),
        )
    ));

    assert!(matches!(expr("lenx"), Ok(("", ast)) if ast == Expr::Ident("lenx".to_string())));
  }

  #[test]
  fn test_unary_oper() {
    assert!(matches!(unary_oper("!"), Ok(("", UnaryOper::Bang))));
    assert!(matches!(unary_oper("-"), Ok(("", UnaryOper::Neg))));
    assert!(matches!(unary_oper("len"), Ok(("", UnaryOper::Len))));
    assert!(matches!(unary_oper("ord"), Ok(("", UnaryOper::Ord))));
    assert!(matches!(unary_oper("chr"), Ok(("", UnaryOper::Chr))));
  }

  #[test]
  fn test_expr_binary_app_sum_mult() {
    let x = expr("w + x * y + z");
    assert!(matches!(
      x,
      Ok((
        "",
        ast)) if ast == Expr::BinaryApp(
          Box::new(Expr::BinaryApp(
            Box::new(Expr::Ident("w".to_string())),
            BinaryOper::Add,
            Box::new(Expr::BinaryApp(
              Box::new(Expr::Ident("x".to_string())),
              BinaryOper::Mul,
              Box::new(Expr::Ident("y".to_string()))
            )),
          )),
          BinaryOper::Add,
          Box::new(Expr::Ident("z".to_string())),
        )
    ))
  }

  #[test]
  fn test_expr_binary_app_add_products() {
    assert!(matches!(
      expr("w * x + y * z"),
      Ok((
        "",
        ast)) if ast == Expr::BinaryApp(
          Box::new(Expr::BinaryApp(
            Box::new(Expr::Ident("w".to_string())),
            BinaryOper::Mul,
            Box::new(Expr::Ident("x".to_string())),
          )),
          BinaryOper::Add,
          Box::new(Expr::BinaryApp(
            Box::new(Expr::Ident("y".to_string())),
            BinaryOper::Mul,
            Box::new(Expr::Ident("z".to_string())),
          )),
        )
    ))
  }

  #[test]
  fn test_expr_binary_app_products_eq() {
    assert!(matches!(
      expr("w * x == y * z"),
      Ok((
        "",
        ast)) if ast == Expr::BinaryApp(
          Box::new(Expr::BinaryApp(
            Box::new(Expr::Ident("w".to_string())),
            BinaryOper::Mul,
            Box::new(Expr::Ident("x".to_string())),
          )),
          BinaryOper::Eq,
          Box::new(Expr::BinaryApp(
            Box::new(Expr::Ident("y".to_string())),
            BinaryOper::Mul,
            Box::new(Expr::Ident("z".to_string())),
          )),
        )
    ))
  }

  #[test]
  fn test_expr_binary_app_brackets() {
    assert!(matches!(
      expr("w * (x == y) * z"),
      Ok((
        "",
        ast)) if ast == Expr::BinaryApp(
          Box::new(Expr::BinaryApp(
            Box::new(Expr::Ident("w".to_string())),
            BinaryOper::Mul,
            Box::new(Expr::BinaryApp(
              Box::new(Expr::Ident("x".to_string())),
              BinaryOper::Eq,
              Box::new(Expr::Ident("y".to_string()))
            )),
          )),
          BinaryOper::Mul,
          Box::new(Expr::Ident("z".to_string())),
        )
    ))
  }

  #[test]
  fn test_expr_binary_app_brackets_desc() {
    assert!(matches!(
      expr("w * (x + (y == z))"),
      Ok((
        "",
        ast)) if ast == Expr::BinaryApp(
          Box::new(Expr::Ident("w".to_string())),
          BinaryOper::Mul,
          Box::new(Expr::BinaryApp(
            Box::new(Expr::Ident("x".to_string())),
            BinaryOper::Add,
            Box::new(Expr::BinaryApp(
              Box::new(Expr::Ident("y".to_string())),
              BinaryOper::Eq,
              Box::new(Expr::Ident("z".to_string()))
            ))
          )),
        )
    ))
  }

  #[test]
  fn test_array_elem() {
    assert!(matches!(
      array_elem("array[2]"),
      Ok(("", ast)) if ast == ArrayElem("array".to_string(), vec!(Expr::IntLiter(2)))
    ));

    assert!(matches!(
      array_elem("otherArray[1][2]"),
      Ok((
        "",
        ast)) if ast == ArrayElem(
          "otherArray".to_string(),
          vec!(Expr::IntLiter(1), Expr::IntLiter(2))
        )
    ));
  }

  #[test]
  fn test_binary_oper() {
    assert!(matches!(
      binary_oper_prec(1)("*"),
      Ok(("", BinaryOper::Mul))
    ));
    assert!(matches!(
      binary_oper_prec(1)("/  "),
      Ok(("", BinaryOper::Div))
    ));
    assert!(matches!(
      binary_oper_prec(1)("%"),
      Ok(("", BinaryOper::Mod))
    ));
    assert!(matches!(
      binary_oper_prec(2)("+"),
      Ok(("", BinaryOper::Add))
    ));
    assert!(matches!(
      binary_oper_prec(2)("-  "),
      Ok(("", BinaryOper::Sub))
    ));
    assert!(matches!(binary_oper_prec(3)(">"), Ok(("", BinaryOper::Gt))));
    assert!(matches!(
      binary_oper_prec(3)(">="),
      Ok(("", BinaryOper::Gte))
    ));
    assert!(matches!(binary_oper_prec(3)("<"), Ok(("", BinaryOper::Lt))));
    assert!(matches!(
      binary_oper_prec(3)("<="),
      Ok(("", BinaryOper::Lte))
    ));
    assert!(matches!(
      binary_oper_prec(4)("== "),
      Ok(("", BinaryOper::Eq))
    ));
    assert!(matches!(
      binary_oper_prec(4)("!="),
      Ok(("", BinaryOper::Neq))
    ));
    assert!(matches!(
      binary_oper_prec(5)("&&"),
      Ok(("", BinaryOper::And))
    ));
    assert!(matches!(
      binary_oper_prec(6)("||"),
      Ok(("", BinaryOper::Or))
    ));

    assert!(matches!(
      binary_oper_prec(3)("< ="),
      Ok(("=", BinaryOper::Lt))
    ));
    assert!(matches!(
      binary_oper_prec(3)("> ="),
      Ok(("=", BinaryOper::Gt))
    ));
  }
}
