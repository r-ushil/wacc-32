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

use super::shared::*;
use crate::ast::*;

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
pub fn expr(input: &str) -> IResult<&str, Expr> {
  let bool_liter = alt((
    value(Expr::BoolLiter(true), tok("true")),
    value(Expr::BoolLiter(false), tok("false")),
  ));

  let char_liter = ws(delimited(
    tag("'"),
    map(character, |c| Expr::CharLiter(c)),
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

  /* Run a parser for each of the enum variants except binary operators. */
  let (input, expr1) = alt((
    map(int_liter, Expr::IntLiter),
    bool_liter,
    char_liter,
    str_liter,
    value(Expr::PairLiter, tok("null")),
    map(array_elem, Expr::ArrayElem),
    unary_app,
    map(ident, Expr::Ident),
    delimited(tok("("), expr, tok(")")),
  ))(input)?;

  /* If we got this far, we know the input at least _starts with_ an expression,
  but the input might continue on in the form of a binary application. */
  /* So we try to parse a binary operator and a second expression, if it
  succeeds, combine both expressions into a binary application. */
  match opt(pair(binary_oper, expr))(input).unwrap() {
    (input, Some((op, expr2))) => {
      Ok((input, Expr::BinaryApp(Box::new(expr1), op, Box::new(expr2))))
    }
    (input, None) => Ok((input, expr1)),
  }
}

//〈int-liter〉::= (‘+’ | ‘-’) ? (‘0’-‘9’)
fn int_liter(input: &str) -> IResult<&str, i32> {
  let (input, (sign, digits)) = pair(opt(ws(alt((char_('+'), char_('-'))))), ws(digit1))(input)?;

  /* Use builtin i32 parsing for digits. */
  let n = digits.parse::<i32>().unwrap();

  /* Negate if negative sign present. */
  let n = if sign == Some('-') { -n } else { n };

  Ok((input, n))
}

/* 〈unary-oper〉::= ‘!’ | ‘-’ | ‘len’ | ‘ord’ | ‘chr’ */
fn unary_oper(input: &str) -> IResult<&str, UnaryOper> {
  alt((
    value(UnaryOper::Bang, tok("!")),
    value(UnaryOper::Neg, tok("-")),
    value(UnaryOper::Len, tok("len")),
    value(UnaryOper::Ord, tok("ord")),
    value(UnaryOper::Chr, tok("chr")),
  ))(input)
}

fn binary_oper(input: &str) -> IResult<&str, BinaryOper> {
  alt((
    value(BinaryOper::Mul, tok("*")),
    value(BinaryOper::Div, tok("/")),
    value(BinaryOper::Mod, tok("%")),
    value(BinaryOper::Add, tok("+")),
    value(BinaryOper::Sub, tok("-")),
    value(BinaryOper::Gte, tok(">=")),
    value(BinaryOper::Gt, tok(">")),
    value(BinaryOper::Lte, tok("<=")),
    value(BinaryOper::Lt, tok("<")),
    value(BinaryOper::Eq, tok("==")),
    value(BinaryOper::Neq, tok("!=")),
    value(BinaryOper::And, tok("&&")),
    value(BinaryOper::Or, tok("||")),
  ))(input)
}

/* 〈array-elem〉::=〈ident〉(‘[’〈expr〉‘]’)+ */
pub fn array_elem(input: &str) -> IResult<&str, ArrayElem> {
  let (input, id) = ident(input)?;

  /* Gets the exprs to be indexed. */
  /* This matches many times because we might have arr[x][y][z], which has
  multiple expressions. (=> ArrayElem("arr", [x, y, z])) */
  let (input, exprs) = many1(delimited(tok("["), expr, tok("]")))(input)?;

  Ok((input, ArrayElem(id, exprs)))
}

/* 〈character〉 ::= any-ASCII-character-except-‘\’-‘'’-‘"’
| ‘\’ 〈escaped-char〉 */
fn character(input: &str) -> IResult<&str, char> {
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
    assert_eq!(expr("true   "), Ok(("", Expr::BoolLiter(true))));
    assert_eq!(expr("( false)   "), Ok(("", Expr::BoolLiter(false))));
    assert_eq!(expr("( - 5321)"), Ok(("", Expr::IntLiter(-5321))));
    assert_eq!(expr("+523"), Ok(("", Expr::IntLiter(523))));
    assert_eq!(expr("1"), Ok(("", Expr::IntLiter(1))));
    assert_eq!(expr("'a'"), Ok(("", Expr::CharLiter('a'))));
    assert_eq!(expr("'\\n'"), Ok(("", Expr::CharLiter('\n'))));
    assert_eq!(expr("'\\b'"), Ok(("", Expr::CharLiter('\u{8}'))));
    assert_eq!(
      expr("\"hello\""),
      Ok(("", Expr::StrLiter(String::from("hello"))))
    );
    assert_eq!(
      expr("\"hello\n\""),
      Ok(("", Expr::StrLiter(String::from("hello\n"))))
    );
    assert_eq!(expr("\"\""), Ok(("", Expr::StrLiter(String::from("")))));
    assert_eq!(expr("null"), Ok(("", Expr::PairLiter)));
    assert_eq!(expr("null  5"), Ok(("5", Expr::PairLiter)));
    assert_eq!(expr("hello "), Ok(("", Expr::Ident(String::from("hello")))));
    assert_eq!(
      expr("hello  5"),
      Ok(("5", Expr::Ident(String::from("hello"))))
    );
    assert_eq!(
      expr("hello [ 2] "),
      Ok((
        "",
        Expr::ArrayElem(ArrayElem(String::from("hello"), vec!(Expr::IntLiter(2)),))
      ))
    );
    assert_eq!(
      expr("- (5)"),
      Ok((
        "",
        Expr::UnaryApp(UnaryOper::Neg, Box::new(Expr::IntLiter(5)),)
      ))
    );
    assert_eq!(
      expr("ord 'a'"),
      Ok((
        "",
        Expr::UnaryApp(UnaryOper::Ord, Box::new(Expr::CharLiter('a')),)
      ))
    );
    assert_eq!(
      expr("5 + 5"),
      Ok((
        "",
        Expr::BinaryApp(
          Box::new(Expr::IntLiter(5)),
          BinaryOper::Add,
          Box::new(Expr::IntLiter(5)),
        )
      ))
    );
    assert_eq!(
      expr("5 && false"),
      Ok((
        "",
        Expr::BinaryApp(
          Box::new(Expr::IntLiter(5)),
          BinaryOper::And,
          Box::new(Expr::BoolLiter(false)),
        )
      ))
    );
    assert_ne!(expr("1 * (2 + 3)"), expr("(1 * 2) + 3"));
    assert_eq!(
      expr("1 * (2 + 3)"),
      Ok((
        "",
        Expr::BinaryApp(
          Box::new(Expr::IntLiter(1)),
          BinaryOper::Mul,
          Box::new(Expr::BinaryApp(
            Box::new(Expr::IntLiter(2)),
            BinaryOper::Add,
            Box::new(Expr::IntLiter(3)),
          )),
        )
      ))
    );
  }

  #[test]
  fn test_unary_oper() {
    assert_eq!(unary_oper("!"), Ok(("", UnaryOper::Bang)));
    assert_eq!(unary_oper("-"), Ok(("", UnaryOper::Neg)));
    assert_eq!(unary_oper("len"), Ok(("", UnaryOper::Len)));
    assert_eq!(unary_oper("ord"), Ok(("", UnaryOper::Ord)));
    assert_eq!(unary_oper("chr"), Ok(("", UnaryOper::Chr)));
  }

  #[test]
  fn test_array_elem() {
    assert_eq!(
      array_elem("array[2]"),
      Ok(("", ArrayElem("array".to_string(), vec!(Expr::IntLiter(2)))))
    );

    assert_eq!(
      array_elem("otherArray[1][2]"),
      Ok((
        "",
        ArrayElem(
          "otherArray".to_string(),
          vec!(Expr::IntLiter(1), Expr::IntLiter(2))
        )
      ))
    );
  }

  #[test]
  fn test_binary_oper() {
    assert_eq!(binary_oper("*"), Ok(("", BinaryOper::Mul)));
    assert_eq!(binary_oper("/  "), Ok(("", BinaryOper::Div)));
    assert_eq!(binary_oper("%"), Ok(("", BinaryOper::Mod)));
    assert_eq!(binary_oper("+"), Ok(("", BinaryOper::Add)));
    assert_eq!(binary_oper("-  "), Ok(("", BinaryOper::Sub)));
    assert_eq!(binary_oper(">"), Ok(("", BinaryOper::Gt)));
    assert_eq!(binary_oper(">="), Ok(("", BinaryOper::Gte)));
    assert_eq!(binary_oper("<"), Ok(("", BinaryOper::Lt)));
    assert_eq!(binary_oper("<="), Ok(("", BinaryOper::Lte)));
    assert_eq!(binary_oper("== "), Ok(("", BinaryOper::Eq)));
    assert_eq!(binary_oper("!="), Ok(("", BinaryOper::Neq)));
    assert_eq!(binary_oper("&&"), Ok(("", BinaryOper::And)));
    assert_eq!(binary_oper("||"), Ok(("", BinaryOper::Or)));

    assert_eq!(binary_oper("< ="), Ok(("=", BinaryOper::Lt)));
    assert_eq!(binary_oper("> ="), Ok(("=", BinaryOper::Gt)));
  }
}
