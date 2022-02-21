use std::{fmt::Display, fs::File};

pub type RegNum = u8;
pub type Branch = String;
pub type Imm = i32;
pub type Label = String;

#[derive(PartialEq, Debug)]
pub struct GeneratedCode {
  pub data: Vec<Instr>,
  pub text: Vec<Instr>,
  // pub footer: Vec<Instr>,
}

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
    use CondCode::*;
    let s = match self {
      EQ => "EQ",
      NE => "NE",
      CS => "CS",
      HS => "HS",
      CC => "CC",
      LO => "LO",
      MI => "MI",
      PL => "PL",
      VS => "VS",
      VC => "VC",
      HI => "HI",
      LS => "LS",
      GE => "GE",
      LT => "LT",
      GT => "GT",
      LE => "LE",
      AL => "",
    };
    write!(f, "{}", s)
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
pub enum Load {
  Imm(Imm),
  Label(Label),
}

impl Display for Load {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Load::Imm(val) => write!(f, "{}", val),
      Load::Label(msg) => write!(f, "{}", msg),
    }
  }
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
  Push,

  Word(i32),
  Ascii(String),

  Label(Label),
  LoadImm(Reg, Load),
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
    use Instr::*;
    match self {
      Push => write!(f, "PUSH {{lr}}"),
      Word(size) => write!(f, ".word {}", size),
      Ascii(string) => write!(f, ".ascii \"{}\"", string),

      Label(label) => write!(f, "{}:", label),
      LoadImm(reg, val) => write!(f, "LDR {}, ={}", reg, val),

      Pop => write!(f, "POP {{pc}}"),
      Assemble => write!(f, ".ltorg"),

      StoreByte(reg, mem_addr) => write!(f, "STRB {}, {}", reg, mem_addr),
      Store(reg, mem_addr) => write!(f, "STR {}, {}", reg, mem_addr),
      LoadMemByte(reg, mem_addr) => write!(f, "LDRSB {}, {}", reg, mem_addr),

      Add(r1, r2, op2) => write!(f, "ADD {}, {}, {}", r1, r2, op2),
      Sub(r1, r2, op2) => write!(f, "SUB {}, {}, {}", r1, r2, op2),
      AddFlags(r1, r2, op2) => write!(f, "ADDS {}, {}, {}", r1, r2, op2),
      SubFlags(r1, r2, op2) => write!(f, "SUBS {}, {}, {}", r1, r2, op2),

      And(r1, r2, op2) => write!(f, "AND {}, {}, {}", r1, r2, op2),
      Or(r1, r2, op2) => write!(f, "ORR {}, {}, {}", r1, r2, op2),
      Cmp(reg, op2) => write!(f, "CMP {}, {}", reg, op2),

      Mov(reg, op2, code) => write!(f, "MOV{} {}, {}", code, reg, op2),
      Branch(branch, code) => write!(f, "BL{} {}", code, branch),

      Multiply(r1, r2, r3, r4) => write!(f, "SMULL {}, {}, {}, {}", r1, r2, r3, r4),
      ReverseSubtract(r1, r2, op2) => write!(f, "RSBS {}, {}, {}", r1, r2, op2),
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
