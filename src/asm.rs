use std::{fmt::Display, fs::File};

pub type RegNum = u8;
pub type Branch = String;
pub type Imm = i32;
pub type Label = String;

impl Display for Op2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Op2::Imm(val) => write!(f, "={}", val),
      Op2::Reg(reg) => write!(f, "{}", reg),
      Op2::LeftShiftedReg(reg, shift_val) => write!(f, "{}, LSL #{}", reg, shift_val),
      Op2::RightShiftedReg(reg, shift_val) => write!(f, "{}, ASR #{}", reg, shift_val),
    }
  }
}

impl Display for CondCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CondCode::EQ => write!(f, "EQ"),
      CondCode::NE => write!(f, "NE"),
      CondCode::CS => write!(f, "CS"),
      CondCode::HS => write!(f, "HS"),
      CondCode::CC => write!(f, "CC"),
      CondCode::LO => write!(f, "LO"),
      CondCode::MI => write!(f, "MI"),
      CondCode::PL => write!(f, "PL"),
      CondCode::VS => write!(f, "VS"),
      CondCode::VC => write!(f, "VC"),
      CondCode::HI => write!(f, "HI"),
      CondCode::LS => write!(f, "LS"),
      CondCode::GE => write!(f, "GE"),
      CondCode::LT => write!(f, "LT"),
      CondCode::GT => write!(f, "GT"),
      CondCode::LE => write!(f, "LE"),
      CondCode::AL => write!(f, ""),
    }
  }
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
pub enum Op2 {
  Reg(Reg),
  Imm(Imm),
  LeftShiftedReg(Reg, Imm),
  RightShiftedReg(Reg, Imm),
}

impl Display for Reg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Reg::RegNum(num) => write!(f, "r{}", num),
      Reg::StackPointer => write!(f, "sp"),
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Reg {
  RegNum(RegNum),
  StackPointer,
}

impl Display for MemAddress {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.offset {
      Some(val) => write!(f, "[{}, #{}", self.reg, val),
      None => write!(f, "[{}]", self.reg),
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MemAddress {
  pub reg: Reg,
  pub offset: Option<i32>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Instr {
  Label(Label),
  LoadImm(Reg, Imm),
  Mov(Reg, Op2, CondCode),

  Branch(Branch, CondCode),
  Pop,
  Assemble, //.ltorg

  StoreByte(Reg, MemAddress),
  Store(Reg, MemAddress),
  LoadMemByte(Reg, MemAddress),

  And(Reg, Reg, Op2),
  Or(Reg, Reg, Op2),

  Cmp(Reg, Op2),

  Add(Reg, Reg, Op2),
  Sub(Reg, Reg, Op2),

  AddFlags(Reg, Reg, Op2),
  SubFlags(Reg, Reg, Op2),

  Multiply(Reg, Reg, Reg, Reg),

  ReverseSubtract(Reg, Reg, Op2),
}

impl Display for Instr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Instr::Label(label) => write!(f, "{}:", label),

      Instr::LoadImm(reg, val) => write!(f, "LDR {}, ={}", reg, val),

      Instr::Pop => write!(f, "POP {{pc}}"),
      Instr::Assemble => write!(f, ".ltorg"),

      Instr::StoreByte(reg, mem_addr) => write!(f, "STRB {}, {}", reg, mem_addr),
      Instr::Store(reg, mem_addr) => write!(f, "STR {}, {}", reg, mem_addr),

      Instr::LoadMemByte(reg, mem_addr) => write!(f, "LDRSB {}, {}", reg, mem_addr),

      Instr::Add(r1, r2, op2) => write!(f, "ADD {}, {}, {}", r1, r2, op2),
      Instr::Sub(r1, r2, op2) => write!(f, "SUB {}, {}, {}", r1, r2, op2),
      Instr::AddFlags(r1, r2, op2) => write!(f, "ADDS {}, {}, {}", r1, r2, op2),
      Instr::SubFlags(r1, r2, op2) => write!(f, "SUBS {}, {}, {}", r1, r2, op2),

      Instr::And(r1, r2, op2) => write!(f, "AND {}, {}, {}", r1, r2, op2),
      Instr::Or(r1, r2, op2) => write!(f, "ORR {}, {}, {}", r1, r2, op2),
      Instr::Cmp(reg, op2) => write!(f, "CMP {}, {}", reg, op2),

      Instr::Mov(reg, op2, code) => write!(f, "MOV{} {}, {}", code, reg, op2),
      Instr::Branch(branch, code) => write!(f, "BL{} {}", code, branch),

      Instr::Multiply(r1, r2, r3, r4) => write!(f, "SMULL {}, {}, {}, {}", r1, r2, r3, r4),
      Instr::ReverseSubtract(r1, r2, op2) => write!(f, "RSBS {}, {}, {}", r1, r2, op2),
    }
  }
}

fn output_assembly(instrs: Vec<Instr>) {
  use std::io::Write;

  let path = "output.s";
  let mut file = File::create(path).unwrap();

  for instr in instrs {
    write!(file, "{}\n", instr).unwrap()
  }
}
