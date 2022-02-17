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
