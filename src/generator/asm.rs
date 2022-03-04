use super::{display::unescape_char, predef::RequiredPredefs};

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

pub const MIN_STACK_MACHINE_REGS: usize = 2;

#[derive(PartialEq, Debug)]
pub struct GeneratedCode {
  pub data: Vec<Asm>,
  pub text: Vec<Asm>,
  pub required_predefs: Vec<RequiredPredefs>,
  next_label: u32,
  next_msg: u32,
}

/**
 * Return unescaped string to be output in the ASM code.
 * eg. "hello\nworld" -> "hello\\nworld"
 */
fn unescaped_string(str: &str) -> String {
  let mut s = String::with_capacity(str.len());

  for ch in str.chars() {
    if let Some(escaped) = unescape_char(ch) {
      s.push_str(escaped);
    } else {
      s.push(ch);
    }
  }

  s
}

impl GeneratedCode {
  pub fn get_label(&mut self) -> Label {
    let s = format!("L{}", self.next_label);
    self.next_label += 1;
    s
  }

  pub fn get_msg(&mut self, content: &str) -> Label {
    use Directive::*;
    let label = format!("msg_{}", self.next_msg);
    self.next_msg += 1;

    let escaped_content = unescaped_string(content);

    /* msg_0: */
    self.data.push(Asm::Directive(Label(label.clone())));

    /* .word 4 */
    self.data.push(Asm::Directive(Word(content.len())));

    /* .ascii "%c\0" */
    self
      .data
      .push(Asm::Directive(Ascii(escaped_content.to_string())));

    label
  }
}

impl Default for GeneratedCode {
  fn default() -> Self {
    Self {
      data: vec![Asm::Directive(Directive::Data)],
      text: vec![Asm::Directive(Directive::Text)],
      required_predefs: Vec::new(),
      next_label: 0,
      next_msg: 0,
    }
  }
}

/* ======== Represents line within produced assembly apart from instructions.  ======== */

/* Line of assembly. */
#[derive(PartialEq, Debug)]
pub enum Asm {
  Directive(Directive),
  Instr(CondCode, Instr),
  // Asms(Vec<Asm>),
}

impl Asm {
  /* Wraps instruction in an assembly line which always executes. */
  pub fn always(i: Instr) -> Asm {
    Asm::Instr(CondCode::AL, i)
  }

  // pub fn binary() {}

  // pub fn add(a: i32, b: i32, c: i32) -> Asm {
  //   Asm::always(Instr::Pop(Reg::PC))
  // }

  // pub fn flags()

  // pub fn eq(cond: CondCode, mut asm: Asm) -> Asm {
  //   match &mut asm {
  //     Asm::Instr(c, _) => *c = cond,
  //     _ => panic!("Incorrect shortcut usage."),
  //   }

  //   asm
  // }
}

// fn main() {
//   let x = Asm::eq(Asm::add(1, 2, 3));
// }

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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AddressingMode {
  Default,
  PreIndexed,
  // PostIndexed,  unused
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
  Store(DataSize, Reg, (Reg, Offset), AddressingMode), // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289906890.htm

  /* LDR{DataSize}{CondCode}, {Reg}, [{Reg}, #{Offset}] */
  Load(DataSize, Reg, LoadArg), // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289873425.htm

  /* SMULL{CondCode} {Reg}, {Reg}, {Reg}, {Reg}  */
  Multiply(Reg, Reg, Reg, Reg), // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289902800.htm
}

impl Instr {
  pub fn store(data_size: DataSize, reg: Reg, (reg2, offset): (Reg, Offset)) -> Instr {
    Instr::Store(data_size, reg, (reg2, offset), AddressingMode::Default)
  }

  pub fn store_with_mode(
    data_size: DataSize,
    reg: Reg,
    (reg2, offset): (Reg, Offset),
    addr_mode: AddressingMode,
  ) -> Instr {
    Instr::Store(data_size, reg, (reg2, offset), addr_mode)
  }
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

pub const OP2_MAX_VALUE: Imm = 1024;

#[derive(PartialEq, Eq, Debug, Clone)]
// https://www.keil.com/support/man/docs/armasm/armasm_dom1361289851539.htm
pub enum Op2 {
  Imm(Imm),
  Char(char),
  /* Register shifted right {Shift} times. */
  Reg(Reg, Shift),
}

impl Op2 {
  pub fn imm_unroll<F>(mut instr_builder: F, imm: Imm) -> Vec<Asm>
  where
    F: FnMut(Imm) -> Asm,
  {
    let asm_count = (imm / OP2_MAX_VALUE).unsigned_abs();
    let mut asms: Vec<Asm> = Vec::with_capacity(asm_count as usize);

    let imm_sign = imm.signum();
    let mut imm_abs = imm;

    while imm_abs > 0 {
      asms.push(instr_builder(imm_sign * OP2_MAX_VALUE.min(imm_abs)));
      imm_abs -= OP2_MAX_VALUE;
    }

    asms
  }
}

impl From<Imm> for Op2 {
  fn from(i: Imm) -> Self {
    Op2::Imm(i)
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Reg {
  Arg(ArgReg),
  General(GenReg),
  StackPointer,
  Link,
  PC,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ArgReg {
  R0,
  R1,
  R2,
  R3,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum GenReg {
  R4,
  R5,
  R6,
  R7,
  R8,
  R9,
  R10,
  R11,
}

pub const ARGUMENT_REGS: [ArgReg; 4] = [ArgReg::R0, ArgReg::R1, ArgReg::R2, ArgReg::R3];

/* General purpose registers usable for expression evaluation. */
pub const GENERAL_REGS: [GenReg; 8] = [
  GenReg::R4,
  GenReg::R5,
  GenReg::R6,
  GenReg::R7,
  GenReg::R8,
  GenReg::R9,
  GenReg::R10,
  GenReg::R11,
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
