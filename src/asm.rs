use std::fmt::Display;

pub type RegNum = u8;
pub type ExitCode = u8;
pub type Branch = String;
pub type Imm = i32;

pub enum Op2 {
  Imm(Imm),
  RegNum(RegNum),
  LeftShiftedReg(RegNum, Imm),
  RightShiftedReg(RegNum, Imm),
  StackPointer,
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
      Instr::Mov(_, _) => todo!(),
      Instr::Branch(_) => todo!(),
      Instr::Pop => todo!(),
      Instr::Assemble => todo!(),
      Instr::StoreByte(_, _) => todo!(),
      Instr::Store(_, _) => todo!(),
      Instr::LoadMemByte(_, _) => todo!(),
      Instr::Add(_, _, _) => todo!(),
      Instr::Sub(_, _, _) => todo!(),
      Instr::And(_, _, _) => todo!(),
      Instr::Or(_, _, _) => todo!(),
      Instr::Cmp(_, _) => todo!(),
      Instr::MovEq(_, _) => todo!(),
      Instr::MovNe(_, _) => todo!(),
      Instr::MovGe(_, _) => todo!(),
      Instr::MovLt(_, _) => todo!(),
      Instr::MovGt(_, _) => todo!(),
      Instr::MovLe(_, _) => todo!(),
      Instr::BranchOverflow(_) => todo!(),
      Instr::AddFlags(_, _, _) => todo!(),
      Instr::SubFlags(_, _, _) => todo!(),
      Instr::Multiply(_, _, _, _) => todo!(),
      Instr::BranchNotEqual(_) => todo!(),
      Instr::ReverseSubtract(_, _, _) => todo!(),
    }
  }
}

fn main() {
  //for instruction in instructions {
  //   write!(f, "{}\n", instruction);
  //}
}
