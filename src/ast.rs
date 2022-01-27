



#[derive(PartialEq, Debug)]
pub struct Program {
  funcs: Vec<Func>,
  statement: Stat,
}

#[derive(PartialEq, Debug)]
pub struct Func {
  return_type: Type,
  ident: Ident,
  param_list: Vec<Param>,
  body: Stat,
}

#[derive(PartialEq, Debug)]
pub struct Param(Type, Ident);

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum AssignLhs {
  Ident(Ident),
  ArrayElem(ArrayElem),
  PairElem(PairElem),
}

#[derive(PartialEq, Debug)]
pub enum AssignRhs {
  Expr(Expr),
  ArrayLiter(Vec<Expr>),
  Pair(Expr, Expr),
  PairElem(PairElem),
  Call(Ident, Vec<Expr>),
}

#[derive(PartialEq, Debug)]
pub enum PairElem {
  Fst(Expr),
  Snd(Expr),
}

#[derive(PartialEq, Debug)]
pub enum Type {
  BaseType(BaseType),
  Array(Box<Type>),
  Pair(PairElemType)
}

#[derive(PartialEq, Debug)]
pub enum PairElemType {
  BaseType(BaseType),
  Array(Box<Type>),
  Pair,
}

#[derive(PartialEq, Debug)]
pub enum BaseType {
  Int,
  Bool,
  Char,
  String,
}

#[derive(PartialEq, Debug)]
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


#[derive(PartialEq, Debug)]
pub enum UnaryOper {
  Bang,
  Neg,
  Len,
  Ord,
  Chr,
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug)]
pub struct ArrayElem(Ident, Vec<Expr>);

#[derive(PartialEq, Debug)]
pub struct ArrayLiter(Vec<Expr>);

