use super::*;

impl Generatable for AssignLhs {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for AssignRhs {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for PairElem {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for ArrayLiter {
  // fn generate(&self, _code: &mut Vec<Instr>, _registers: &[Reg]) {}
}

impl Generatable for Stat {
  fn generate(&self, code: &mut GeneratedCode, min_regs: &mut u8) {
    match self {
      Stat::Skip => (),
      Stat::Declaration(_, _, _) => todo!(),
      Stat::Assignment(_, _) => todo!(),
      Stat::Read(_) => todo!(),
      Stat::Free(_) => todo!(),
      Stat::Return(_) => todo!(),

      Stat::Exit(expr) => {
        // match expr {
        //   Expr::IntLiter(exit_code) => {
        //     code.text.push(Instr::Load(
        //       DataSize::Word,
        //       Reg::RegNum(*min_regs),
        //       Load::Imm(*exit_code),
        //     ));
        //     code.text.push(Instr::Mov(
        //       Reg::RegNum(0),
        //       Op2::Reg(Reg::RegNum(*min_regs)),
        //       CondCode::AL,
        //     ));
        //     //*min_regs += 1; don't need to increment!
        //     code
        //       .text
        //       .push(Instr::Branch(String::from("exit"), CondCode::AL));
        //   }
        //   _ => unreachable!("Unreachable Syntax Error"),
        // }
        todo!();
      }

      Stat::Print(expr) => match expr {
        Expr::IntLiter(_) => todo!(),
        Expr::BoolLiter(_) => todo!(),
        Expr::CharLiter(_) => todo!(),
        Expr::StrLiter(msg) => todo!(),
        Expr::PairLiter => todo!(),
        Expr::Ident(_) => todo!(),
        Expr::ArrayElem(_) => todo!(),
        Expr::UnaryApp(_, _) => todo!(),
        Expr::BinaryApp(_, _, _) => todo!(),
      },
      Stat::Println(_) => todo!(),
      Stat::If(_, _, _) => todo!(),
      Stat::While(_, _) => todo!(),
      Stat::Scope(_) => todo!(),
      Stat::Sequence(_, _) => todo!(),
    }
  }
}

/*

1) generate code using B print_int
1.5) stat.generate(cod)
2) mark the fact we need it to exist // code.prints.int = true
...
100) once code generated, generate all the things which need to exist
101) if code.prints.int == true { print_int(code) }

*/

// fn print_int(code: &mut GeneratedCode) {
//   use Instr::*;

//   code.data.push(Label(String::from("msg_int")));
//   code.data.push(Word(3));
//   code.data.push(Ascii(String::from("%d\\0")));

//   code.text.push(Label(String::from("p_print_int")));
//   code.text.push(Push);
//   code
//     .text
//     .push(Mov(Reg::RegNum(1), Op2::Reg(Reg::RegNum(0)), CondCode::AL));
//   code.text.push(LoadImm(
//     Reg::RegNum(0),
//     Load::Label(String::from("msg_int")),
//   ));
//   code
//     .text
//     .push(Add(Reg::RegNum(0), Reg::RegNum(0), Op2::Imm(4)));
//   code.text.push(Branch(String::from("printf"), CondCode::AL));
//   code
//     .text
//     .push(Mov(Reg::RegNum(0), Op2::Imm(0), CondCode::AL));
//   code.text.push(Branch(String::from("fflush"), CondCode::AL));
//   code.text.push(Pop);
// }

#[cfg(test)]
mod tests {

  use super::*;
  use Instr::*;

  #[test]
  fn exit_statement() {
    let exit_code = 0;
    let stat = Stat::Exit(Expr::IntLiter(exit_code));
    let mut min_regs = 4;

    let mut actual_code = GeneratedCode {
      data: vec![],
      text: vec![],
    };
    stat.generate(&mut actual_code, &mut min_regs);

    let expected_code = GeneratedCode {
      data: vec![],
      text: vec![
        /* LDR r4, #0 */
        Asm::Instr(
          CondCode::AL,
          Unary(UnaryInstr::Mov, Reg::RegNum(4), Op2::Imm(0), false),
        ),
        /* MOV r0, r4 */
        Asm::Instr(
          CondCode::AL,
          Unary(
            UnaryInstr::Mov,
            Reg::RegNum(0),
            Op2::Reg(Reg::RegNum(4), 0),
            false,
          ),
        ),
        Asm::Instr(CondCode::AL, Branch(false, String::from("exit"))),
      ],
    };

    assert_eq!(actual_code, expected_code);
  }

  // #[test]
  // fn if_statement() {
  //   let cond = Expr::BoolLiter(true); // true
  //   let true_body = Stat::Println(Expr::StrLiter(String::from("True Body"))); // println "True Body"
  //   let false_body = Stat::Println(Expr::StrLiter(String::from("False Body"))); // println "False Body"

  //   let if_statement = Stat::If(
  //     // if
  //     cond.clone(),                 // true
  //     Box::new(true_body.clone()),  // then println "True Body"
  //     Box::new(false_body.clone()), // else println "False Body"
  //   ); // fi

  //   let mut min_regs = 4;

  //   let actual_code = &mut vec![];
  //   if_statement.generate(actual_code, &mut min_regs);

  //   let expected_code = &mut vec![];
  //   cond.generate(expected_code, &mut min_regs);
  //   expected_code.push(Cmp(Reg::RegNum(4), Op2::Imm(0))); // CMP r4, #0
  //   expected_code.push(Branch(String::from("L0"), CondCode::EQ)); // BEQ L0
  //   true_body.generate(expected_code, &mut min_regs);
  //   expected_code.push(Branch(String::from("L1"), CondCode::AL)); // B L1
  //   expected_code.push(Label(String::from("L0"))); // LO:
  //   false_body.generate(expected_code, &mut min_regs);
  //   expected_code.push(Label(String::from("L1"))); // LO:

  //   assert_eq!(actual_code, expected_code);
  // }
}
