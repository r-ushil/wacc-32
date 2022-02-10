#[derive(PartialEq, Debug, Clone)]
pub struct Program {
  pub funcs: Vec<Func>,
  pub statement: Stat,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Func {
  pub ident: Ident,
  pub signature: FuncSig,
  pub body: Stat,
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
  Assignment(AssignLhs, AssignRhs),
  Read(AssignLhs),
  Free(Expr),
  Return(Expr),
  Exit(Expr),
  Print(Expr),
  Println(Expr),
  If(Expr, Box<Stat>, Box<Stat>),
  While(Expr, Box<Stat>),
  Scope(Box<Stat>),
  Sequence(Box<Stat>, Box<Stat>),
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
