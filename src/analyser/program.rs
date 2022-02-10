use super::{
  stat::{ReturnBehaviour::*, *},
  symbol_table::SymbolTable,
  AResult,
};
use crate::ast::*;

fn func(symbol_table: &mut SymbolTable, func: &Func) -> AResult<()> {
  /* Add function's type to symbol table. */
  // symbol_table.insert(func.ident, func.type_);

  /* Create new scope for function body. */
  let mut func_scope = symbol_table.new_scope();

  /* Add parameters to inner scope. */
  func
    .param_list
    .iter()
    .try_for_each(|Param(pt, pi)| func_scope.insert(pi, pt.clone()))?;

  /* Type check function body and make sure it returns value of correct type. */
  match stat(&mut func_scope, &func.body)? {
    Always(t) if t != func.return_type => Err(format!(
      "Function body returns {:?} but function signature expects {:?}",
      t, func.return_type
    )),
    Always(_) => Ok(()),
    _ => Err(format!(
      "Function must always return a value in every branch."
    )),
  }
}

/* Semantically checks an entire program. */
/* This function initialises the symbol table and function table. */
#[allow(dead_code)]
fn program(symbol_table: &mut SymbolTable, program: &Program) -> AResult<()> {
  program
    .funcs
    .iter()
    .try_for_each(|f| func(symbol_table, f))?;

  stat(symbol_table, &program.statement)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::analyser::symbol_table;

  #[test]
  fn func_parameters_checked() {
    let symbol_table = &mut SymbolTable::new();

    /* Function */
    /* int double(int x) is return x * 2 end */
    let f = Func {
      return_type: Type::BaseType(BaseType::Int),
      ident: Ident(String::from("double")),
      param_list: vec![Param(
        Type::BaseType(BaseType::Int),
        Ident(String::from("x")),
      )],
      body: Stat::Return(Expr::BinaryApp(
        Box::new(Expr::Ident(Ident(String::from("x")))),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(2)),
      )),
    };

    /* Works in it's default form. */
    assert!(func(symbol_table, &f).is_ok());

    /* Doesn't work if wrong type returned. */
    /* int double(int x) is return false end */
    let mut f1 = f.clone();
    f1.body = Stat::Return(Expr::BoolLiter(false));
    assert!(func(symbol_table, &f1).is_err());

    /* Can compare parameter type with return type. */
    /* bool double(int x) is return x end */
    let mut f2 = f.clone();
    f2.return_type = Type::BaseType(BaseType::Bool);
    f2.body = Stat::Return(Expr::Ident(Ident(String::from("x"))));
    assert!(func(symbol_table, &f2).is_err());
  }

  #[test]
  fn branching_return_types_checked() {
    let symbol_table = &mut SymbolTable::new();

    /* Function */
    /* int double(int x) is return x * 2 end */
    let f = Func {
      return_type: Type::BaseType(BaseType::Int),
      ident: Ident(String::from("double")),
      param_list: vec![Param(
        Type::BaseType(BaseType::Int),
        Ident(String::from("x")),
      )],
      body: Stat::Return(Expr::BinaryApp(
        Box::new(Expr::Ident(Ident(String::from("x")))),
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
    assert!(func(symbol_table, &f3).is_ok());

    /* int double(int x) is
      if true then return false else return 2 fi
    end */
    let mut f4 = f.clone();
    f4.body = Stat::If(
      Expr::BoolLiter(false),
      Box::new(Stat::Return(Expr::BoolLiter(false))),
      Box::new(Stat::Return(Expr::IntLiter(2))),
    );
    assert!(func(symbol_table, &f4).is_err());

    /* Only one statement has to return. */
    /* int double(int x) is
      print "hello world"; return 5
    end*/
    let mut f5 = f.clone();
    f5.body = Stat::Sequence(
      Box::new(Stat::Print(Expr::StrLiter(String::from("hello world")))),
      Box::new(Stat::Return(Expr::IntLiter(5))),
    );
    assert!(func(symbol_table, &f5).is_ok());

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
    println!("{:?}", func(symbol_table, &f6));
    assert!(func(symbol_table, &f6).is_err());
  }
}
