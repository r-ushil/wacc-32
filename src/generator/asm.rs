use super::predef::RequiredPredefs;
use std::fs::File;

/* ======== Type aliases. ======== */

/* r4, r5, r0, ... */
pub type RegNum = u8;
/* An immediate value. */
pub type Imm = i32;
/* A location which can be branched to. */
pub type Label = String;
/* Whether or not an instruction sets flags. */
pub type Flags = bool;
/* How much to offset a register by. */
pub type Offset = i32;
/* How much to shift a register by. */
pub type Shift = i32;

/* ======== Represents entire program. ======== */

#[derive(PartialEq, Debug)]
pub struct GeneratedCode {
  pub data: Vec<Asm>,
  pub text: Vec<Asm>,
  pub required_predefs: Vec<RequiredPredefs>,
  next_label: u32,
}

impl GeneratedCode {
  pub fn get_label(&mut self) -> Label {
    let s = format!("L{}", self.next_label);
    self.next_label += 1;
    s
  }
  pub fn asm<I: Into<Asm>>(&mut self, i: I) {
    self.text.push(i.into())
  }
}

impl Default for GeneratedCode {
  fn default() -> Self {
    Self {
      data: vec![Asm::Directive(Directive::Data)],
      text: vec![Asm::Directive(Directive::Text)],
      required_predefs: Vec::new(),
      next_label: 0,
    }
  }
}

/* ======== Represents line within produced assembly apart from instructions.  ======== */

/* Line of assembly. */
#[derive(PartialEq, Debug)]
pub enum Asm {
  Directive(Directive),
  Instr(CondCode, Instr),
}

impl Asm {
  /* Wraps instruction in an assembly line which always executes. */
  pub fn always(i: Instr) -> Asm {
    Asm::Instr(CondCode::AL, i)
  }
}

impl<I> From<I> for Asm
where
  I: Into<Instr>,
{
  fn from(i: I) -> Self {
    Asm::Instr(CondCode::AL, i.into())
  }
}

#[derive(PartialEq, Debug)]
pub enum Directive {
  Text,          /* .text */
  Data,          /* .data */
  Assemble,      /* .ltorg */
  Label(String), /* foo: */
  Word(usize),   /* .word 5 */
  Ascii(String), /* .ascii "Hello World" */
}

/* ======== Instructions! ======== */

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Instr {
  /* PUSH {reg} */
  Push(Reg),
  /* POP {reg} */
  Pop(Reg),

  /* B{L?}{CondCode} {Label} */
  /* If bool true, branch with link. */
  Branch(bool, Label),
  // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289863797.htm
  // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289865686.htm

  /* Instructions which take an operand2 and store result in a register. */
  /* {UnaryInstr}{UnaryArgs}Flags){CondCode} {UnaryArgs.Reg,Reg,Op2) */
  Unary(UnaryInstr, Reg, Op2, Flags),

  /* Instructions which take an operand2, register and store result in a register. */
  /* {BinaryInstr}{BinaryArgs.Flags){CondCode} {BinaryArgs.Reg,Reg,Op2} */
  Binary(BinaryInstr, Reg, Reg, Op2, Flags),

  /* STR{DataSize}{CondCode}, {Reg}, [{Reg}, #{Offset}] */
  Store(DataSize, Reg, (Reg, Offset)), // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289906890.htm

  /* LDR{DataSize}{CondCode}, {Reg}, [{Reg}, #{Offset}] */
  Load(DataSize, Reg, LoadArg), // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289873425.htm

  /* SMULL{CondCode} {Reg}, {Reg}, {Reg}, {Reg}  */
  Multiply(Reg, Reg, Reg, Reg), // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289902800.htm
}

impl<Op, D, S, O> From<(Op, D, S, O)> for Instr
where
  Op: Into<BinaryInstr>,
  D: Into<Reg>,
  S: Into<Reg>,
  O: Into<Op2>,
{
  fn from((op, dst, src, op2): (Op, D, S, O)) -> Self {
    Instr::Binary(op.into(), dst.into(), src.into(), op2.into(), false)
  }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LoadArg {
  Imm(Imm),
  MemAddress(Reg, Offset),
  Label(Label),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DataSize {
  Byte,
  // Halfword, // Not used yet
  Word,
}

impl From<i32> for DataSize {
  fn from(i: i32) -> Self {
    match i {
      1 => DataSize::Byte,
      4 => DataSize::Word,
      _ => unimplemented!(),
    }
  }
}

/*  */
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UnaryInstr {
  Mov, // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289878994.htm
  Cmp, // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289868786.htm
}

/* Instructions which take the form "XXX{flags}{cond} Rd, Rn, Operand2" */
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BinaryInstr {
  Add,    // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289861747.htm
  Sub,    // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289908389.htm
  RevSub, // ??
  And,    // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289863017.htm
  Or,     // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289884183.htm
  Eor,    // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289871065.htm
}

/* ======== Helper types for use within assembly representations.  ======== */

#[derive(PartialEq, Eq, Debug, Clone)]
// https://www.keil.com/support/man/docs/armasm/armasm_dom1361289851539.htm
pub enum Op2 {
  Imm(Imm),
  Char(char),
  /* Register shifted right {Shift} times. */
  Reg(Reg, Shift),
}

impl From<Imm> for Op2 {
  fn from(i: Imm) -> Self {
    Op2::Imm(i)
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Reg {
  RegNum(RegNum),
  StackPointer,
  Link,
  PC,
}

/* General purpose registers usable for expression evaluation. */
pub const GENERAL_REGS: [Reg; 8] = [
  Reg::RegNum(4),
  Reg::RegNum(5),
  Reg::RegNum(6),
  Reg::RegNum(7),
  Reg::RegNum(8),
  Reg::RegNum(9),
  Reg::RegNum(10),
  Reg::RegNum(11),
];

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CondCode {
  EQ,
  NE,
  CS,
  HS,
  CC,
  LO,
  MI,
  PL,
  VS,
  VC,
  HI,
  LS,
  GE,
  LT,
  GT,
  LE,
  AL,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Load {
  Imm(Imm),
  Label(Label),
}
