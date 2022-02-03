extern crate nom;
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{alpha1, alphanumeric1, char as char_, digit1, multispace0, none_of},
  combinator::{map, opt, recognize, value},
  error::ParseError,
  multi::{many0, many1},
  sequence::{delimited, pair, preceded, terminated, tuple},
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
  delimited(multispace0, inner, multispace0)
}

pub fn tok<'a>(t: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
  delimited(multispace0, tag(t), multispace0)
}

/* ======= PARSERS ======= */
/* The parser which parses a string into a AST node of type
ExampleNode, will have the name example_node. */
/* If names conflict with Rust keywords, an underscore is appended. */
/* All parsers will consume all leading whitespace before and after parsing. */

/* program ::= 'begin' <func>* <stat> 'end' */
pub fn program(input: &str) -> IResult<&str, Program> {
  let (input, (funcs, statement)) =
    delimited(tok("begin"), pair(many0(func), stat), tok("end"))(input)?;

  Ok((input, Program { funcs, statement }))
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
  alt((stat_multiple, stat_unit))(input)
}

fn stat_unit(input: &str) -> IResult<&str, Stat> {
  let skip = value(Stat::Skip, tok("skip"));
  let declaration = map(
    tuple((type_, ident, tok("="), assign_rhs)),
    |(t, id, _, ass)| Stat::Declaration(t, id, ass),
  );
  let assignment = map(
    tuple((assign_lhs, tok("="), assign_rhs)),
    |(ass_lhs, _, ass_rhs)| Stat::Assignment(ass_lhs, ass_rhs),
  );
  let read = map(preceded(tok("read"), assign_lhs), Stat::Read);

  let free = map(preceded(tok("free"), expr), Stat::Free);
  let return_ = map(preceded(tok("return"), expr), Stat::Return);
  let exit = map(preceded(tok("exit"), expr), Stat::Exit);
  let print = map(preceded(tok("print"), expr), Stat::Print);
  let println = map(preceded(tok("println"), expr), Stat::Println);

  let if_ = map(
    tuple((
      tok("if"),
      expr,
      tok("then"),
      stat,
      tok("else"),
      stat,
      tok("fi"),
    )),
    |(_, e, _, stat_if, _, stat_else, _)| Stat::If(e, Box::new(stat_if), Box::new(stat_else)),
  );

  let while_ = map(
    tuple((tok("while"), expr, tok("do"), stat, tok("done"))),
    |(_, e, _, s, _)| Stat::While(e, Box::new(s)),
  );

  let begin = map(tuple((tok("begin"), stat, tok("end"))), |(_, s, _)| {
    Stat::Scope(Box::new(s))
  });

  alt((
    skip,
    declaration,
    assignment,
    read,
    free,
    return_,
    exit,
    println,
    print,
    if_,
    while_,
    begin,
  ))(input)
}

fn stat_multiple(input: &str) -> IResult<&str, Stat> {
  map(tuple((stat_unit, tok(";"), stat)), |(s1, _, s2)| {
    Stat::Sequence(Box::new(s1), Box::new(s2))
  })(input)
}

/* assign-lhs ::= <ident> | <array-elem> | <pair-elem> */
fn assign_lhs(input: &str) -> IResult<&str, AssignLhs> {
  alt((
    map(pair_elem, AssignLhs::PairElem),
    map(array_elem, AssignLhs::ArrayElem),
    map(ident, AssignLhs::Ident),
  ))(input)
}

/* assign-rhs ::= <expr>
| <array-liter>
| 'newpair' '(' <expr> ',' <expr> ')'
| <pair-elem>
| 'call' <ident> '(' <arg-list>? ')' */
/* arg-list ::= <expr> ( ',' <expr> )* */
fn assign_rhs(input: &str) -> IResult<&str, AssignRhs> {
  alt((
    map(
      tuple((tok("newpair"), tok("("), expr, tok(","), expr, tok(")"))),
      |(_, _, e1, _, e2, _)| AssignRhs::Pair(e1, e2),
    ),
    map(pair_elem, AssignRhs::PairElem),
    map(expr, AssignRhs::Expr),
    map(array_liter, AssignRhs::ArrayLiter),
  ))(input)
}

