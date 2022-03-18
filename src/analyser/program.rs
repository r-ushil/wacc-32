use super::*;
use crate::{ast::*, generator::asm::Reg};
use stat::*;
use ReturnBehaviour::*;

impl Analysable for Func {
  type Input = ();
  type Output = ();

  fn analyse(&mut self, scope: &mut ScopeBuilder, _: ()) -> AResult<()> {
    let scope = &mut scope.new_scope(&mut self.params_st);

    /* Makee scope.get_veg() */
    scope.vegs = &mut self.vegs;

    let pts = self.signature.param_types.iter().rev();
    let pis = self.param_ids.iter_mut().rev();

    /* Add parameters to parameter scope. */
    for (arg_num, (pt, pi)) in pts.zip(pis).enumerate() {
      scope.insert(pi, IdentInfo::LocalVar(pt.clone(), Reg::FuncArg(arg_num)));
      // scope.insert_var(pi, pt.clone())?;
    }

    /* Enter body scope. */
    let scope = &mut scope.new_scope(&mut self.body_st);

    /* Type check function body and make sure it returns value of correct type. */
    match self.body.analyse(scope, ())? {
      AtEnd(t)
        if t.clone().unify(self.signature.return_type.clone()) == None =>
      {
        Err(SemanticError::Normal(format!(
          "Function body returns {:?} but function signature expects {:?}",
          t, self.signature.return_type
        )))
      }
      AtEnd(_) => Ok(()),
      _ => Err(SemanticError::Syntax(
        "The last statement should be a return or exit.".to_string(),
      )),
    }
  }
}

/* Semantically checks an entire program. */
/* This function initialises the symbol table and function table. */
impl Analysable for Program {
  type Input = ();
  type Output = ();

  fn analyse(&mut self, _: &mut ScopeBuilder, _: ()) -> AResult<()> {
    /* root, global scope. */
    let vegs = Cell::new(0);
    let mut scope = ScopeBuilder::new(&mut self.symbol_table, &vegs);

    /* Add all function signatures to global before analysing. (hoisting) */
    for func_tuple in self.funcs.iter() {
      let (ident, func) = func_tuple;
      scope.insert(
        &ident,
        IdentInfo::Label(
          Type::Func(Box::new(func.signature.clone())),
          format!("f_{}", ident),
        ),
      )?;
    }

    /* Analyse functions. */
    for (_, f) in self.funcs.iter_mut() {
      f.analyse(&mut scope, ())?;
    }

    /* Program body must never return, but it can exit. */
    scope.vegs = &mut self.statement_vegs;
    match self.statement.analyse(&mut scope, ())? {
      MidWay(t) | AtEnd(t) if t != Type::Any => Err(SemanticError::Normal(
        "Cannot have 'return' statement in main".to_string(),
      )),
      _ => Ok(()),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::analyser::context::SymbolTable;

  use super::*;

  #[test]
  fn func_parameters_checked() {
    let mut symbol_table = SymbolTable::default();
    let cell = Cell::new(0);
    let scope = &mut ScopeBuilder::new(&mut symbol_table, &cell);

    /* Function */
    /* int double(int x) is return x * 2 end */
    let f = Func {
      signature: FuncSig {
        param_types: vec![Type::Int],
        return_type: Type::Int,
      },
      body: Stat::Return(Expr::BinaryApp(
        Box::new(Expr::Ident(String::from("x"))),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(2)),
      )),
      params_st: SymbolTable::default(),
      body_st: SymbolTable::default(),
      param_ids: vec![String::from("x")],
      vegs: Cell::new(0),
    };

    /* Works in it's default form. */
    assert!(f.clone().analyse(scope, ()).is_ok());

    /* Doesn't work if wrong type returned. */
    /* int double(int x) is return false end */
    let mut f1 = f.clone();
    f1.body = Stat::Return(Expr::BoolLiter(false));
    assert!(f1.analyse(scope, ()).is_err());

    /* Can compare parameter type with return type. */
    /* bool double(int x) is return x end */
    let mut f2 = f;
    f2.signature.return_type = Type::Bool;
    f2.body = Stat::Return(Expr::Ident(String::from("x")));
    assert!(f2.analyse(scope, ()).is_err());
  }

  #[test]
  fn branching_return_types_checked() {
    /* Function */
    /* int double(int x) is return x * 2 end */
    let f = Func {
      signature: FuncSig {
        param_types: vec![Type::Int],
        return_type: Type::Int,
      },
      body: Stat::Return(Expr::BinaryApp(
        Box::new(Expr::Ident(String::from("x"))),
        BinaryOper::Mul,
        Box::new(Expr::IntLiter(2)),
      )),
      params_st: SymbolTable::default(),
      body_st: SymbolTable::default(),
      param_ids: vec![String::from("x")],
      vegs: Cell::new(0),
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
    assert!(f3
      .analyse(
        &mut ScopeBuilder::new(&mut SymbolTable::default(), &Cell::new(0)),
        ()
      )
      .is_ok());

    /* int double(int x) is
      if true then return false else return 2 fi
    end */
    let mut f4 = f.clone();
    f4.body = Stat::If(
      Expr::BoolLiter(false),
      ScopedStat::new(Stat::Return(Expr::BoolLiter(false))),
      ScopedStat::new(Stat::Return(Expr::IntLiter(2))),
    );

    assert!(f4
      .analyse(
        &mut ScopeBuilder::new(&mut SymbolTable::default(), &Cell::new(0)),
        ()
      )
      .is_err());

    /* Only one statement has to return. */
    /* int double(int x) is
      print "hello world"; return 5
    end*/
    let mut f5 = f.clone();
    f5.body = Stat::Sequence(
      Box::new(Stat::Print(TypedExpr::new(Expr::StrLiter(String::from(
        "hello world",
      ))))),
      Box::new(Stat::Return(Expr::IntLiter(5))),
    );
    let x = f5.analyse(
      &mut ScopeBuilder::new(&mut SymbolTable::default(), &Cell::new(0)),
      (),
    );
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
      Box::new(Stat::Print(TypedExpr::new(Expr::StrLiter(String::from(
        "Hello World",
      ))))),
    );
    assert!(f6
      .analyse(
        &mut ScopeBuilder::new(&mut SymbolTable::default(), &Cell::new(0)),
        ()
      )
      .is_err());
  }
}
