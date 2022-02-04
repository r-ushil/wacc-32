extern crate nom;
use nom::{
  branch::alt,
  combinator::{map, value},
  sequence::{delimited, preceded, tuple},
  IResult,
};

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
pub fn stat(input: &str) -> IResult<&str, Stat> {
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

/* pair-elem ::= 'fst' <expr> | 'snd' <expr> */
fn pair_elem(input: &str) -> IResult<&str, PairElem> {
  ws(alt((
    map(preceded(tok("fst"), expr), PairElem::Fst),
    map(preceded(tok("snd"), expr), PairElem::Snd),
  )))(input)
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
      tuple((
        tok("call"),
        ident,
        tok("("),
        many0_delimited(expr, tok(",")),
        tok(")"),
      )),
      |(_, id, _, exprs, _)| AssignRhs::Call(id, exprs),
    ),
    map(
      tuple((tok("newpair"), tok("("), expr, tok(","), expr, tok(")"))),
      |(_, _, e1, _, e2, _)| AssignRhs::Pair(e1, e2),
    ),
    map(pair_elem, AssignRhs::PairElem),
    map(expr, AssignRhs::Expr),
    map(array_liter, AssignRhs::ArrayLiter),
  ))(input)
}

/* 〈array-liter〉::= ‘[’ (〈expr〉 (‘,’〈expr〉)* )? ‘]’ */
fn array_liter(input: &str) -> IResult<&str, ArrayLiter> {
  ws(delimited(
    tok("["),
    map(many0_delimited(expr, tok(",")), ArrayLiter),
    tok("]"),
  ))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

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

    // assert_eq!(
    //   "ifx theskipelseskipfi",
    //   format!("{:?}", stat("ifx theskipelseskipfi"))
    // );
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

    assert_eq!(
      assign_rhs("call callee ()"),
      Ok(("", AssignRhs::Call(Ident("callee".to_string()), vec!(),)))
    )
  }

  #[test]
  fn test_array_liter() {}
}
