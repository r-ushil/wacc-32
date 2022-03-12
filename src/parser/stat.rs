extern crate nom;
use nom::{
  branch::alt,
  combinator::{map, value},
  sequence::{delimited, pair, preceded, separated_pair, tuple},
  IResult,
};
use nom_supreme::error::ErrorTree;

use super::expr::*;
use super::shared::*;
use super::type_::*;
use crate::ast::*;

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
pub fn stat(input: &str) -> IResult<&str, Stat, ErrorTree<&str>> {
  alt((stat_multiple, stat_unit))(input)
}

fn stat_unit(input: &str) -> IResult<&str, Stat, ErrorTree<&str>> {
  let skip = value(Stat::Skip, tok("skip"));
  let declaration = map(
    tuple((type_, ident, tok("="), assign_rhs)),
    |(t, id, _, ass)| Stat::Declaration(t, id, ass),
  );
  let assignment = map(
    tuple((assign_lhs, tok("="), assign_rhs)),
    |(ass_lhs, _, ass_rhs)| Stat::Assignment(ass_lhs, Type::default(), ass_rhs),
  );
  let read = map(preceded(tok("read"), assign_lhs), |e| {
    Stat::Read(Type::default(), e)
  });

  let free = map(preceded(tok("free"), expr), |e| {
    Stat::Free(Type::default(), e)
  });
  let return_ = map(preceded(tok("return"), expr), Stat::Return);
  let exit = map(preceded(tok("exit"), expr), Stat::Exit);
  let print = map(preceded(tok("print"), expr), |e| {
    Stat::Print(Type::default(), e)
  });
  let println = map(preceded(tok("println"), expr), |e| {
    Stat::Println(Type::default(), e)
  });

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
    |(_, e, _, stat_if, _, stat_else, _)| {
      Stat::If(e, ScopedStat::new(stat_if), ScopedStat::new(stat_else))
    },
  );

  let while_ = map(
    tuple((tok("while"), expr, tok("do"), stat, tok("done"))),
    |(_, e, _, s, _)| Stat::While(e, ScopedStat::new(s)),
  );

  let begin = map(tuple((tok("begin"), stat, tok("end"))), |(_, s, _)| {
    Stat::Scope(ScopedStat::new(s))
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

fn stat_multiple(input: &str) -> IResult<&str, Stat, ErrorTree<&str>> {
  map(tuple((stat_unit, tok(";"), stat)), |(s1, _, s2)| {
    Stat::Sequence(Box::new(s1), Box::new(s2))
  })(input)
}

/* assign-lhs ::= <struct-elem> | <ident> | <array-elem> | <pair-elem> */
fn assign_lhs(input: &str) -> IResult<&str, AssignLhs, ErrorTree<&str>> {
  /* Attempt to parse as expression just to catch struct elem case. */
  if let Ok((input, Expr::StructElem(elem))) = expr(input) {
    return Ok((input, AssignLhs::StructElem(elem)));
  }

  alt((
    map(pair_elem, AssignLhs::PairElem),
    map(array_elem, AssignLhs::ArrayElem),
    map(ident, AssignLhs::Ident),
  ))(input)
}

/* pair-elem ::= 'fst' <expr> | 'snd' <expr> */
fn pair_elem(input: &str) -> IResult<&str, PairElem, ErrorTree<&str>> {
  ws(alt((
    map(preceded(tok("fst"), expr), |e| {
      PairElem::Fst(Type::default(), e)
    }),
    map(preceded(tok("snd"), expr), |e| {
      PairElem::Snd(Type::default(), e)
    }),
  )))(input)
}

/* assign-rhs ::= <expr>
| <array-liter>
| 'newpair' '(' <expr> ',' <expr> ')'
| <pair-elem>
| 'call' <ident> '(' <arg-list>? ')' */
/* arg-list ::= <expr> ( ',' <expr> )* */
fn assign_rhs(input: &str) -> IResult<&str, AssignRhs, ErrorTree<&str>> {
  alt((
    map(
      tuple((
        tok("call"),
        expr,
        tok("("),
        many0_delimited(expr, tok(",")),
        tok(")"),
      )),
      |(_, func, _, args, _)| AssignRhs::Call(Type::default(), func, args),
    ),
    map(
      tuple((tok("newpair"), tok("("), expr, tok(","), expr, tok(")"))),
      |(_, _, e1, _, e2, _)| AssignRhs::Pair(e1, e2),
    ),
    map(pair_elem, AssignRhs::PairElem),
    map(struct_liter, AssignRhs::StructLiter),
    map(expr, AssignRhs::Expr),
    map(array_liter, AssignRhs::ArrayLiter),
  ))(input)
}

/* <struct-liter> ::= <ident> '{' ( (<field-liter> ',')* <field-liter> )? '}' */
fn struct_liter(input: &str) -> IResult<&str, StructLiter, ErrorTree<&str>> {
  let (input, (id, fields)) = pair(
    ident,
    delimited(tok("{"), many0_delimited(field_liter, tok(",")), tok("}")),
  )(input)?;

  Ok((
    input,
    StructLiter {
      id,
      fields: fields.into_iter().collect(),
    },
  ))
}

/* <field-liter> ::= <ident> ':' <expr> */
fn field_liter(input: &str) -> IResult<&str, (Ident, Expr), ErrorTree<&str>> {
  separated_pair(ident, tok(":"), expr)(input)
}

/* 〈array-liter〉::= ‘[’ (〈expr〉 (‘,’〈expr〉)* )? ‘]’ */
fn array_liter(input: &str) -> IResult<&str, ArrayLiter, ErrorTree<&str>> {
  ws(delimited(
    tok("["),
    map(many0_delimited(expr, tok(",")), ArrayLiter),
    tok("]"),
  ))(input)
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use super::*;

  #[test]
  fn test_struct_liter() {
    let sl = struct_liter("Foo { x: 5 }").unwrap().1;

    assert_eq!(sl.id, "Foo");
    assert_eq!(sl.fields.len(), 1);
    assert_eq!(sl.fields.get("x").unwrap(), &Expr::IntLiter(5));
  }

  #[test]
  fn test_struct_declaration() {
    let statement = stat("IntBox box = IntBox { x : 5 }").unwrap().1;

    let (t, id, rhs) = match statement {
      Stat::Declaration(t, id, rhs) => (t, id, rhs),
      _ => panic!("Statement should be declaration."),
    };

    assert_eq!(t, Type::Custom(format!("IntBox")));
    assert_eq!(id, format!("box"));
    assert_eq!(
      rhs,
      AssignRhs::StructLiter(StructLiter {
        id: format!("IntBox"),
        fields: HashMap::from([(format!("x"), Expr::IntLiter(5))])
      })
    );
  }

  #[test]
  fn test_stat_skip() {
    assert!(matches!(
      stat("skip; skip"),
      Ok((
        "",
        ast)) if ast == Stat::Sequence(Box::new(Stat::Skip), Box::new(Stat::Skip))
    ));
    assert!(matches!(stat("skip"), Ok(("", Stat::Skip))));
    assert!(stat("sk ip").is_err());
    assert!(stat("skiip").is_err());
  }

  #[test]
  fn test_stat_dec_keyword() {
    assert!(matches!(
      stat("int interesting = 5"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Int,
          "interesting".to_string(),
          AssignRhs::Expr(Expr::IntLiter(5)),
        )
    ));
  }

  #[test]
  fn test_stat_dec_int() {
    assert!(matches!(
      stat("int x = 5"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Int,
          "x".to_string(),
          AssignRhs::Expr(Expr::IntLiter(5)),
        )
    ));
  }

  #[test]
  fn test_stat_dec_bool() {
    assert!(matches!(
      stat("bool x = true"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Bool,
          "x".to_string(),
          AssignRhs::Expr(Expr::BoolLiter(true)),
        )
    ));

    assert!(matches!(
      stat("bool x = false"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Bool,
          "x".to_string(),
          AssignRhs::Expr(Expr::BoolLiter(false)),
        )
    ));
  }

  #[test]
  fn test_stat_dec_char() {
    assert!(matches!(
      stat("char x = 'a'"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Char,
          "x".to_string(),
          AssignRhs::Expr(Expr::CharLiter('a')),
        )
    ));
  }

  #[test]
  fn test_stat_dec_char_escape() {
    assert!(matches!(
      stat("char x = '\n'"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Char,
          "x".to_string(),
          AssignRhs::Expr(Expr::CharLiter('\n')),
        )
    ));
  }

  #[test]
  fn test_stat_dec_str() {
    assert!(matches!(
      stat("string x = \"hello world!\""),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::String,
          "x".to_string(),
          AssignRhs::Expr(Expr::StrLiter("hello world!".to_string())),
        )
    ));
  }

  #[test]
  fn test_stat_dec_array_int() {
    assert!(matches!(
      stat("int[] arr = [1,2,3,4,5]"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Array(Box::new(Type::Int)),
          "arr".to_string(),
          AssignRhs::ArrayLiter(ArrayLiter((1..=5).map(Expr::IntLiter).collect()))
        )
    ));
  }

  #[test]
  fn test_stat_dec_array_char() {
    assert!(matches!(
      stat("char[] arr = ['a','b','c','d','e']"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Array(Box::new(Type::Char)),
          "arr".to_string(),
          AssignRhs::ArrayLiter(ArrayLiter(('a'..='e').map(Expr::CharLiter).collect()))
        )
    ));
  }

  #[test]
  fn test_stat_dec_array_string() {
    assert!(matches!(
      stat("string[] arr = [\"hello\",\"world\"]"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Array(Box::new(Type::String)),
          "arr".to_string(),
          AssignRhs::ArrayLiter(ArrayLiter(vec![
            Expr::StrLiter("hello".to_string()),
            Expr::StrLiter("world".to_string())
          ]))
        )
    ));
  }

  #[test]
  fn test_stat_dec_array_pair() {
    assert!(matches!(
      stat("pair(int, int)[] arr = [null,null,null]"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Array(Box::new(Type::Pair(
            Box::new(Type::Int),
            Box::new(Type::Int)
          ))),
          "arr".to_string(),
          AssignRhs::ArrayLiter(ArrayLiter(vec![Expr::PairLiter; 3]))
        )
    ));
  }

  #[test]
  fn test_stat_dec_pair_int_int() {
    assert!(matches!(
      stat("pair(int, int) x = newpair(1,2)"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Pair(Box::new(Type::Int), Box::new(Type::Int)),
          "x".to_string(),
          AssignRhs::Pair(Expr::IntLiter(1), Expr::IntLiter(2)),
        )
    ));
  }

  #[test]
  fn test_stat_dec_pair_pair() {
    assert!(matches!(
      stat("pair(pair, pair) x = newpair(null, null)"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Pair(
            Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any))),
            Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any)))
          ),
          "x".to_string(),
          AssignRhs::Pair(Expr::PairLiter, Expr::PairLiter),
        )
    ));
  }

  #[test]
  fn test_stat_dec_pair_null() {
    assert!(matches!(
      stat("pair(pair, pair) x = null"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Pair(
            Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any))),
            Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any)))
          ),
          "x".to_string(),
          AssignRhs::Expr(Expr::PairLiter),
        )
    ));
  }

  #[test]
  fn test_stat_dec_pair_int_pair() {
    assert!(matches!(
      stat("pair(int, pair) x = newpair(1,null)"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Pair(
            Box::new(Type::Int),
            Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any)))
          ),
          "x".to_string(),
          AssignRhs::Pair(Expr::IntLiter(1), Expr::PairLiter),
        )
    ));
  }

  // TODO: https://gitlab.doc.ic.ac.uk/lab2122_spring/WACC_32/-/issues/2
  #[test]
  fn test_stat_ass_idtype() {
    assert!(matches!(
      stat("intx = 5"),
      Ok((
        "",
        ast)) if ast == Stat::Assignment(
          AssignLhs::Ident("intx".to_string()),
          Type::default(),
          AssignRhs::Expr(Expr::IntLiter(5))
        )
    ));
  }

  #[test]
  fn test_stat_ass_arr() {
    assert!(matches!(
      stat("int[] arr = [1,2,3,4,5]"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Array(Box::new(Type::Int)),
          "arr".to_string(),
          AssignRhs::ArrayLiter(ArrayLiter((1..=5).map(Expr::IntLiter).collect()))
        )
    ));
  }

  #[test]
  fn test_stat_ass_int2() {
    assert!(matches!(
      stat("aaa = 123"),
      Ok((
        "",
        ast)) if ast == Stat::Assignment(
          AssignLhs::Ident("aaa".to_string()),
          Type::default(),
          AssignRhs::Expr(Expr::IntLiter(123))
        )
    ));
  }

  #[test]
  fn test_stat_ass_array_pair() {
    assert!(matches!(
      stat("array[2] = newpair (1, 'a') restOfString"),
      Ok((
        "restOfString",
        ast)) if ast == Stat::Assignment(
          AssignLhs::ArrayElem(ArrayElem("array".to_string(), vec!(Expr::IntLiter(2)))),
          Type::default(),
          AssignRhs::Pair(Expr::IntLiter(1), Expr::CharLiter('a'))
        )
    ));
  }

  #[test]
  fn test_stat_read() {
    assert!(matches!(
      stat("read test"),
      Ok(("", ast)) if ast == Stat::Read(Type::default(), AssignLhs::Ident("test".to_string()))
    ));
  }

  #[test]
  fn test_stat_free() {
    assert!(matches!(
      stat("free 5"),
      Ok(("", Stat::Free(Type::Any, Expr::IntLiter(5))))
    ));
  }

  #[test]
  fn test_stat_return() {
    assert!(matches!(
      stat("return 5"),
      Ok(("", Stat::Return(Expr::IntLiter(5))))
    ));
  }

  #[test]
  fn test_stat_exit() {
    assert!(matches!(
      stat("exit 5"),
      Ok(("", Stat::Exit(Expr::IntLiter(5))))
    ));
  }

  #[test]
  fn test_stat_print() {
    assert!(matches!(
      stat("print 5"),
      Ok(("", Stat::Print(Type::Any, Expr::IntLiter(5))))
    ));
    assert!(matches!(
      stat("print \"hello\""),
      Ok(("", ast)) if ast == Stat::Print(Type::Any, Expr::StrLiter("hello".to_string()))
    ));
  }

  #[test]
  fn test_stat_println() {
    assert!(matches!(
      stat("println 5"),
      Ok(("", Stat::Println(Type::Any, Expr::IntLiter(5))))
    ));
    assert!(matches!(
      stat("println \"hello\""),
      Ok(("", ast)) if ast == Stat::Println(Type::Any, Expr::StrLiter("hello".to_string()))
    ));
  }

  #[test]
  fn test_stat_if() {
    assert!(matches!(
      stat("if b == 2 then x = 5 else x = 6 fi"),
      Ok((
        "",
        ast)) if ast == Stat::If(
          Expr::BinaryApp(
            Box::new(Expr::Ident("b".to_string())),
            BinaryOper::Eq,
            Box::new(Expr::IntLiter(2)),
          ),
          ScopedStat::new(Stat::Assignment(
            AssignLhs::Ident("x".to_string()),
            Type::default(),
            AssignRhs::Expr(Expr::IntLiter(5)),
          )),
          ScopedStat::new(Stat::Assignment(
            AssignLhs::Ident("x".to_string()),
            Type::default(),
            AssignRhs::Expr(Expr::IntLiter(6)),
          )),
        )
    ));
  }

  #[test]
  fn test_stat_while() {
    assert!(matches!(
      stat("while n != 0 do acc = acc * n; n = n - 1 done"),
      Ok((
        "",
        ast)) if ast == Stat::While(
          Expr::BinaryApp(
            Box::new(Expr::Ident("n".to_string())),
            BinaryOper::Neq,
            Box::new(Expr::IntLiter(0)),
          ),
          ScopedStat::new(Stat::Sequence(
            Box::new(Stat::Assignment(
              AssignLhs::Ident("acc".to_string()),
              Type::default(),
              AssignRhs::Expr(Expr::BinaryApp(
                Box::new(Expr::Ident("acc".to_string())),
                BinaryOper::Mul,
                Box::new(Expr::Ident("n".to_string())),
              )),
            )),
            Box::new(Stat::Assignment(
              AssignLhs::Ident("n".to_string()),
              Type::default(),
              AssignRhs::Expr(Expr::BinaryApp(
                Box::new(Expr::Ident("n".to_string())),
                BinaryOper::Sub,
                Box::new(Expr::IntLiter(1)),
              )),
            )),
          )),
        )
    ));
  }

  #[test]
  fn test_stat_scope() {
    assert!(matches!(
      stat("begin skip end"),
      Ok(("", ast)) if ast == Stat::Scope(ScopedStat::new(Stat::Skip))
    ));

    // assert!(matches!(
    //   "ifx theskipelseskipfi",
    //   format!("{:?}", stat("ifx theskipelseskipfi"))
    // );

    assert!(matches!(
      stat("bool[] bools = [ false, true ]"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Array(Box::new(Type::Bool)),
          String::from("bools"),
          AssignRhs::ArrayLiter(ArrayLiter(vec!(
            Expr::BoolLiter(false),
            Expr::BoolLiter(true)
          )))
        )
    ));
  }

  #[test]
  fn test_stat_ass_binary_app_sum_mult() {
    assert!(matches!(
      stat("a = w + x * y + z"),
      Ok((
        "",
        ast)) if ast == Stat::Assignment(
          AssignLhs::Ident("a".to_string()),
          Type::default(),
          AssignRhs::Expr(Expr::BinaryApp(
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
          ))
        )
    ));
  }

  #[test]
  fn test_pair_elem() {
    assert!(matches!(
      pair_elem("fst 5"),
      Ok(("", PairElem::Fst(Type::Any, Expr::IntLiter(5))))
    ));
    assert!(matches!(
      pair_elem("snd null"),
      Ok(("", PairElem::Snd(Type::Any, Expr::PairLiter)))
    ));
  }

  #[test]
  fn test_assign_lhs() {
    assert!(matches!(
      assign_lhs("foo"),
      Ok(("", ast)) if ast == AssignLhs::Ident("foo".to_string()),
    ));
    assert!(matches!(
      assign_lhs("foo [ 5]"),
      Ok((
        "",
        ast)) if ast == AssignLhs::ArrayElem(ArrayElem("foo".to_string(), vec!(Expr::IntLiter(5)),
      ))
    ));
    assert!(matches!(
      assign_lhs("fst 5"),
      Ok((
        "",
        AssignLhs::PairElem(PairElem::Fst(Type::Any, Expr::IntLiter(5)))
      )),
    ));
    assert!(matches!(
      assign_lhs("snd null"),
      Ok((
        "",
        AssignLhs::PairElem(PairElem::Snd(Type::Any, Expr::PairLiter))
      )),
    ));
  }

  #[test]
  fn test_assign_rhs1() {
    assert!(matches!(
      assign_rhs("5"),
      Ok(("", AssignRhs::Expr(Expr::IntLiter(5))))
    ));
  }

  #[test]
  fn test_assign_rhs2() {
    assert!(matches!(
      assign_rhs("[1, 2 ,3 ,4,5]"),
      Ok((
        "",
        ast)) if ast == AssignRhs::ArrayLiter(ArrayLiter((1..=5).map(Expr::IntLiter).collect()))
    ));
  }

  #[test]
  fn test_assign_rhs3() {
    assert!(matches!(
      assign_rhs("[1, 'c']"),
      Ok((
        "",
        ast)) if ast == AssignRhs::ArrayLiter(ArrayLiter(vec!(Expr::IntLiter(1), Expr::CharLiter('c'))))
    ));
  }

  #[test]
  fn test_assign_rhs4() {
    assert!(matches!(
      assign_rhs("[]"),
      Ok(("", ast)) if ast == AssignRhs::ArrayLiter(ArrayLiter(vec!()))
    ));
  }
  #[test]
  fn test_assign_rhs5() {
    assert!(matches!(
      assign_rhs("newpair (1, 2)"),
      Ok(("", AssignRhs::Pair(Expr::IntLiter(1), Expr::IntLiter(2))))
    ));
  }

  #[test]
  fn test_assign_rhs6() {
    assert!(matches!(
      assign_rhs("fst 5"),
      Ok((
        "",
        AssignRhs::PairElem(PairElem::Fst(Type::Any, Expr::IntLiter(5)))
      ))
    ));
  }

  #[test]
  fn test_assign_rhs7() {
    assert!(matches!(
      assign_rhs("snd null"),
      Ok((
        "",
        AssignRhs::PairElem(PairElem::Snd(Type::Any, Expr::PairLiter))
      ))
    ));
  }

  #[test]
  fn test_assign_rhs8() {
    assert!(matches!(
      assign_rhs("fst 1 ; snd 2"),
      Ok((
        "; snd 2",
        AssignRhs::PairElem(PairElem::Fst(Type::Any, Expr::IntLiter(1)))
      ))
    ));
  }

  #[test]
  fn test_assign_rhs9() {
    assert_eq!(
      assign_rhs("call callee ()").unwrap().1,
      AssignRhs::Call(
        Type::default(),
        Expr::Ident("callee".to_string()),
        vec!()
      )
    );
  }
  #[test]
  fn test_assign_rhs10() {
    assert!(matches!(
      assign_rhs("[ false, true ]"),
      Ok((
        "",
        ast)) if ast == AssignRhs::ArrayLiter(ArrayLiter(vec!(
          Expr::BoolLiter(false),
          Expr::BoolLiter(true)
        )
      ))
    ));
  }
}
