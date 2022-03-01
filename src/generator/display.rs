use super::*;
use std::fmt::Display;

/* This file describes how the asm instructions and programs are
converted to text for an assembly file. */

/* ======== Represents entire program. ======== */

impl Display for GeneratedCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self
      .data
      .iter()
      .try_for_each(|asm| write!(f, "{}\n", asm))?;
    self
      .text
      .iter()
      .try_for_each(|asm| write!(f, "{}\n", asm))?;

    /* Display print statements. */

    Ok(())
  }
}

/* ======== Represents line within produced assembly apart from instructions.  ======== */

impl Display for Asm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Asm::Directive(d) => write!(f, "{}", d),
      Asm::Instr(cond, i) => write!(f, "  {}{}", cond, i),
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

impl Display for MemAddress {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.offset.is_none() {
      write!(f, "{}", self.reg)
    } else {
      write!(f, "[{}, #{}", self.reg, self.offset.unwrap())
    }
  }
}

impl Display for LoadArg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LoadArg::Imm(val) => write!(f, "#{}", val),
      LoadArg::MemAddress(addr) => write!(f, "{}", addr),
      LoadArg::Label(msg) => write!(f, "={}", msg),
    }
  }
}

/* ======== Instructions! ======== */

impl Display for Instr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Instr::*;
    match self {
      Push(reg) => write!(f, "PUSH {{{}}}", reg),

      Pop(reg) => write!(f, "POP {{{}}}", reg),

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

      Load(size, dst, load_arg) => write!(f, "LDR{} {}, {}", size, dst, load_arg),

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

impl Display for DataSize {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use DataSize::*;
    match self {
      Byte => write!(f, "B"),
      Word => Ok(()),
    }
  }
}

impl Display for UnaryInstr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use UnaryInstr::*;
    let tag = match self {
      Mov => "MOV",
      Cmp => "CMP",
    };
    write!(f, "{}", tag)
  }
}

impl Display for BinaryInstr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use BinaryInstr::*;
    let tag = match self {
      Add => "ADDS",
      Sub => "SUB",
      RevSub => "RSBS",
      And => "AND",
      Or => "ORR",
      Eor => "EOR",
    };
    write!(f, "{}", tag)
  }
}

impl Display for Op2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Op2::*;
    match self {
      Imm(val) => write!(f, "={}", val),
      Reg(reg, shift) => {
        write!(f, "{}", reg)?;
        if *shift > 0 {
          write!(f, ", ASR #{}", shift)?;
        } else if *shift < 0 {
          write!(f, ", LSL #{}", -shift)?;
        }
        Ok(())
      }
      Char(ch) => write!(f, "#'{}'", ch),
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
    use Reg::*;
    match self {
      RegNum(num) => write!(f, "r{}", num),
      StackPointer => write!(f, "sp"),
      Link => write!(f, "lr"),
      PC => write!(f, "pc"),
    }
  }
}
