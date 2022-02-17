use std::fmt::Display;

pub type RegNum = u8;
pub type ExitCode = u8;
pub type Branch = String;
pub type Imm = i32;

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

pub struct MemAddress {
  pub reg: Reg,
  pub offset: Option<i32>,
}

pub enum Instr {
  LoadImm(Reg, Imm),
  Mov(Reg, Op2),

  Branch(Branch),
  Pop,
  Assemble, //.ltorg

  StoreByte(Reg, MemAddress),
  Store(Reg, MemAddress),
  LoadMemByte(Reg, MemAddress),

  Add(Reg, Reg, Op2),
  Sub(Reg, Reg, Op2),

  And(Reg, Reg, Op2),
  Or(Reg, Reg, Op2),

  Cmp(Reg, Op2),
  MovEq(Reg, Op2), //moves depending on flag
  MovNe(Reg, Op2),
  MovGe(Reg, Op2),
  MovLt(Reg, Op2),
  MovGt(Reg, Op2),
  MovLe(Reg, Op2),

  BranchOverflow(Branch),

  AddFlags(Reg, Reg, Op2),
  SubFlags(Reg, Reg, Op2),

  Multiply(Reg, Reg, Reg, Reg),

  BranchNotEqual(Branch),
  ReverseSubtract(Reg, Reg, Op2),
}

impl Display for Instr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Instr::LoadImm(reg, val) => write!(f, "LDR {}, ={}", reg, val),
      Instr::Mov(reg, op2) => write!(f, "MOV {}, {}", reg, op2),
      Instr::Branch(branch) => write!(f, "BL {}", branch),
      Instr::Pop => write!(f, "POP {{pc}}"),
      Instr::Assemble => write!(f, ".ltorg"),
      Instr::StoreByte(reg, mem_addr) => write!(f, "STRB {}, {}", reg, mem_addr),
      Instr::Store(reg, mem_addr) => write!(f, "STRB {}, {}", reg, mem_addr),
      Instr::LoadMemByte(reg, mem_addr) => write!(f, "LDRSB {}, {}", reg, mem_addr),
      Instr::Add(r1, r2, op2) => write!(f, "ADD {}, {}, {}", r1, r2, op2),
      Instr::Sub(r1, r2, op2) => write!(f, "SUB {}, {}, {}", r1, r2, op2),
      Instr::And(r1, r2, op2) => write!(f, "AND {}, {}, {}", r1, r2, op2),
      Instr::Or(r1, r2, op2) => write!(f, "ORR {}, {}, {}", r1, r2, op2),
      Instr::Cmp(reg, op2) => write!(f, "CMP {}, {}", reg, op2),
      Instr::MovEq(reg, op2) => write!(f, "MOVEQ {}, {}", reg, op2),
      Instr::MovNe(reg, op2) => write!(f, "MOVNE {}, {}", reg, op2),
      Instr::MovGe(reg, op2) => write!(f, "MOVGE {}, {}", reg, op2),
      Instr::MovLt(reg, op2) => write!(f, "MOVLT {}, {}", reg, op2),
      Instr::MovGt(reg, op2) => write!(f, "MOVGT {}, {}", reg, op2),
      Instr::MovLe(reg, op2) => write!(f, "MOVLE {}, {}", reg, op2),
      Instr::BranchOverflow(branch) => write!(f, "BLVS {}", branch),
      Instr::AddFlags(r1, r2, op2) => write!(f, "ADDS {}, {}, {}", r1, r2, op2),
      Instr::SubFlags(r1, r2, op2) => write!(f, "SUBS {}, {}, {}", r1, r2, op2),
      Instr::Multiply(r1, r2, r3, r4) => write!(f, "SMULL {}, {}, {}, {}", r1, r2, r3, r4),
      Instr::ReverseSubtract(r1, r2, op2) => write!(f, "RSBS {}, {}, {}", r1, r2, op2),
      Instr::BranchNotEqual(branch) => write!(f, "BLNE {}", branch),
    }
  }
}
