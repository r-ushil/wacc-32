use crate::{
  analyser::context::SymbolTable,
  generator::asm::{Offset, VegNum},
};
use std::{cell::Cell, collections::HashMap};

pub type NamedFunc = (Ident, Func);

#[derive(PartialEq, Debug, Clone)]
pub struct Program {
  /* User defined functions. */
  pub funcs: Vec<NamedFunc>,
  /* Program body. */
  pub statement: ScopedStat,
  pub statement_vegs: Cell<VegNum>,
  /* Top level symbol table (root node in any
  given scope in this program.) */
  pub symbol_table: SymbolTable,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Func {
  //pub ident: Ident,
  pub param_ids: Vec<Ident>,
  pub signature: FuncSig,
  pub body: Stat,
  pub params_st: SymbolTable,
  pub body_st: SymbolTable,
  /* How many vegs this function uses. */
  pub vegs: Cell<VegNum>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FuncSig {
  pub param_types: Vec<Type>,
  pub return_type: Type,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Stat {
  Skip,
  Declaration(Type, Expr, Expr),
  Assignment(Expr, Type, Expr),
  Read(TypedExpr),
  Free(TypedExpr),
  Return(Expr),
  Exit(Expr),
  Print(TypedExpr),
  Println(TypedExpr),
  Sequence(Box<Stat>, Box<Stat>),

  /* SCOPING STATEMENTS */
  /* These statements hold their own symbol table, which contains the variables
  declared within, and a reference to the parent symbol table. */
  If(Expr, ScopedStat, ScopedStat),
  While(Expr, ScopedStat),
  Scope(ScopedStat),
}

#[derive(PartialEq, Debug, Clone)]
pub struct ScopedStat(pub SymbolTable, pub Box<Stat>);

impl ScopedStat {
  pub fn new(statement: Stat) -> ScopedStat {
    ScopedStat(SymbolTable::default(), Box::new(statement))
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct StructLiter {
  pub id: Ident,
  pub fields: HashMap<Ident, Expr>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct StructElem(pub Ident, pub Box<Expr>, pub Ident);

#[derive(PartialEq, Debug, Clone)]
pub enum PairElem {
  /* Type of fst and snd elem respectively.
  (fst and snd are concidered generic functions) */
  Fst(TypedExpr),
  Snd(TypedExpr),
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
  Custom(Ident),
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Struct {
  /* Details about the fields of this struct. */
  pub fields: HashMap<Ident, (Type, Offset)>,
  /* Size in bytes of the whole struct. */
  pub size: Offset,
}

impl Struct {
  pub fn new() -> Struct {
    Self {
      fields: HashMap::new(),
      size: 0,
    }
  }

  pub fn add_field(&mut self, t: Type, id: Ident) -> Option<Offset> {
    let offset = self.size;

    /* Grow size of structs by size of this type. */
    self.size += t.size();

    match self.fields.insert(id, (t, offset)) {
      Some(_) => None,
      None => Some(offset),
    }
  }
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
      _ => 4,
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct TypedExpr(pub Type, pub Expr);

impl TypedExpr {
  pub fn new(expr: Expr) -> Self {
    TypedExpr(Type::default(), expr)
  }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
  /* Literal values. */
  IntLiter(i32),
  BoolLiter(bool),
  CharLiter(char),
  StrLiter(String),
  NullPairLiter,
  PairLiter(Box<TypedExpr>, Box<TypedExpr>),
  BlankArrayLiter(Type, Box<Expr>),
  ArrayLiter(ArrayLiter), /* Type is type of elements. */
  StructLiter(StructLiter),
  /* Identifiers. */
  Ident(Ident),
  /* Element access. */
  /* Type stored on array elem is the type of the elements of the array. */
  ArrayElem(Type, Box<Expr>, Box<Expr>), /* a[b] where a, b are 1st and 2nd expression. */
  StructElem(StructElem),
  PairElem(Box<PairElem>),
  /* Operator application. */
  UnaryApp(UnaryOper, Box<Expr>),
  BinaryApp(Box<Expr>, BinaryOper, Box<Expr>),
  /* Function calls. */
  Call(Type, Box<Expr>, Vec<Expr>),
  /* Anonymous functions. */
  AnonFunc(Box<Func>),
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

/* Stores the type of the elements. */
#[derive(PartialEq, Debug, Clone)]
pub struct ArrayLiter(pub Type, pub Vec<Expr>);
