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
  pub predefs: GeneratePredefs,
}

impl GeneratedCode {
  pub fn asm<I: Into<Asm>>(&mut self, i: I) {
    self.text.push(i.into())
  }
}

impl Default for GeneratedCode {
  fn default() -> Self {
    Self {
      data: vec![Asm::Directive(Directive::Data)],
      text: vec![Asm::Directive(Directive::Text)],
      predefs: GeneratePredefs::default(),
    }
  }
}

#[derive(PartialEq, Debug)]
pub struct GeneratePredefs {
  pub print_ints: bool,
  pub print_strings: bool,
  pub print_bools: bool,
  pub print_refs: bool,
  pub println: bool,
  pub read_char: bool,
  pub read_int: bool,
  pub free_pair: bool,
  pub runtime_err: bool,
  pub overflow_err: bool,
  pub div_by_zero: bool,
}

impl Default for GeneratePredefs {
  fn default() -> Self {
    Self {
      print_ints: false,
      print_strings: false,
      print_bools: false,
      print_refs: false,
      println: false,
      read_char: false,
      read_int: false,
      free_pair: false,
      runtime_err: false,
      overflow_err: false,
      div_by_zero: false,
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
  MemAddress(MemAddress),
  Label(Label),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MemAddress {
  pub reg: Reg,
  pub offset: Option<Imm>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DataSize {
  Byte,
  // Halfword, // Not used yet
  Word,
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Reg {
  RegNum(RegNum),
  StackPointer,
  Link,
  PC,
}

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

fn output_assembly(instrs: Vec<Instr>) {
  use std::io::Write;

  let path = "output.s";
  let mut file = File::create(path).unwrap();

  for instr in instrs {
    write!(file, "{}\n", instr).unwrap()
  }
}
