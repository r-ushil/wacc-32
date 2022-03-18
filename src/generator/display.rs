use super::*;
use std::fmt::Display;

/* This file describes how the asm instructions and programs are
converted to text for an assembly file. */

/* ======== Represents entire program. ======== */

impl Display for GeneratedCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.data.len() > 1 {
      /* If there is only a single thing in data, that's ".data", and there's
      no point outputting that if there is nothing in the data segment. */
      self
        .data
        .iter()
        .try_for_each(|asm| writeln!(f, "{}", asm))?;

      writeln!(f)?;
    }

    /* Always output .text segment. */
    self
      .text
      .iter()
      .try_for_each(|asm| writeln!(f, "{}", asm))?;

    /* Display print statements. */

    Ok(())
  }
}

/* ======== Represents line within produced assembly apart from instructions.  ======== */

impl Display for Asm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Instr::*;
    match self {
      Asm::Directive(d) => write!(f, "{}", d),
      Asm::Instr(cond, i) => {
        write!(f, "\t")?;
        match i {
          Push(reg) => write!(f, "PUSH{} {{{}}}", cond, reg.get()),

          Pop(reg) => write!(f, "POP{} {{{}}}", cond, reg.get()),

          Branch(link, label) => {
            write!(f, "B{}{} {}", if *link { "L" } else { "" }, cond, label)
          }

          Store(size, dst, (src, off), addr_mode) => {
            write!(f, "STR{}{} {}, ", cond, size, dst.get())?;

            match addr_mode {
              AddressingMode::Default => {
                if *off == 0 {
                  write!(f, "[{}]", src.get())
                } else {
                  write!(f, "[{}, #{}]", src.get(), off)
                }
              }
              AddressingMode::PreIndexed => {
                write!(f, "[{}, #{}]!", src.get(), off)
              } // AddressingMode::PostIndexed => write!(f, "[{}], #{}", src, off),   unused
            }
          }

          Load(size, dst, load_arg) => {
            let ldr_sign_extend = match size {
              DataSize::Byte => "S",
              _ => "",
            };

            write!(
              f,
              "LDR{}{}{} {}, {}",
              ldr_sign_extend,
              size,
              cond,
              dst.get(),
              load_arg
            )
          }

          Binary(instr, dst, src, op2, flags) => {
            write!(
              f,
              "{}{}{} {}, {}, {}",
              instr,
              if *flags { "S" } else { "" },
              cond,
              dst.get(),
              src.get(),
              op2
            )
          }

          Unary(instr, dst, op2, flags) => {
            write!(
              f,
              "{}{}{} {}, {}",
              instr,
              if *flags { "S" } else { "" },
              cond,
              dst.get(),
              op2
            )
          }

          Multiply(r1, r2, r3, r4) => {
            write!(
              f,
              "SMULL{} {}, {}, {}, {}",
              cond,
              r1.get(),
              r2.get(),
              r3.get(),
              r4.get()
            )
          }
          BranchReg(link, reg) => {
            write!(
              f,
              "B{}X{} {}",
              if *link { "L" } else { "" },
              cond,
              reg.get()
            )
          }
        }
      }
    }
  }
}

impl Display for Directive {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Directive::*;
    match self {
      Text => write!(f, ".text\n\n.global main"),
      Data => writeln!(f, ".data"),
      Assemble => write!(f, "\t.ltorg"),
      Label(l) => write!(f, "{}:", l),
      Word(n) => write!(f, "\t.word {}", n),
      Ascii(s) => write!(f, "\t.ascii\t\"{}\"", s),
    }
  }
}

impl Display for LoadArg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LoadArg::Imm(val) => write!(f, "={}", val),
      LoadArg::MemAddress(reg, offset) => {
        if *offset == 0 {
          write!(f, "[{}]", reg.get())
        } else {
          write!(f, "[{}, #{}]", reg.get(), offset)
        }
      }
      LoadArg::Label(msg) => write!(f, "={}", msg),
    }
  }
}

/* ======== Instructions! ======== */

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
      Add => "ADD",
      Sub => "SUB",
      RevSub => "RSB",
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
      Imm(val) => write!(f, "#{}", val),
      Reg(reg, shift) => {
        write!(f, "{}", reg.get())?;
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
      // HS => "HS",     unused
      // CC => "CC",     unused
      // LO => "LO",     unused
      // MI => "MI",     unused
      // PL => "PL",     unused
      VS => "VS",
      // VC => "VC",     unused
      // HI => "HI",     unused
      // LS => "LS",     unused
      GE => "GE",
      LT => "LT",
      GT => "GT",
      LE => "LE",
      AL => "",
    };
    write!(f, "{}", s)
  }
}

impl Display for Reg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Reg::*;
    match self {
      StackPointer => write!(f, "sp"),
      Link => write!(f, "lr"),
      PC => write!(f, "pc"),
      Arg(arg_reg) => write!(f, "{}", arg_reg),
      General(gen_reg) => write!(f, "{}", gen_reg),
      // Virtual(_) => panic!("A virtual register made it to assembly!"),
      Virtual(v) => write!(f, "v{}", v),
    }
  }
}

impl Display for GenReg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      GenReg::R4 => write!(f, "r4"),
      GenReg::R5 => write!(f, "r5"),
      GenReg::R6 => write!(f, "r6"),
      GenReg::R7 => write!(f, "r7"),
      GenReg::R8 => write!(f, "r8"),
      GenReg::R9 => write!(f, "r9"),
      GenReg::R10 => write!(f, "r10"),
      GenReg::R11 => write!(f, "r11"),
    }
  }
}

impl Display for ArgReg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ArgReg::R0 => write!(f, "r0"),
      ArgReg::R1 => write!(f, "r1"),
      ArgReg::R2 => write!(f, "r2"),
      // ArgReg::R3 => write!(f, "r2"),        unused
    }
  }
}

pub fn unescape_char(ch: char) -> Option<&'static str> {
  match ch {
    '\0' => Some("\\0"),
    '\u{8}' => Some("\\b"),
    '\t' => Some("\\t"),
    '\n' => Some("\\n"),
    '\u{c}' => Some("\\f"),
    '\r' => Some("\\r"),
    '\"' => Some("\\\""),
    '\'' => Some("\\\'"),
    '\\' => Some("\\\\"),
    _ => None,
  }
}
