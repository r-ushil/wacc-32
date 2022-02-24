use super::*;
use std::fmt::Display;

/* This file describes how the asm instructions and programs are
converted to text for an assembly file. */

impl Display for Asm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Asm::Directive(d) => write!(f, "{}", d),
      Asm::Instr(cond, i) => write!(f, "{}{}", cond, i),
    }
  }
}

impl Display for Op2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Op2::Imm(val) => write!(f, "={}", val),
      Op2::Reg(reg, shift) => {
        write!(f, "{}", reg)?;
        if *shift > 0 {
          write!(f, ", ASR #{}", shift)?;
        } else if *shift < 0 {
          write!(f, ", LSL #{}", -shift)?;
        }
        Ok(())
      }
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

impl Display for Load {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Load::Imm(val) => write!(f, "{}", val),
      Load::Label(msg) => write!(f, "{}", msg),
    }
  }
}

impl Display for Reg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Reg::RegNum(num) => write!(f, "r{}", num),
      Reg::StackPointer => write!(f, "sp"),
    }
  }
}

impl Display for Directive {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Directive::*;
    match self {
      Text => write!(f, ".text"),
      Data => write!(f, ".data"),
      Assemble => write!(f, ".ltorg"),
      Label(l) => write!(f, "{}:", l),
      Word(n) => write!(f, ".word {}", n),
      Ascii(s) => write!(f, ".ascii \"{}\"", s),
    }
  }
}

impl Display for DataSize {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl Display for BinaryInstr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let tag = match self {
      Add => "ADDS",
      Sub => "SUB",
      RevSub => "RSBS",
      And => "AND",
      Or => "ORR",
    };
    write!(f, "{}", tag)
  }
}

impl Display for UnaryInstr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let tag = match self {
      Mov => "MOV",
      Cmp => "CMP",
    };
    write!(f, "{}", tag)
  }
}

impl Display for Instr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Instr::*;
    match self {
      Push => write!(f, "PUSH {{lr}}"),

      Pop => write!(f, "POP {{pc}}"),

      Branch(link, label) => {
        write!(f, "B{} {}", if *link { "L" } else { "" }, label)
      }

      Store(size, dst, (src, off)) => {
        write!(f, "STR{} {}, ", size, dst)?;
        if *off == 0 {
          write!(f, "{}", src)
        } else {
          write!(f, "[{}, #{}]", src, off)
        }
      }

      Load(size, dst, (src, off)) => {
        write!(f, "LDRS{} {}, ", size, dst)?;
        if *off == 0 {
          write!(f, "{}", src)
        } else {
          write!(f, "[{}, #{}]", src, off)
        }
      }

      Binary(instr, dst, src, op2, flags) => {
        write!(
          f,
          "{}{} {}, {}, {}",
          instr,
          if *flags { "S" } else { "" },
          dst,
          src,
          op2
        )
      }

      Unary(instr, dst, op2, flags) => {
        write!(
          f,
          "{}{} {}, {}",
          instr,
          if *flags { "S" } else { "" },
          dst,
          op2
        )
      }

      Multiply(r1, r2, r3, r4) => write!(f, "SMULL {}, {}, {}, {}", r1, r2, r3, r4),
    }
  }
}
