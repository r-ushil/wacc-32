use super::{predef::ReadFmt, *};
use Directive::*;
use Instr::*;

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
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg]) {
    match self {
      Stat::Skip => (),
      // Stat::Declaration(_, _, _) => todo!(),
      // Stat::Assignment(_, _) => todo!(),
      // Stat::Read(expr) => {
      // TODO: expr is not and Expr or an Ident, function needs re-writing.
      // // expr is expected to be an identifier, needs to read into a variable
      // expr.generate(scope, code, regs); //generate expr, load into min_reg

      // /* MOV r0, {min_reg} */
      // code.text.push(Asm::Instr(
      //   CondCode::AL,
      //   Instr::Unary(
      //     UnaryInstr::Mov,
      //     Reg::General(0),
      //     Op2::Reg(Reg::General(*regs), 0),
      //     false,
      //   ),
      // ));
      // //expr.get_type //todo!() get type of ident
      // let read_type = if true {
      //   code.predefs.read_char = true;
      //   ReadFmt::Char
      // } else {
      //   code.predefs.read_int = true;
      //   ReadFmt::Int
      // }; //replace true with expr type check

      // /* BL p_read_{read_type} */
      // code.text.push(Asm::Instr(
      //   CondCode::AL,
      //   Instr::Branch(true, format!("p_read_{}", read_type)),
      // ));

      // *regs = *regs - 1; //decrement min_reg by 1, no longer needed
      // }
      // Stat::Free(expr) => {
      // TODO: expr is not and Expr or an Ident, function needs re-writing.
      // //expr must be of type ident, referring to a pair

      // expr.generate(scope, code, regs); //load pair address into min_reg
      //                                   /* MOV r0, {min_reg}        //move pair address into r0 */
      // code.text.push(Asm::Instr(
      //   CondCode::AL,
      //   Instr::Unary(
      //     UnaryInstr::Mov,
      //     Reg::General(0),
      //     Op2::Reg(Reg::General(*regs), 0),
      //     false,
      //   ),
      // ));

      // code.predefs.free_pair = true; //set free_pair flag to true
      //                                /* BL p_free_pair */
      // code.text.push(Asm::Instr(
      //   CondCode::AL,
      //   Instr::Branch(true, String::from("p_free_pair")),
      // ));

      // *regs = *regs - 1; //decrement min_reg by 1, no longer needed
      // }
      Stat::Return(expr) => {
        /* regs[0] = eval(expr) */
        expr.generate(scope, code, regs);

        /* r0 = regs[0] */
        code.text.push(Asm::Instr(
          CondCode::AL,
          Instr::Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Reg(regs[0], 0), false),
        ));

        // todo!()
        // total_offset = somehow get total stack offset for all local vars
        let total_offset = 0;

        /* ADD sp, sp, #{total_offset} */
        code.text.push(Asm::Instr(
          CondCode::AL,
          Instr::Binary(
            BinaryInstr::Add,
            Reg::StackPointer,
            Reg::StackPointer,
            Op2::Imm(total_offset),
            false,
          ),
        ));

        /* POP {pc} */
        code
          .text
          .push(Asm::Instr(CondCode::AL, Instr::Pop(Reg::PC)));

        /* POP {pc} */
        code
          .text
          .push(Asm::Instr(CondCode::AL, Instr::Pop(Reg::PC)));
      }
      Stat::Exit(expr) => {
        /* regs[0] = eval(expr) */
        expr.generate(scope, code, regs);

        /* r0 = regs[0] */
        code.text.push(Asm::Instr(
          CondCode::AL,
          Instr::Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Reg(regs[0], 0), false),
        ));

        /* B exit */
        code.text.push(Asm::Instr(
          CondCode::AL,
          Instr::Branch(true, String::from("exit")),
        ));
      }

      // Stat::Print(expr) => {
      //   expr.generate(scope, code, min_reg);
      //   todo!(); //get type of expr, and switch to the appropriate print branch

      //   // print_stat_gen(code, expr.get_type);
      // }

      // Stat::Println(expr) => {
      //   expr.generate(scope, code, min_reg);
      //   todo!();
      //   // print_stat_gen(code, expr.get_type);
      //   // code.predefs.println = true;
      //   // /* BL println */
      //   // code.text.push(Asm::Instr(CondCode::AL, Instr::Branch(true, String::from("println"))));
      // }
      // Stat::If(_, _, _) => todo!(),
      // Stat::While(_, _) => todo!(),
      // Stat::Scope(_) => todo!(),
      Stat::Sequence(head, tail) => {
        head.generate(scope, code, regs);
        tail.generate(scope, code, regs);
      }
      _ => code.text.push(Asm::Directive(Directive::Label(format!(
        "{:?}.generate(_, {:?})",
        self, regs
      )))),
    }
  }
}

