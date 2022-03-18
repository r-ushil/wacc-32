use std::{
  cell::{Cell, RefCell},
  collections::HashSet,
};

use super::{display::unescape_char, predef::RequiredPredefs};

/* ======== Type aliases. ======== */

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
  next_anon: u32,
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
    self.data.push(Asm::Directive(Ascii(escaped_content)));

    label
  }

  // pub fn get_veg(&mut self) -> VegNum {
  //   self.vegs += 1;
  //   self.vegs
  // }
}

impl Default for GeneratedCode {
  fn default() -> Self {
    Self {
      data: vec![Asm::Directive(Directive::Data)],
      text: vec![Asm::Directive(Directive::Text)],
      required_predefs: Vec::new(),
      next_label: 0,
      next_msg: 0,
      next_anon: 0,
    }
  }
}

/* ======== Represents line within produced assembly apart from instructions.  ======== */

/* Line of assembly. */
#[derive(PartialEq, Debug, Clone)]
pub enum Asm {
  Directive(Directive),
  Instr(CondCode, Instr),
}

/* ======= SHORTCUTS ======== */

/* In the future, the unused shortcuts might become useful,
so we're allowing unused functions in this case. */
#[allow(dead_code)]
impl Asm {
  /* Returns vector of registers this instruction reads. */
  pub fn uses(&mut self) -> HashSet<VegNum> {
    let mut v = HashSet::new();

    self.map_uses(|reg| {
      if let Reg::Virtual(vn) = reg {
        v.insert(*vn);
      }
    });

    v
  }

  /* Calls closure on the instructions this instruction needs. */
  pub fn map_uses(&mut self, mut f: impl FnMut(&mut Reg)) {
    use Instr::*;

    let mut g = |reg: &mut Reg| {
      if let Reg::Virtual(_) = reg {
        f(reg)
      }
    };

    match self {
      Asm::Instr(_,
        /* Binary uses two registers if it's second
        operand is a register. */
        Binary(_, _, r1, Op2::Reg(r2, _), _)
        /* Store and multiply always use two registers. */
        | Store(_, r1, (r2, _), _)
        | Multiply(_, _, r1, r2)
      ) => {g(r1); g(r2)},
      Asm::Instr(_, instr) => match instr {
        /* Push, BranchReg, and Load always use one register. */
        Push(r)=> g(r),
        BranchReg(_, r) => g(r),
        /* Unary uses a register if it's operand is a register. */
        Unary(UnaryInstr::Mov, _, Op2::Reg(r, _), _) => g(r),
        Unary(UnaryInstr::Cmp, r1, Op2::Reg(r2, _), _) => {g(r1); g(r2)},
        Unary(UnaryInstr::Cmp, r1, _, _) => g(r1),
        /* Binary uses one register if it's second operand isn't
        a register, which we know to be the case at this pointer because
        otherwise we would've hit above branch and returned. */
        Binary(_, r, _, _, _) => g(r),
        /* Load uses a register if the memory address is specified
        by a register. */
        Load(_, _, LoadArg::MemAddress(r, _)) => g(r),
        _ => (),
      }
      _ => ()
    }
  }

  pub fn defines(&mut self) -> HashSet<VegNum> {
    let mut v = HashSet::new();

    self.map_defines(|reg| {
      if let Reg::Virtual(vn) = reg {
        v.insert(*vn);
      }
    });

    v
  }

  /* Returns register this instruction defines. */
  pub fn map_defines(&mut self, mut f: impl FnMut(&mut Reg)) {
    use Instr::*;

    let mut g = |reg: &mut Reg| {
      if let Reg::Virtual(_) = reg {
        f(reg)
      }
    };

    match self {
      /* Pop always defines the register it writes to from the stack. */
      /* Unar, Binary, and Load always defines it's output register. */
      Asm::Instr(_, Pop(r)) => g(r),
      Asm::Instr(_, Unary(UnaryInstr::Mov, r, _, _)) => g(r),
      Asm::Instr(_, Binary(_, r, _, _, _)) => g(r),
      Asm::Instr(_, Load(_, r, _)) => g(r),
      /* Multiply always writes to two register. */
      Asm::Instr(_, Multiply(r1, r2, _, _)) => {
        g(r1);
        g(r2)
      }
      _ => (),
    };
  }

  /* ==== MODIFIERS ==== */
  /* These modify already existing Asm instructions. */

