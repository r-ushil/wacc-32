#[derive(PartialEq, Debug, Clone)]
pub struct Program {
  pub funcs: Vec<Func>,
  pub statement: Stat,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Func {
  pub return_type: Type,
  pub ident: Ident,
  pub param_list: Vec<Param>,
  pub body: Stat,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Param(pub Type, pub Ident);

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
  BaseType(BaseType),
  Array(Box<Type>),
  Pair(Box<Type>, Box<Type>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum BaseType {
  Int,
  Bool,
  Char,
  String,
  Any,
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

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone)]
pub struct ArrayElem(pub Ident, pub Vec<Expr>);

#[derive(PartialEq, Debug, Clone)]
pub struct ArrayLiter(pub Vec<Expr>);
