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
}

/* ======= SHORTCUTS ======== */

/* In the future, the unused shortcuts might become useful,
so we're allowing unused functions in this case. */
#[allow(dead_code)]
impl Asm {
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

  pub fn push(reg: Reg) -> Self {
    Self::instr(Instr::Push(reg))
  }

  pub fn pop(reg: Reg) -> Self {
    Self::instr(Instr::Pop(reg))
  }

  pub fn b(label: impl Into<Label>) -> Self {
    Self::instr(Instr::Branch(false, label.into()))
  }

  pub fn link(mut self) -> Self {
    match &mut self {
      Self::Instr(_, Instr::Branch(l, _)) => *l = true,
      _ => panic!("Can only apply link to branches."),
    }
    self
  }

  /* UNARY INSTRUCTIONS */
  fn unary(unary_instr: UnaryInstr, reg: Reg, op2: Op2) -> Self {
    Self::instr(Instr::Unary(unary_instr, reg, op2, false))
  }
  pub fn mov(reg: Reg, op2: Op2) -> Self {
    Self::unary(UnaryInstr::Mov, reg, op2)
  }
  pub fn cmp(reg: Reg, op2: Op2) -> Self {
    Self::unary(UnaryInstr::Cmp, reg, op2)
  }

  /* BINARY INSTRUCTIONS */
  fn binary(binary_instr: BinaryInstr, r1: Reg, r2: Reg, op2: Op2) -> Self {
    Self::instr(Instr::Binary(binary_instr, r1, r2, op2, false))
  }
  pub fn add(r1: Reg, r2: Reg, op2: Op2) -> Self {
    Self::binary(BinaryInstr::Add, r1, r2, op2)
  }
  pub fn sub(r1: Reg, r2: Reg, op2: Op2) -> Self {
    Self::binary(BinaryInstr::Sub, r1, r2, op2)
  }
  pub fn rev_sub(r1: Reg, r2: Reg, op2: Op2) -> Self {
    Self::binary(BinaryInstr::RevSub, r1, r2, op2)
  }
  pub fn and(r1: Reg, r2: Reg, op2: Op2) -> Self {
    Self::binary(BinaryInstr::And, r1, r2, op2)
  }
  pub fn or(r1: Reg, r2: Reg, op2: Op2) -> Self {
    Self::binary(BinaryInstr::Or, r1, r2, op2)
  }
  pub fn eor(r1: Reg, r2: Reg, op2: Op2) -> Self {
    Self::binary(BinaryInstr::Eor, r1, r2, op2)
  }

  /* STORE AND LOAD */
  pub fn str(r1: Reg, (r2, offset): (Reg, Offset)) -> Self {
    Self::instr(Instr::Store(
      DataSize::Word,
      r1,
      (r2, offset),
      AddressingMode::Default,
    ))
  }
  pub fn pre_indexed(mut self) -> Self {
    match &mut self {
      Self::Instr(_, Instr::Store(_, _, _, mode)) => *mode = AddressingMode::PreIndexed,
      _ => panic!("Can only pre-index a store."),
    }
    self
  }
  pub fn ldr(r1: Reg, arg: impl Into<LoadArg>) -> Self {
    Self::instr(Instr::Load(DataSize::Word, r1, arg.into()))
  }
  pub fn size(mut self, size: DataSize) -> Self {
    match &mut self {
      Self::Instr(_, Instr::Store(s, _, _, _) | Instr::Load(s, _, _)) => *s = size,
      _ => panic!("Can only put loads and stores into size mode."),
    }
    self
  }
  pub fn byte(self) -> Self {
    self.size(DataSize::Byte)
  }

  /* MUL */
  pub fn smull(r1: Reg, r2: Reg, r3: Reg, r4: Reg) -> Self {
    Self::instr(Instr::Multiply(r1, r2, r3, r4))
  }

  /* FLAGS */
  pub fn flags(mut self) -> Self {
    match &mut self {
      Asm::Instr(_, Instr::Unary(_, _, _, flags) | Instr::Binary(_, _, _, _, flags)) => {
        *flags = true
      }
      _ => panic!("Can only set flags on unary and binary instructions."),
    }
    self
  }
}

/* ======== ASM HELPERS ======== */

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

impl From<Reg> for LoadArg {
  fn from(r: Reg) -> Self {
    LoadArg::MemAddress(r, 0)
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
  // R3, unused
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