  /* ==== MODIFIER CONDITIONS ==== */
  pub fn cond(mut self, cond: CondCode) -> Self {
    match &mut self {
      Asm::Instr(c, _) => *c = cond,
      _ => panic!("Condition can only be applied to instructions!"),
    }
    self
  }
  pub fn eq(self) -> Self {
    self.cond(CondCode::EQ)
  }
  pub fn ne(self) -> Self {
    self.cond(CondCode::NE)
  }
  pub fn cs(self) -> Self {
    self.cond(CondCode::CS)
  }
  pub fn vs(self) -> Self {
    self.cond(CondCode::VS)
  }
  pub fn ge(self) -> Self {
    self.cond(CondCode::GE)
  }
  pub fn lt(self) -> Self {
    self.cond(CondCode::LT)
  }
  pub fn gt(self) -> Self {
    self.cond(CondCode::GT)
  }
  pub fn le(self) -> Self {
    self.cond(CondCode::LE)
  }
  pub fn al(self) -> Self {
    self.cond(CondCode::AL)
  }

  /* ==== INSTRUCTIONS ==== */

  pub fn instr(i: Instr) -> Self {
    Self::Instr(CondCode::AL, i)
  }

  pub fn push(reg: impl Into<Reg>) -> Self {
    Self::instr(Instr::Push(reg.into()))
  }

  pub fn pop(reg: impl Into<Reg>) -> Self {
    Self::instr(Instr::Pop(reg.into()))
  }

  pub fn b(label: impl Into<Label>) -> Self {
    Self::instr(Instr::Branch(false, label.into()))
  }

  pub fn bx(reg: impl Into<Reg>) -> Self {
    Self::instr(Instr::BranchReg(false, reg.into()))
  }

  pub fn link(mut self) -> Self {
    match &mut self {
      Self::Instr(_, Instr::Branch(l, _) | Instr::BranchReg(l, _)) => *l = true,
      _ => panic!("Can only apply link to branches."),
    }
    self
  }

  /* UNARY INSTRUCTIONS */
  fn unary(
    unary_instr: UnaryInstr,
    reg: impl Into<Reg>,
    op2: impl Into<Op2>,
  ) -> Self {
    Self::instr(Instr::Unary(unary_instr, reg.into(), op2.into(), false))
  }
  pub fn mov(reg: impl Into<Reg>, op2: impl Into<Op2>) -> Self {
    Self::unary(UnaryInstr::Mov, reg, op2.into())
  }
  pub fn cmp(reg: impl Into<Reg>, op2: impl Into<Op2>) -> Self {
    Self::unary(UnaryInstr::Cmp, reg, op2)
  }

  /* BINARY INSTRUCTIONS */
  fn binary(
    binary_instr: BinaryInstr,
    r1: impl Into<Reg>,
    r2: impl Into<Reg>,
    op2: impl Into<Op2>,
  ) -> Self {
    Self::instr(Instr::Binary(
      binary_instr,
      r1.into(),
      r2.into(),
      op2.into(),
      false,
    ))
  }
  pub fn add(
    r1: impl Into<Reg>,
    r2: impl Into<Reg>,
    op2: impl Into<Op2>,
  ) -> Self {
    Self::binary(BinaryInstr::Add, r1.into(), r2.into(), op2.into())
  }
  pub fn sub(
    r1: impl Into<Reg>,
    r2: impl Into<Reg>,
    op2: impl Into<Op2>,
  ) -> Self {
    Self::binary(BinaryInstr::Sub, r1.into(), r2.into(), op2.into())
  }
  pub fn rev_sub(
    r1: impl Into<Reg>,
    r2: impl Into<Reg>,
    op2: impl Into<Op2>,
  ) -> Self {
    Self::binary(BinaryInstr::RevSub, r1.into(), r2.into(), op2.into())
  }
  pub fn and(
    r1: impl Into<Reg>,
    r2: impl Into<Reg>,
    op2: impl Into<Op2>,
  ) -> Self {
    Self::binary(BinaryInstr::And, r1.into(), r2.into(), op2.into())
  }
  pub fn or(
    r1: impl Into<Reg>,
    r2: impl Into<Reg>,
    op2: impl Into<Op2>,
  ) -> Self {
    Self::binary(BinaryInstr::Or, r1.into(), r2.into(), op2.into())
  }
  pub fn eor(
    r1: impl Into<Reg>,
    r2: impl Into<Reg>,
    op2: impl Into<Op2>,
  ) -> Self {
    Self::binary(BinaryInstr::Eor, r1.into(), r2.into(), op2.into())
  }

