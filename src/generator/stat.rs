use super::Generatable;
use crate::ast::*;
use crate::generator::asm::*;

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
        /* Evalutates expression into min_reg */
        expr.generate(code, min_regs);

        /* MOV r0, r{min_reg} */
        code.text.push(Asm::Instr(
          CondCode::AL,
          Instr::Unary(
            UnaryInstr::Mov,
            Reg::RegNum(0),
            Op2::Reg(Reg::RegNum(*min_regs), 0),
            false,
          ),
        ));

        /* B exit */
        code.text.push(Asm::Instr(
          CondCode::AL,
          Instr::Branch(false, String::from("exit")),
        ));
      }

      Stat::Print(expr) => match expr {
        Expr::IntLiter(_) => todo!(),
        Expr::BoolLiter(_) => todo!(),
        Expr::CharLiter(_) => todo!(),
        Expr::StrLiter(_) => todo!(),
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn exit_statement() {
    use self::Instr::*;

    let expr = Expr::IntLiter(0);
    let stat = Stat::Exit(expr.clone());
    let mut min_regs = 4;

    /* Actual output. */
    let mut actual_code = GeneratedCode {
      data: vec![],
      text: vec![],
      print_branches: GeneratePredefs {
        print_ints: false,
        print_strings: false,
        print_bools: false,
        print_refs: false,
        println: false,
        read_char: false,
        read_int: false,
        free_pair: false,
        runtime_err: false,
      },
    };
    stat.generate(&mut actual_code, &mut min_regs);

    /* Expected output. */
    let mut expected_code = GeneratedCode {
      data: vec![],
      text: vec![],
      print_branches: GeneratePredefs::default(),
    };
    expr.generate(&mut expected_code, &mut min_regs); // <= important line

    /* MOV r0, r4 */
    expected_code.text.push(Asm::Instr(
      CondCode::AL,
      Unary(
        UnaryInstr::Mov,
        Reg::RegNum(0),
        Op2::Reg(Reg::RegNum(4), 0),
        false,
      ),
    ));

    /* B exit */
    expected_code.text.push(Asm::Instr(
      CondCode::AL,
      Branch(false, String::from("exit")),
    ));

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
