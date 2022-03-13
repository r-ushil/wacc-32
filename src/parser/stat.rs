extern crate nom;
use nom::{
  branch::alt,
  combinator::{map, value},
  sequence::{preceded, tuple},
  IResult,
};
use nom_supreme::error::{ErrorTree, Expectation};

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

  let declaration = |input| {
    let (input, (t, lhs_expr, _, ass)) =
      tuple((type_, expr, tok("="), assign_rhs))(input)?;

    /* RULE: local variables must not start with an upper case. */
    if let Expr::Ident(id) = &lhs_expr {
      if id.chars().nth(0).unwrap().is_uppercase() {
        return Err(nom::Err::Error(nom_supreme::error::ErrorTree::Base {
          location: input,
          kind: nom_supreme::error::BaseErrorKind::Expected(Expectation::Tag(
            "Variable names cannot start with an upper case character.",
          )),
        }));
      }
    }

    Ok((input, Stat::Declaration(t, lhs_expr, ass)))
  };

  let assignment = map(
    tuple((assign_lhs, tok("="), assign_rhs)),
    |(ass_lhs, _, ass_rhs)| Stat::Assignment(ass_lhs, Type::default(), ass_rhs),
  );

  let read = map(preceded(tok("read"), assign_lhs), |e| {
    Stat::Read(Type::default(), e)
  });

  let free = map(preceded(tok("free"), expr), |e| {
    Stat::Free(TypedExpr::new(e))
  });

  let return_ = map(preceded(tok("return"), expr), Stat::Return);

  let exit = map(preceded(tok("exit"), expr), Stat::Exit);

  let print = map(preceded(tok("print"), expr), |e| {
    Stat::Print(TypedExpr::new(e))
  });

  let println = map(preceded(tok("println"), expr), |e| {
    Stat::Println(TypedExpr::new(e))
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
  let (input, lhs_expr) = expr(input)?;

  let lhs = match lhs_expr {
    Expr::StructElem(elem) => AssignLhs::StructElem(elem),
    _ => AssignLhs::Expr(lhs_expr),
  };

  Ok((input, lhs))
}

fn assign_rhs(input: &str) -> IResult<&str, Expr, ErrorTree<&str>> {
  /* EXCEPTION: Expressions may be preceded by 'call' if they
  are a function call. */
  if let Ok((input, e @ Expr::Call(_, _, _))) =
    preceded(tok("call"), expr)(input)
  {
    Ok((input, e))
  } else {
    expr(input)
  }
}

/* pair-elem ::= 'fst' <expr> | 'snd' <expr> */
pub fn pair_elem(input: &str) -> IResult<&str, PairElem, ErrorTree<&str>> {
  ws(alt((
    map(preceded(tok("fst"), expr), |e| {
      PairElem::Fst(TypedExpr::new(e))
    }),
    map(preceded(tok("snd"), expr), |e| {
      PairElem::Snd(TypedExpr::new(e))
    }),
  )))(input)
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use super::*;

  #[test]
  fn test_struct_declaration() {
    let statement = stat("IntBox box = IntBox { x : 5 }").unwrap().1;

    let (t, id, rhs) = match statement {
      Stat::Declaration(t, id, rhs) => (t, id, rhs),
      _ => panic!("Statement should be declaration."),
    };

    assert_eq!(t, Type::Custom(format!("IntBox")));
    assert_eq!(id, Expr::Ident(format!("box")));
    assert_eq!(
      rhs,
      (Expr::StructLiter(StructLiter {
        id: format!("IntBox"),
        fields: HashMap::from([(format!("x"), Expr::IntLiter(5))])
      }))
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
          Expr::Ident("interesting".to_string()),
          Expr::IntLiter(5),
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
          Expr::Ident("x".to_string()),
          Expr::IntLiter(5),
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
          Expr::Ident("x".to_string()),
          Expr::BoolLiter(true),
        )
    ));

    assert!(matches!(
      stat("bool x = false"),
      Ok((
        "",
        ast)) if ast == Stat::Declaration(
          Type::Bool,
          Expr::Ident("x".to_string()),
          Expr::BoolLiter(false),
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
          Expr::Ident("x".to_string()),
          Expr::CharLiter('a'),
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
          Expr::Ident("x".to_string()),
          Expr::CharLiter('\n'),
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
          Expr::Ident("x".to_string()),
          Expr::StrLiter("hello world!".to_string()),
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
          Expr::Ident("arr".to_string()),
          Expr::ArrayLiter(ArrayLiter(Type::default(), (1..=5).map(Expr::IntLiter).collect()))
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
          Expr::Ident("arr".to_string()),
          Expr::ArrayLiter(ArrayLiter(Type::default(), ('a'..='e').map(Expr::CharLiter).collect()))
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
          Expr::Ident("arr".to_string()),
          Expr::ArrayLiter(ArrayLiter(Type::default(), vec![
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
          Expr::Ident("arr".to_string()),
          Expr::ArrayLiter(ArrayLiter(Type::default(), vec![Expr::NullPairLiter; 3]))
        )
    ));
  }

  #[test]
  fn test_stat_dec_pair_int_int() {
    assert_eq!(
      stat("pair(int, int) x = newpair(1,2)").unwrap().1,
      Stat::Declaration(
        Type::Pair(Box::new(Type::Int), Box::new(Type::Int)),
        Expr::Ident("x".to_string()),
        Expr::PairLiter(
          Box::new(TypedExpr::new(Expr::IntLiter(1))),
          Box::new(TypedExpr::new(Expr::IntLiter(2)))
        ),
      )
    );
  }

  #[test]
  fn test_stat_dec_pair_pair() {
    assert_eq!(
      stat("pair(pair, pair) x = newpair(null, null)").unwrap().1,
      Stat::Declaration(
        Type::Pair(
          Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any))),
          Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any)))
        ),
        Expr::Ident("x".to_string()),
        Expr::PairLiter(
          Box::new(TypedExpr::new(Expr::NullPairLiter)),
          Box::new(TypedExpr::new(Expr::NullPairLiter))
        ),
      )
    );
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
          Expr::Ident("x".to_string()),
          Expr::NullPairLiter,
        )
    ));
  }

  #[test]
  fn test_stat_dec_pair_int_pair() {
    assert_eq!(
      stat("pair(int, pair) x = newpair(1,null)").unwrap().1,
      Stat::Declaration(
        Type::Pair(
          Box::new(Type::Int),
          Box::new(Type::Pair(Box::new(Type::Any), Box::new(Type::Any)))
        ),
        Expr::Ident("x".to_string()),
        Expr::PairLiter(
          Box::new(TypedExpr::new(Expr::IntLiter(1))),
          Box::new(TypedExpr::new(Expr::NullPairLiter))
        )
      )
    );
  }

  // TODO: https://gitlab.doc.ic.ac.uk/lab2122_spring/WACC_32/-/issues/2
  #[test]
  fn test_stat_ass_idtype() {
    assert!(matches!(
      stat("intx = 5"),
      Ok((
        "",
        ast)) if ast == Stat::Assignment(
          AssignLhs::Expr(Expr::Ident("intx".to_string())),
          Type::default(),
          Expr::IntLiter(5)
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
          Expr::Ident("arr".to_string()),
          Expr::ArrayLiter(ArrayLiter(Type::default(), (1..=5).map(Expr::IntLiter).collect()))
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
          AssignLhs::Expr(Expr::Ident("aaa".to_string())),
          Type::default(),
          Expr::IntLiter(123)
        )
    ));
  }

  #[test]
  fn test_stat_ass_array_pair() {
    assert_eq!(
      stat("array[2] = newpair (1, 'a') restOfString").unwrap(),
      (
        "restOfString",
        Stat::Assignment(
          AssignLhs::Expr(Expr::ArrayElem(ArrayElem(
            "array".to_string(),
            vec!(Expr::IntLiter(2))
          ))),
          Type::default(),
          Expr::PairLiter(
            Box::new(TypedExpr::new(Expr::IntLiter(1))),
            Box::new(TypedExpr::new(Expr::CharLiter('a')))
          )
        )
      )
    );
  }

  #[test]
  fn test_stat_read() {
    assert_eq!(
      stat("read test").unwrap().1,
      Stat::Read(
        Type::default(),
        AssignLhs::Expr(Expr::Ident("test".to_string()))
      )
    );
  }

  #[test]
  fn test_stat_free() {
    assert_eq!(
      stat("free 5").unwrap().1,
      Stat::Free(TypedExpr::new(Expr::IntLiter(5)))
    );
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
    assert_eq!(
      stat("print 5").unwrap().1,
      Stat::Print(TypedExpr::new(Expr::IntLiter(5)))
    );
    assert_eq!(
      stat("print \"hello\"").unwrap().1,
      Stat::Print(TypedExpr::new(Expr::StrLiter("hello".to_string())))
    );
  }

  #[test]
  fn test_stat_println() {
    assert_eq!(
      stat("println 5").unwrap().1,
      Stat::Println(TypedExpr::new(Expr::IntLiter(5)))
    );
    assert_eq!(
      stat("println \"hello\"").unwrap().1,
      Stat::Println(TypedExpr::new(Expr::StrLiter("hello".to_string())))
    );
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
            AssignLhs::Expr(Expr::Ident("x".to_string())),
            Type::default(),
            Expr::IntLiter(5),
          )),
          ScopedStat::new(Stat::Assignment(
            AssignLhs::Expr(Expr::Ident("x".to_string())),
            Type::default(),
            Expr::IntLiter(6),
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
              AssignLhs::Expr(Expr::Ident("acc".to_string())),
              Type::default(),
              Expr::BinaryApp(
                Box::new(Expr::Ident("acc".to_string())),
                BinaryOper::Mul,
                Box::new(Expr::Ident("n".to_string())),
              ),
            )),
            Box::new(Stat::Assignment(
              AssignLhs::Expr(Expr::Ident("n".to_string())),
              Type::default(),
              Expr::BinaryApp(
                Box::new(Expr::Ident("n".to_string())),
                BinaryOper::Sub,
                Box::new(Expr::IntLiter(1)),
              ),
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
          Expr::Ident(String::from("bools")),
          Expr::ArrayLiter(ArrayLiter(Type::default(), vec!(
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
          AssignLhs::Expr(Expr::Ident("a".to_string())),
          Type::default(),
          Expr::BinaryApp(
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
        )
    ));
  }

  #[test]
  fn test_pair_elem() {
    assert_eq!(
      pair_elem("fst 5").unwrap().1,
      PairElem::Fst(TypedExpr::new(Expr::IntLiter(5)))
    );
    assert_eq!(
      pair_elem("snd null").unwrap().1,
      PairElem::Snd(TypedExpr::new(Expr::NullPairLiter))
    );
  }

  #[test]
  fn test_assign_lhs() {
    assert_eq!(
      assign_lhs("foo").unwrap().1,
      AssignLhs::Expr(Expr::Ident("foo".to_string()))
    );
    assert_eq!(
      assign_lhs("foo [ 5]").unwrap().1,
      AssignLhs::Expr(Expr::ArrayElem(ArrayElem(
        "foo".to_string(),
        vec!(Expr::IntLiter(5))
      ))),
    );
    assert_eq!(
      assign_lhs("fst 5").unwrap().1,
      AssignLhs::Expr(Expr::PairElem(Box::new(PairElem::Fst(TypedExpr::new(
        Expr::IntLiter(5)
      )))))
    );
    assert_eq!(
      assign_lhs("snd null").unwrap().1,
      AssignLhs::Expr(Expr::PairElem(Box::new(PairElem::Snd(TypedExpr::new(
        Expr::NullPairLiter
      )))))
    );
  }

  #[test]
  fn test_assign_rhs1() {
    assert!(matches!(assign_rhs("5"), Ok(("", Expr::IntLiter(5)))));
  }

  #[test]
  fn test_assign_rhs2() {
    assert!(matches!(
      assign_rhs("[1, 2 ,3 ,4,5]"),
      Ok((
        "",
        ast)) if ast == (Expr::ArrayLiter(ArrayLiter(Type::default(), (1..=5).map(Expr::IntLiter).collect())))
    ));
  }

  #[test]
  fn test_assign_rhs3() {
    assert!(matches!(
      assign_rhs("[1, 'c']"),
      Ok((
        "",
        ast)) if ast == (Expr::ArrayLiter(ArrayLiter(Type::default(), vec!(Expr::IntLiter(1), Expr::CharLiter('c')))))
    ));
  }

  #[test]
  fn test_assign_rhs4() {
    assert!(matches!(
      assign_rhs("[]"),
      Ok(("", ast)) if ast == (Expr::ArrayLiter(ArrayLiter(Type::default(), vec!())))
    ));
  }
  #[test]
  fn test_assign_rhs5() {
    assert_eq!(
      assign_rhs("newpair (1, 2)").unwrap().1,
      (Expr::PairLiter(
        Box::new(TypedExpr::new(Expr::IntLiter(1))),
        Box::new(TypedExpr::new(Expr::IntLiter(2)))
      ))
    );
  }

  #[test]
  fn test_assign_rhs9() {
    assert_eq!(
      assign_rhs("call callee ()").unwrap().1,
      (Expr::Call(
        Type::default(),
        Box::new(Expr::Ident("callee".to_string())),
        vec!()
      ))
    );
  }
  #[test]
  fn test_assign_rhs10() {
    assert!(matches!(
      assign_rhs("[ false, true ]"),
      Ok((
        "",
        ast)) if ast == (Expr::ArrayLiter(ArrayLiter(Type::default(), vec!(
          Expr::BoolLiter(false),
          Expr::BoolLiter(true)
        ))
      ))
    ));
  }
}