/* pair-elem ::= 'fst' <expr> | 'snd' <expr> */
fn pair_elem(input: &str) -> IResult<&str, PairElem> {
  ws(alt((
    map(preceded(tok("fst"), expr), PairElem::Fst),
    map(preceded(tok("snd"), expr), PairElem::Snd),
  )))(input)
}

/* type ::= <base-type> | <array-type> | <pair-type> */
fn type_(input: &str) -> IResult<&str, Type> {
  /* Parses everything apart from the trailing array notes. */
  let (input, mut t) = alt((
    map(base_type, |bt| Type::BaseType(bt)),
    map(
      tuple((
        tok("pair"),
        tok("("),
        pair_elem_type,
        tok(","),
        pair_elem_type,
        tok(")"),
      )),
      |(_, _, l, _, r, _)| Type::Pair(l, r),
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
    _ => value(PairElemType::Pair, tok("pair"))(input),
  }
}

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
fn expr(input: &str) -> IResult<&str, Expr> {
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
    },
    (input, None) => Ok((input, expr1)),
  }
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

/* 〈ident〉::= (‘_’ | ‘a’-‘z’ | ‘A’-‘Z’) (‘_’ | ‘a’-‘z’ | ‘A’-‘Z’ |
 * ‘0’-‘9’)* */
fn ident(input: &str) -> IResult<&str, Ident> {
  ws(map(
    /* Then recognise will return the part of the input that got consumed. */
    recognize(pair(
      /* The parsers in here will match the whole identifier. */
      alt((alpha1, tag("_"))),
      many0(alt((alphanumeric1, tag("_")))),
    )),
    |s: &str| Ident(s.to_string()), /* Copy string into identifier. */
  ))(input)
}

/* 〈array-elem〉::=〈ident〉(‘[’〈expr〉‘]’)+ */
fn array_elem(input: &str) -> IResult<&str, ArrayElem> {
  let (input, id) = ident(input)?;

  /* Gets the exprs to be indexed. */
  /* This matches many times because we might have arr[x][y][z], which has
  multiple expressions. (=> ArrayElem("arr", [x, y, z])) */
  let (input, exprs) = many1(delimited(tok("["), expr, tok("]")))(input)?;

  Ok((input, ArrayElem(id, exprs)))
}

//〈int-liter〉::= (‘+’ | ‘-’) ? (‘0’-‘9’)
fn int_liter(input: &str) -> IResult<&str, i32> {
  let (input, (sign, digits)) = pair(opt(ws(alt((char_('+'), char_('-'))))), digit1)(input)?;

  /* Use builtin i32 parsing for digits. */
  let n = digits.parse::<i32>().unwrap();

  /* Negate if negative sign present. */
  let n = if sign == Some('-') { -n } else { n };

  Ok((input, n))
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

/* 〈array-liter〉::= ‘[’ (〈expr〉 (‘,’〈expr〉)* )? ‘]’ */
fn array_liter(input: &str) -> IResult<&str, ArrayLiter> {
  ws(delimited(
    tok("["),
    map(
      pair(many0(terminated(expr, tok(","))), opt(expr)),
      |(mut es, oe): (Vec<Expr>, Option<Expr>)| {
        if let Some(e) = oe {
          es.push(e);
        }

        ArrayLiter(es)
      },
    ),
    tok("]"),
  ))(input)
}

pub fn main() {
  let x = expr("1");

  println!("x = {:?}", x);
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
    assert_eq!(
      param("int x"),
      Ok((
        "",
        Param(Type::BaseType(BaseType::Int), Ident("x".to_string()))
      ))
    );
    assert_eq!(
      param("int [ ][ ] x"),
      Ok((
        "",
        Param(
          Type::Array(Box::new(Type::Array(Box::new(Type::BaseType(
            BaseType::Int
          ))))),
          Ident("x".to_string())
        )
      ))
    );
  }

  #[test]
  fn test_stat() {
    assert_eq!(stat(" skip @"), Ok(("@", Stat::Skip)));

    assert_eq!(
      stat("int x = 5"),
      Ok((
        "",
        Stat::Declaration(
          Type::BaseType(BaseType::Int),
          Ident("x".to_string()),
          AssignRhs::Expr(Expr::IntLiter(5)),
        )
      ))
    );
    assert_eq!(
      stat("int[] arr = [1,2,3,4,5]"),
      Ok((
        "",
        Stat::Declaration(
          Type::Array(Box::new(Type::BaseType(BaseType::Int))),
          Ident("arr".to_string()),
          AssignRhs::ArrayLiter(ArrayLiter((1..=5).map(Expr::IntLiter).collect()))
        )
      ))
    );

    assert_eq!(
      stat("aaa = 123"),
      Ok((
        "",
        Stat::Assignment(
          AssignLhs::Ident(Ident("aaa".to_string())),
          AssignRhs::Expr(Expr::IntLiter(123))
        )
      ))
    );

    assert_eq!(
      stat("array[2] = newpair (1, 'a') restOfString"),
      Ok((
        "restOfString",
        Stat::Assignment(
          AssignLhs::ArrayElem(ArrayElem(
            Ident("array".to_string()),
            vec!(Expr::IntLiter(2))
          )),
          AssignRhs::Pair(Expr::IntLiter(1), Expr::CharLiter('a'))
        )
      ))
    );

    assert_eq!(
      stat("read test"),
      Ok(("", Stat::Read(AssignLhs::Ident(Ident("test".to_string())))))
    );

    let e1 = Expr::IntLiter(5);
    assert_eq!(stat("free 5"), Ok(("", Stat::Free(e1.clone()))));
    assert_eq!(stat("return 5"), Ok(("", Stat::Return(e1.clone()))));
    assert_eq!(stat("exit 5"), Ok(("", Stat::Exit(e1.clone()))));
    assert_eq!(stat("print 5"), Ok(("", Stat::Print(e1.clone()))));
    assert_eq!(stat("println 5"), Ok(("", Stat::Println(e1.clone()))));

    let e2 = Expr::StrLiter("hello".to_string());
    assert_eq!(stat("print \"hello\""), Ok(("", Stat::Print(e2.clone()))));
    assert_eq!(
      stat("println \"hello\""),
      Ok(("", Stat::Println(e2.clone())))
    );

    assert_eq!(
      stat("if b == 2 then x = 5 else x = 6 fi"),
      Ok((
        "",
        Stat::If(
          Expr::BinaryApp(
            Box::new(Expr::Ident(Ident("b".to_string()))),
            BinaryOper::Eq,
            Box::new(Expr::IntLiter(2)),
          ),
          Box::new(Stat::Assignment(
            AssignLhs::Ident(Ident("x".to_string())),
            AssignRhs::Expr(Expr::IntLiter(5)),
          )),
          Box::new(Stat::Assignment(
            AssignLhs::Ident(Ident("x".to_string())),
            AssignRhs::Expr(Expr::IntLiter(6)),
          )),
        )
      ))
    );

    assert_eq!(
      stat("while n != 0 do acc = acc * n; n = n - 1 done"),
      Ok((
        "",
        Stat::While(
          Expr::BinaryApp(
            Box::new(Expr::Ident(Ident("n".to_string()))),
            BinaryOper::Neq,
            Box::new(Expr::IntLiter(0)),
          ),
          Box::new(Stat::Sequence(
            Box::new(Stat::Assignment(
              AssignLhs::Ident(Ident("acc".to_string())),
              AssignRhs::Expr(Expr::BinaryApp(
                Box::new(Expr::Ident(Ident("acc".to_string()))),
                BinaryOper::Mul,
                Box::new(Expr::Ident(Ident("n".to_string()))),
              )),
            )),
            Box::new(Stat::Assignment(
              AssignLhs::Ident(Ident("n".to_string())),
              AssignRhs::Expr(Expr::BinaryApp(
                Box::new(Expr::Ident(Ident("n".to_string()))),
                BinaryOper::Sub,
                Box::new(Expr::IntLiter(1)),
              )),
            )),
          )),
        )
      ))
    );

    assert_eq!(
      stat("begin skip end"),
      Ok(("", Stat::Scope(Box::new(Stat::Skip))))
    );
  }

  #[test]
  fn test_assign_lhs() {
    assert_eq!(
      assign_lhs("foo"),
      Ok(("", AssignLhs::Ident(Ident("foo".to_string())))),
    );
    assert_eq!(
      assign_lhs("foo [ 5]"),
      Ok((
        "",
        AssignLhs::ArrayElem(ArrayElem(Ident("foo".to_string()), vec!(Expr::IntLiter(5)))),
      ))
    );
    assert_eq!(
      assign_lhs("fst 5"),
      Ok(("", AssignLhs::PairElem(PairElem::Fst(Expr::IntLiter(5))))),
    );
    assert_eq!(
      assign_lhs("snd null"),
      Ok(("", AssignLhs::PairElem(PairElem::Snd(Expr::PairLiter)))),
    )
  }

  #[test]
  fn test_assign_rhs() {
    assert_eq!(
      assign_rhs("5"),
      Ok(("", AssignRhs::Expr(Expr::IntLiter(5))))
    );
    assert_eq!(
      assign_rhs("[1, 2 ,3 ,4,5]"),
      Ok((
        "",
        AssignRhs::ArrayLiter(ArrayLiter((1..=5).map(Expr::IntLiter).collect()))
      ))
    );
    assert_eq!(
      assign_rhs("[1, 'c']"),
      Ok((
        "",
        AssignRhs::ArrayLiter(ArrayLiter(vec!(Expr::IntLiter(1), Expr::CharLiter('c'))))
      ))
    );
    assert_eq!(
      assign_rhs("[]"),
      Ok(("", AssignRhs::ArrayLiter(ArrayLiter(vec!()))))
    );
    assert_eq!(
      assign_rhs("newpair (1, 2)"),
      Ok(("", AssignRhs::Pair(Expr::IntLiter(1), Expr::IntLiter(2))))
    );

    assert_eq!(
      assign_rhs("fst 5"),
      Ok(("", AssignRhs::PairElem(PairElem::Fst(Expr::IntLiter(5)))))
    );
    assert_eq!(
      assign_rhs("snd null"),
      Ok(("", AssignRhs::PairElem(PairElem::Snd(Expr::PairLiter))))
    );
    assert_eq!(
      assign_rhs("fst 1 ; snd 2"),
      Ok((
        "; snd 2",
        AssignRhs::PairElem(PairElem::Fst(Expr::IntLiter(1)))
      ))
    );
  }

  #[test]
  fn test_pair_elem() {
    assert_eq!(
      pair_elem("fst 5"),
      Ok(("", PairElem::Fst(Expr::IntLiter(5))))
    );
    assert_eq!(
      pair_elem("snd null"),
      Ok(("", PairElem::Snd(Expr::PairLiter)))
    );
  }

  #[test]
  fn test_type_() {
    assert_eq!(type_("int"), Ok(("", Type::BaseType(BaseType::Int))),);
    assert_eq!(
      type_("pair (int [], int)[ ]"),
      Ok((
        "",
        Type::Array(Box::new(Type::Pair(
          PairElemType::Array(Box::new(Type::BaseType(BaseType::Int))),
          PairElemType::BaseType(BaseType::Int)
        )))
      ))
    );
    assert_eq!(
      type_("pair (pair , string)"),
      Ok((
        "",
        Type::Pair(PairElemType::Pair, PairElemType::BaseType(BaseType::String))
      ))
    );
    assert!(type_("pair(pair(int, int), string)").is_err());
  }

  #[test]
  fn test_pair_elem_type() {
    assert_eq!(
      pair_elem_type("int"),
      Ok(("", PairElemType::BaseType(BaseType::Int))),
    );
    assert_eq!(
      pair_elem_type("char[ ]"),
      Ok((
        "",
        PairElemType::Array(Box::new(Type::BaseType(BaseType::Char)))
      )),
    );
    assert_eq!(pair_elem_type("pair"), Ok(("", PairElemType::Pair)));
    assert_eq!(
      pair_elem_type("pair(int, int)"),
      Ok(("(int, int)", PairElemType::Pair))
    );
    assert_eq!(
      pair_elem_type("pair(int, int)[]"),
      Ok((
        "",
        PairElemType::Array(Box::new(Type::Pair(
          PairElemType::BaseType(BaseType::Int),
          PairElemType::BaseType(BaseType::Int),
        )))
      ))
    );
  }

  #[test]
  fn test_base_type() {
    assert_eq!(base_type("int"), Ok(("", BaseType::Int)));
    assert_eq!(base_type("bool"), Ok(("", BaseType::Bool)));
    assert_eq!(base_type("char"), Ok(("", BaseType::Char)));
    assert_eq!(base_type("string"), Ok(("", BaseType::String)));
  }

  #[test]
  fn test_expr() {
    assert_eq!(expr(" true   "), Ok(("", Expr::BoolLiter(true))));
    assert_eq!(expr(" ( false)   "), Ok(("", Expr::BoolLiter(false))));
    assert_eq!(expr(" ( - 5321)"), Ok(("", Expr::IntLiter(-5321))));
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
    assert_eq!(expr("  null  5"), Ok(("5", Expr::PairLiter)));
    assert_eq!(
      expr(" hello "),
      Ok(("", Expr::Ident(Ident(String::from("hello")))))
    );
    assert_eq!(
      expr(" hello  5"),
      Ok(("5", Expr::Ident(Ident(String::from("hello")))))
    );
    assert_eq!(
      expr("hello [ 2] "),
      Ok((
        "",
        Expr::ArrayElem(ArrayElem(
          Ident(String::from("hello")),
          vec!(Expr::IntLiter(2)),
        ))
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
  fn test_binary_oper() {
    assert_eq!(binary_oper("  *"), Ok(("", BinaryOper::Mul)));
    assert_eq!(binary_oper("/  "), Ok(("", BinaryOper::Div)));
    assert_eq!(binary_oper("%"), Ok(("", BinaryOper::Mod)));
    assert_eq!(binary_oper("+"), Ok(("", BinaryOper::Add)));
    assert_eq!(binary_oper("-  "), Ok(("", BinaryOper::Sub)));
    assert_eq!(binary_oper(">"), Ok(("", BinaryOper::Gt)));
    assert_eq!(binary_oper(">="), Ok(("", BinaryOper::Gte)));
    assert_eq!(binary_oper("<"), Ok(("", BinaryOper::Lt)));
    assert_eq!(binary_oper(" <="), Ok(("", BinaryOper::Lte)));
    assert_eq!(binary_oper(" == "), Ok(("", BinaryOper::Eq)));
    assert_eq!(binary_oper("!="), Ok(("", BinaryOper::Neq)));
    assert_eq!(binary_oper("&&"), Ok(("", BinaryOper::And)));
    assert_eq!(binary_oper("||"), Ok(("", BinaryOper::Or)));

    assert_eq!(binary_oper(" < ="), Ok(("=", BinaryOper::Lt)));
    assert_eq!(binary_oper("> ="), Ok(("=", BinaryOper::Gt)));
  }

  #[test]
  fn test_ident() {
    assert_eq!(ident("_hello123"), Ok(("", Ident("_hello123".to_string()))));
    assert_eq!(
      ident("_hello123 test"),
      Ok(("test", Ident("_hello123".to_string())))
    );
    assert!(ident("9test").is_err());
    assert_eq!(ident("te@st"), Ok(("@st", Ident("te".to_string()))));
  }

  #[test]
  fn test_array_elem() {
    assert_eq!(
      array_elem("array[2]"),
      Ok((
        "",
        ArrayElem(Ident("array".to_string()), vec!(Expr::IntLiter(2)))
      ))
    );

    assert_eq!(
      array_elem("otherArray[1][2]"),
      Ok((
        "",
        ArrayElem(
          Ident("otherArray".to_string()),
          vec!(Expr::IntLiter(1), Expr::IntLiter(2))
        )
      ))
    );
  }

  #[test]
  fn test_array_liter() {}
}