// todo!(), add parameter for expr_type
fn print_stat_gen(code: &mut GeneratedCode, min_reg: &mut RegNum) {

  //   let branch_name = match expr_type {
  //     Type::String => {
  //       code.predefs.print_strings = true;
  //       String::from("p_print_string")
  //     }
  //     Type::Bool => {
  //       code.predefs.print_bools = true;
  //       String::from("p_print_bool")
  //     }
  //     Type::Int => {
  //       code.predefs.print_ints = true;
  //       String::from("p_print_int")
  //     }
  //     Type::Ref => {
  //       code.predefs.print_refs = true;
  //       String::from("p_print_reference")
  //     }
  //   };

  // /* MOV r0, min_reg */
  // code.text.push(Asm::Instr(CondCode::AL, Instr::Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Reg(Reg::RegNum(*min_reg), 0), false)));

  // /* BL {branch_name} */
  // code.text.push(Asm::Instr(CondCode::AL, Instr::Branch(true, branch_name)));

  // *min_reg = *min_reg - 1; //decrement min_reg by 1, no longer needed
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
    let symbol_table = SymbolTable::default();
    let scope = &Scope::new(&symbol_table);
    let expr = Expr::IntLiter(0);
    let stat = Stat::Exit(expr.clone());
    let regs = &GENERAL_REGS;

    /* Actual output. */
    let mut actual_code = GeneratedCode::default();
    stat.generate(scope, &mut actual_code, regs);

    /* Expected output. */
    let mut expected_code = GeneratedCode::default();
    expr.generate(scope, &mut expected_code, regs);

    /* MOV r0, r4 */
    expected_code.text.push(Asm::Instr(
      CondCode::AL,
      Instr::Unary(
        UnaryInstr::Mov,
        Reg::RegNum(0),
        Op2::Reg(Reg::RegNum(4), 0),
        false,
      ),
    ));

    /* B exit */
    expected_code.text.push(Asm::Instr(
      CondCode::AL,
      Instr::Branch(false, String::from("exit")),
    ));

    assert_eq!(actual_code, expected_code);
  }

  // #[test]
  // fn if_statement() {
  //   let cond = Expr::BoolLiter(true); // true
  //   let true_body = Stat::Println(Expr::StrLiter(String::from("True Body"))); // println "True Body"
  //   let false_body = Stat::Println(Expr::StrLiter(String::from("False Body"))); // println "False Body"

  //   let if_statement = Stat::If(
  //     cond.clone(),                 // if true
  //     Box::new(true_body.clone()),  // then println "True Body"
  //     Box::new(false_body.clone()), // else println "False Body"
  //   ); // fi

  //   let min_reg = &mut 4;

  //   let actual_code = &mut GeneratedCode::default();
  //   if_statement.generate(actual_code, min_reg);

  //   let expected_code = &mut GeneratedCode::default();
  //   let l0 = expected_code.get_label();
  //   let l1 = expected_code.get_label();

  //   /* Condition. */
  //   cond.generate(expected_code, min_reg);

  //   /* Is condition == 0? */
  //   expected_code.text.push(Asm::always(Unary(
  //     UnaryInstr::Cmp,
  //     Reg::RegNum(4),
  //     Op2::Imm(0),
  //     false,
  //   )));

  //   /* Branch to false case if cond == 0. */
  //   expected_code
  //     .text
  //     .push(Asm::always(Branch(false, l0.clone())));

  //   /* True body. */
  //   true_body.generate(expected_code, min_reg);
  //   /* Exit if statement. */
  //   expected_code
  //     .text
  //     .push(Asm::always(Branch(false, l1.clone())));

  //   /* Label for false case to skip to. */
  //   expected_code.text.push(Asm::Directive(Label(l0)));

  //   /* False body. */
  //   false_body.generate(expected_code, min_reg);

  //   /* Label to exit if statement. */
  //   expected_code.text.push(Asm::Directive(Label(l1)));

  //   assert_eq!(actual_code, expected_code);
  // }
}
