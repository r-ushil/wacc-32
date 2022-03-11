use super::*;
use crate::ast::*;
use stat::*;
use ReturnBehaviour::*;

fn func(scope: &ScopeBuilder, func: &mut Func) -> AResult<()> {
  let scope = &mut scope.new_scope(&mut func.params_st);

  let pts = func.signature.param_types.iter().rev();
  let pis = func.param_ids.iter_mut().rev();

  /* Add parameters to parameter scope. */
  for (pt, pi) in pts.zip(pis) {
    scope.insert_var(pi, pt.clone())?;
  }

  /* Enter body scope. */
  let scope = &mut scope.new_scope(&mut func.body_st);

  /* Type check function body and make sure it returns value of correct type. */
  match stat(scope, &mut func.body)? {
    AtEnd(t) if t.clone().unify(func.signature.return_type.clone()) == None => {
      Err(SemanticError::Normal(format!(
        "Function body returns {:?} but function signature expects {:?}",
        t, func.signature.return_type
      )))
    }
    AtEnd(_) => Ok(()),
    _ => Err(SemanticError::Syntax(
      "The last statement should be a return or exit.".to_string(),
    )),
  }
}

/* Semantically checks an entire program. */
/* This function initialises the symbol table and function table. */
#[allow(dead_code)]
pub fn program(program: &mut Program) -> AResult<()> {
  /* root, global scope. */
  let mut scope = ScopeBuilder::new(&mut program.symbol_table);

  /* Add all function signatures to global before analysing. (hoisting) */
  for func in program.funcs.iter() {
    scope.insert(
      &func.ident,
      IdentInfo::Label(
        Type::Func(Box::new(func.signature.clone())),
        format!("f_{}", func.ident),
      ),
    )?;
  }

  /* Analyse functions. */
  for f in program.funcs.iter_mut() {
    func(&scope, f)?;
  }

  /* Program body must never return, but it can exit. */
  match scoped_stat(&scope, &mut program.statement)? {
    MidWay(t) | AtEnd(t) if t != Type::Any => Err(SemanticError::Normal(
      "Cannot have 'return' statement in main".to_string(),
    )),
    _ => Ok(()),
  }
}

#[cfg(test)]
mod tests {
  use crate::analyser::context::SymbolTable;

  use super::*;

  #[test]
  fn func_parameters_checked() {
    let mut symbol_table = SymbolTable::default();
    let scope = &mut ScopeBuilder::new(&mut symbol_table);

    /* Function */
    /* int double(int x) is return x * 2 end */
    let f = Func {
      ident: String::from("double"),
      signature: FuncSig {
        param_types: vec![Type::Int],
        return_type: Type::Int,
      },
      body: Stat::Return(Expr::BinaryApp(
        Box::new(Expr::LocalVar(String::from("x"))),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(2)),
      )),
      params_st: SymbolTable::default(),
      body_st: SymbolTable::default(),
      param_ids: vec![String::from("x")],
    };

    /* Works in it's default form. */
    assert!(func(scope, &mut f.clone()).is_ok());

    /* Doesn't work if wrong type returned. */
    /* int double(int x) is return false end */
    let mut f1 = f.clone();
    f1.body = Stat::Return(Expr::BoolLiter(false));
    assert!(func(scope, &mut f1).is_err());

    /* Can compare parameter type with return type. */
    /* bool double(int x) is return x end */
    let mut f2 = f;
    f2.signature.return_type = Type::Bool;
    f2.body = Stat::Return(Expr::LocalVar(String::from("x")));
    assert!(func(scope, &mut f2).is_err());
  }

  #[test]
  fn branching_return_types_checked() {
    /* Function */
    /* int double(int x) is return x * 2 end */
    let f = Func {
      ident: String::from("double"),
      signature: FuncSig {
        param_types: vec![Type::Int],
        return_type: Type::Int,
      },
      body: Stat::Return(Expr::BinaryApp(
        Box::new(Expr::LocalVar(String::from("x"))),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(2)),
      )),
      params_st: SymbolTable::default(),
      body_st: SymbolTable::default(),
      param_ids: vec![String::from("x")],
    };

    /* Both branches of if statements must return correct type. */
    /* int double(int x) is
      if true then return 5 else return 2 fi
    end */
    let mut f3 = f.clone();
    f3.body = Stat::If(
      Expr::BoolLiter(false),
      ScopedStat::new(Stat::Return(Expr::IntLiter(5))),
      ScopedStat::new(Stat::Return(Expr::IntLiter(2))),
    );
    assert!(
      func(&mut ScopeBuilder::new(&mut SymbolTable::default()), &mut f3)
        .is_ok()
    );

    /* int double(int x) is
      if true then return false else return 2 fi
    end */
    let mut f4 = f.clone();
    f4.body = Stat::If(
      Expr::BoolLiter(false),
      ScopedStat::new(Stat::Return(Expr::BoolLiter(false))),
      ScopedStat::new(Stat::Return(Expr::IntLiter(2))),
    );

    assert!(
      func(&mut ScopeBuilder::new(&mut SymbolTable::default()), &mut f4)
        .is_err()
    );

    /* Only one statement has to return. */
    /* int double(int x) is
      print "hello world"; return 5
    end*/
    let mut f5 = f.clone();
    f5.body = Stat::Sequence(
      Box::new(Stat::Print(
        Type::default(),
        Expr::StrLiter(String::from("hello world")),
      )),
      Box::new(Stat::Return(Expr::IntLiter(5))),
    );
    let x = func(&mut ScopeBuilder::new(&mut SymbolTable::default()), &mut f5);
    assert!(x.is_ok());

    /* Spots erroneous returns. */
    /* int double(int x) is
      if true then return true else skip fi;
      return 5
    end*/
    let mut f6 = f;
    f6.body = Stat::Sequence(
      Box::new(Stat::If(
        Expr::BoolLiter(true),
        ScopedStat::new(Stat::Return(Expr::BoolLiter(true))),
        ScopedStat::new(Stat::Skip),
      )),
      Box::new(Stat::Print(
        Type::default(),
        Expr::StrLiter(String::from("Hello World")),
      )),
    );
    assert!(
      func(&mut ScopeBuilder::new(&mut SymbolTable::default()), &mut f6)
        .is_err()
    );
  }
}
