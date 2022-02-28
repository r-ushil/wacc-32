use super::{
  context::{ContextLocation, Scope},
  stat::{ReturnBehaviour::*, *},
  unify::Unifiable,
  SemanticError,
};
use crate::ast::*;

fn func(scope: &Scope, errors: &mut Vec<SemanticError>, func: &Func) -> Option<()> {
  let func_scope = &mut scope.new_scope(ContextLocation::Function);

  /* Add parameters to inner scope. */
  let result = func
    .signature
    .params
    .iter()
    .try_for_each(|(pt, pi)| func_scope.insert(pi, pt.clone()));

  /* Type check function body and make sure it returns value of correct type. */

  match stat(func_scope, errors, &func.body)? {
    AtEnd(t) if t.clone().unify(func.signature.return_type.clone()) == None => {
      func_scope.add_error(
        errors,
        SemanticError::Normal(format!(
          "Function body returns {:?} but function signature expects {:?}",
          t, func.signature.return_type
        )),
      );
      return None;
    }
    AtEnd(_) => result,
    _ => {
      func_scope.add_error(
        errors,
        SemanticError::Syntax(format!("The last statement should be a return or exit.")),
      );
      return None;
    }
  }
}

/* Semantically checks an entire program. */
/* This function initialises the symbol table and function table. */
#[allow(dead_code)]
pub fn program(
  scope: &mut Scope,
  errors: &mut Vec<SemanticError>,
  program: &Program,
) -> Option<()> {
  /* Add all function signatures to global. */
  program
    .funcs
    .iter()
    .try_for_each(|func| scope.insert(&func.ident, Type::Func(Box::new(func.signature.clone()))))?;

  /* Add all functions to the symbol table. */
  let result = program
    .funcs
    .iter()
    .try_for_each(|f| func(scope, errors, f));

  /* Program body must never return, but it can exit. */
  match stat(
    &mut scope.new_scope(ContextLocation::ProgramBody),
    errors,
    &program.statement,
  )? {
    MidWay(t) | AtEnd(t) if t != Type::Any => {
      scope.add_error(
        errors,
        SemanticError::Normal(format!("Cannot have 'return' statement in main")),
      );
      None
    }
    _ => result,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn func_parameters_checked() {
    let scope = &mut Scope::new();

    /* Function */
    /* int double(int x) is return x * 2 end */
    let f = Func {
      ident: String::from("double"),
      signature: FuncSig {
        params: vec![(Type::Int, String::from("x"))],
        return_type: Type::Int,
      },
      body: Stat::Return(Expr::BinaryApp(
        Box::new(Expr::Ident(String::from("x"))),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(2)),
      )),
    };

    /* Works in it's default form. */
    assert!(func(scope, &mut vec![], &f).is_some());

    /* Doesn't work if wrong type returned. */
    /* int double(int x) is return false end */
    let mut f1 = f.clone();
    f1.body = Stat::Return(Expr::BoolLiter(false));
    assert!(func(scope, &mut vec![], &f1).is_none());

    /* Can compare parameter type with return type. */
    /* bool double(int x) is return x end */
    let mut f2 = f.clone();
    f2.signature.return_type = Type::Bool;
    f2.body = Stat::Return(Expr::Ident(String::from("x")));
    assert!(func(scope, &mut vec![], &f2).is_none());
  }

  #[test]
  fn branching_return_types_checked() {
    /* Function */
    /* int double(int x) is return x * 2 end */
    let f = Func {
      ident: String::from("double"),
      signature: FuncSig {
        params: vec![(Type::Int, String::from("x"))],
        return_type: Type::Int,
      },
      body: Stat::Return(Expr::BinaryApp(
        Box::new(Expr::Ident(String::from("x"))),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(2)),
      )),
    };

    /* Both branches of if statements must return correct type. */
    /* int double(int x) is
      if true then return 5 else return 2 fi
    end */
    let mut f3 = f.clone();
    f3.body = Stat::If(
      Expr::BoolLiter(false),
      Box::new(Stat::Return(Expr::IntLiter(5))),
      Box::new(Stat::Return(Expr::IntLiter(2))),
    );
    assert!(func(&mut Scope::new(), &mut vec![], &f3).is_some());

    /* int double(int x) is
      if true then return false else return 2 fi
    end */
    let mut f4 = f.clone();
    f4.body = Stat::If(
      Expr::BoolLiter(false),
      Box::new(Stat::Return(Expr::BoolLiter(false))),
      Box::new(Stat::Return(Expr::IntLiter(2))),
    );

    assert!(func(&mut Scope::new(), &mut vec![], &f4).is_none());

    /* Only one statement has to return. */
    /* int double(int x) is
      print "hello world"; return 5
    end*/
    let mut f5 = f.clone();
    f5.body = Stat::Sequence(
      Box::new(Stat::Print(Expr::StrLiter(String::from("hello world")))),
      Box::new(Stat::Return(Expr::IntLiter(5))),
    );
    let x = func(&mut Scope::new(), &mut vec![], &f5);
    assert!(x.is_some());

    /* Spots erroneous returns. */
    /* int double(int x) is
      if true then return true else skip fi;
      return 5
    end*/
    let mut f6 = f.clone();
    f6.body = Stat::Sequence(
      Box::new(Stat::If(
        Expr::BoolLiter(true),
        Box::new(Stat::Return(Expr::BoolLiter(true))),
        Box::new(Stat::Skip),
      )),
      Box::new(Stat::Print(Expr::StrLiter(String::from("Hello World")))),
    );
    assert!(func(&mut Scope::new(), &mut vec![], &f6).is_none());
  }
}
