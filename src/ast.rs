use crate::analyser::context::SymbolTable;

#[derive(PartialEq, Debug, Clone)]
pub struct Program {
  pub funcs: Vec<Func>,
  pub statement: ScopedStat,
  pub symbol_table: SymbolTable,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Func {
  pub ident: Ident,
  pub signature: FuncSig,
  pub body: Stat,
  pub params_st: SymbolTable,
  pub body_st: SymbolTable,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FuncSig {
  pub params: Vec<(Type, Ident)>,
  pub return_type: Type,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Stat {
  Skip,
  Declaration(Type, Ident, AssignRhs),
  Assignment(AssignLhs, Type, AssignRhs),
  Read(Type, AssignLhs),
  Free(Type, Expr),
  Return(Expr),
  Exit(Expr),
  Print(Type, Expr),
  Println(Type, Expr),
  Sequence(Box<Stat>, Box<Stat>),

  /* SCOPING STATEMENTS */
  /* These statements hold their own symbol table, which contains the variables
  declared within, and a reference to the parent symbol table. */
  If(Expr, ScopedStat, ScopedStat),
  While(Expr, ScopedStat),
  Scope(ScopedStat),
}

impl Stat {
  pub fn sequence(s1: Stat, s2: Stat) -> Stat {
    Stat::Sequence(Box::new(s1), Box::new(s2))
  }

  pub fn declaration(t: Type, i: impl Into<Ident>, r: impl Into<AssignRhs>) -> Stat {
    Stat::Declaration(t, i.into(), r.into())
  }

  pub fn return_(e: impl Into<Expr>) -> Stat {
    Stat::Return(e.into())
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ScopedStat(pub SymbolTable, pub Box<Stat>);

impl ScopedStat {
  pub fn new(statement: Stat) -> ScopedStat {
    ScopedStat(SymbolTable::default(), Box::new(statement))
  }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AssignLhs {
  Ident(Ident),
  ArrayElem(ArrayElem),
  PairElem(PairElem),
}

#[derive(PartialEq, Debug, Clone)]
pub enum AssignRhs {
  Expr(Expr),
  ArrayLiter(ArrayLiter),
  Pair(Expr, Expr),
  PairElem(PairElem),
  Call(Ident, Vec<Expr>),
}

/* Expr => AssignRhs::Expr */
impl<E> From<E> for AssignRhs
where
  E: Into<Expr>,
{
  fn from(e: E) -> Self {
    AssignRhs::Expr(e.into())
  }
}

#[derive(PartialEq, Debug, Clone)]
pub enum PairElem {
  Fst(Expr),
  Snd(Expr),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
  Int,
  Bool,
  Char,
  String,
  Any,
  Array(Box<Type>),
  Pair(Box<Type>, Box<Type>),
  Func(Box<FuncSig>),
}

impl Default for Type {
  fn default() -> Self {
    Type::Any
  }
}

impl Type {
  /* Returns how many bytes are required to store a value of this type. */
  pub fn size(&self) -> i32 {
    use Type::*;
    match self {
      Bool | Char => 1,
      Any => panic!("Size of Type::Any can not be known."),
      Func(_) => 0,
      _ => 4,
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
  IntLiter(i32),
  BoolLiter(bool),
  CharLiter(char),
  StrLiter(String),
  PairLiter,
  Ident(Ident),
  ArrayElem(ArrayElem),
  UnaryApp(UnaryOper, Box<Expr>),
  BinaryApp(Box<Expr>, BinaryOper, Box<Expr>),
}

impl Expr {
  pub fn ident(s: impl Into<String>) -> Expr {
    Expr::Ident(s.into())
  }
}

impl From<i32> for Expr {
  fn from(n: i32) -> Self {
    Expr::IntLiter(n)
  }
}

#[derive(PartialEq, Debug, Clone)]
pub enum UnaryOper {
  Bang,
  Neg,
  Len,
  Ord,
  Chr,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOper {
  Mul,
  Div,
  Mod,
  Add,
  Sub,
  Gt,
  Gte,
  Lt,
  Lte,
  Eq,
  Neq,
  And,
  Or,
}

// #[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub type Ident = String;
// pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone)]
pub struct ArrayElem(pub Ident, pub Vec<Expr>);

#[derive(PartialEq, Debug, Clone)]
pub struct ArrayLiter(pub Vec<Expr>);
