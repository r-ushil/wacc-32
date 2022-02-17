pub type RegNum = u8;
pub type ExitCode = u8;
pub type Dst = Reg;
pub type Src = Reg;
pub type Branch = String;
pub type Imm = i32;

pub enum Val {
  Reg(Reg),
  Imm(Imm),
}

pub enum Reg {
  RegNum(RegNum),
  LeftShiftedReg(RegNum, Imm),
  RightShiftedReg(RegNum, Imm),
  StackPointer(StackPointer),
}

pub struct MemAddress {
  pub reg: Reg,
  pub offset: Option<i32>,
}

pub enum Instr {
  LoadImm(Reg, Imm),
  Mov(Dst, Src),
  Branch(Branch),
  Pop,
  Assemble, //.ltorg

  StoreByte(Reg, MemAddress),
  Store(Reg, MemAddress),

  Add(Dst, Src, Imm),
  LoadMemByte(Reg, MemAddress),
  And(Dst, Src, Src),
  Sub(Dst, Src, Imm),
  Or(Dst, Src, Src),

  Cmp(Src, Src),
  MovEq(Dst, Val), //moves depending on flag
  MovNe(Dst, Val),
  MovGe(Dst, Val),
  MovLt(Dst, Val),
  MovGt(Dst, Val),
  MovLe(Dst, Val),

  BranchOverflow(Branch),

  AddFlags(Dst, Src, Src),
  SubFlags(Dst, Src, Src),

  Multiply(Dst, Dst, Src, Src),
  CompareRightShift(Src, Src, Imm),

  BranchNotEqual(Branch),
  ReverseSubtract(Dst, Src, Imm),
}