  /* STORE AND LOAD */
  pub fn str(
    r1: impl Into<Reg>,
    (r2, offset): (impl Into<Reg>, Offset),
  ) -> Self {
    Self::instr(Instr::Store(
      DataSize::Word,
      r1.into(),
      (r2.into(), offset),
      AddressingMode::Default,
    ))
  }
  pub fn pre_indexed(mut self) -> Self {
    match &mut self {
      Self::Instr(_, Instr::Store(_, _, _, mode)) => {
        *mode = AddressingMode::PreIndexed
      }
      _ => panic!("Can only pre-index a store."),
    }
    self
  }
  pub fn ldr(r1: impl Into<Reg>, arg: impl Into<LoadArg>) -> Self {
    Self::instr(Instr::Load(DataSize::Word, r1.into(), arg.into()))
  }
  pub fn size(mut self, size: DataSize) -> Self {
    match &mut self {
      Self::Instr(_, Instr::Store(s, _, _, _) | Instr::Load(s, _, _)) => {
        *s = size
      }
      _ => panic!("Can only put loads and stores into size mode."),
    }
    self
  }
  pub fn byte(self) -> Self {
    self.size(DataSize::Byte)
  }

  /* MUL */
  pub fn smull(
    r1: impl Into<Reg>,
    r2: impl Into<Reg>,
    r3: impl Into<Reg>,
    r4: impl Into<Reg>,
  ) -> Self {
    Self::instr(Instr::Multiply(r1.into(), r2.into(), r3.into(), r4.into()))
  }

  /* FLAGS */
  pub fn flags(mut self) -> Self {
    match &mut self {
      Asm::Instr(
        _,
        Instr::Unary(_, _, _, flags) | Instr::Binary(_, _, _, _, flags),
      ) => *flags = true,
      _ => panic!("Can only set flags on unary and binary instructions."),
    }
    self
  }
}

/* ======== ASM HELPERS ======== */

#[derive(PartialEq, Debug, Clone)]
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

  /* B{L?}X{CondCode} {Reg} */
  /* Jumps to address specified by register. (Function pointers) */
  BranchReg(bool, Reg),
  // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289866466.htm

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

impl From<Reg> for LoadArg {
  fn from(reg: Reg) -> Self {
    LoadArg::MemAddress(reg.into(), 0)
  }
}

impl From<Label> for LoadArg {
  fn from(l: Label) -> Self {
    LoadArg::Label(l)
  }
}

impl From<i32> for LoadArg {
  fn from(n: i32) -> Self {
    LoadArg::Imm(n)
  }
}

impl From<(Reg, Offset)> for LoadArg {
  fn from((r, offset): (Reg, Offset)) -> Self {
    LoadArg::MemAddress(r, offset)
  }
}

pub const ARM_DSIZE_WORD: i32 = 4;
pub const ARM_DSIZE_BYTE: i32 = 1;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DataSize {
  Byte,
  // Halfword, // Not used yet
  Word,
}

impl From<i32> for DataSize {
  fn from(i: i32) -> Self {
    match i {
      ARM_DSIZE_BYTE => DataSize::Byte,
      ARM_DSIZE_WORD => DataSize::Word,
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
  Add, // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289861747.htm
  Sub, // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289908389.htm
  RevSub, // ??
  And, // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289863017.htm
  Or, // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289884183.htm
  Eor, // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289871065.htm
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

impl From<Imm> for Op2 {
  fn from(i: Imm) -> Self {
    Op2::Imm(i)
  }
}

impl From<Reg> for Op2 {
  fn from(reg: Reg) -> Self {
    Op2::Reg(reg.into(), 0)
  }
}

impl From<ArgReg> for Op2 {
  fn from(ar: ArgReg) -> Self {
    Op2::Reg(ar.into(), 0)
  }
}

impl From<GenReg> for Op2 {
  fn from(ar: GenReg) -> Self {
    Op2::Reg(ar.into(), 0)
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Reg {
  Arg(ArgReg),
  General(GenReg),
  StackPointer,
  Link,
  PC,
  /* Represents a value which has not yet been given a register. */
  Virtual(VegNum),
}

pub type VegNum = usize;

impl From<ArgReg> for Reg {
  fn from(ar: ArgReg) -> Self {
    Reg::Arg(ar)
  }
}

impl From<GenReg> for Reg {
  fn from(ar: GenReg) -> Self {
    Reg::General(ar)
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum ArgReg {
  R0,
  R1,
  R2,
  // R3, unused
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
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

// https://www.keil.com/support/man/docs/armasm/armasm_dom1361289860997.htm
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CondCode {
  EQ,
  NE,
  CS,
  // HS, unused
  // CC, unused
  // LO, unused
  // MI, unused
  // PL, unused
  VS,
  // VC, unused
  // HI, unused
  // LS, unused
  GE,
  LT,
  GT,
  LE,
  AL,
}
