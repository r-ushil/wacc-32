use std::fmt::Display;

use super::*;
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
          Unary(
            UnaryInstr::Mov,
            Reg::RegNum(0),
            Op2::Reg(Reg::RegNum(*min_regs), 0),
            false,
          ),
        ));

        /* B exit */
        code.text.push(Asm::Instr(
          CondCode::AL,
          Branch(false, String::from("exit")),
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

fn free_pair(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label to display in an attempt to free a null pair */

  /* msg_null_deref: */
  code
    .data
    .push(Directive(Label("msg_null_deref".to_string())));
  /* .word 50                   //allocate space for a word of size 50 */
  code.data.push(Directive(Word(50)));
  /* .ascii "NullReferenceError: ...\n\0"         //convert into ascii */
  code.data.push(Directive(Ascii(String::from(
    "NullReferenceError: dereference a null reference\\n\\0",
  ))));

  /* Generate the p_free_pair label to free the pair in r0, predefined */

  /* p_free_pair: */
  code
    .text
    .push(Directive(Label(String::from("p_free_pair"))));
  /*  PUSH {lr}            //push link reg */
  code.text.push(Instr(AL, Push));
  /*  CMP r0, #0           //compare the contents of r0 to 0 and set flags */
  code.text.push(Instr(
    AL,
    Unary(UnaryInstr::Cmp, Reg::RegNum(0), Op2::Imm(0), false),
  ));
  /*  LDREQ r0, =msg_null_deref   //load deref msg if r0 equals 0 */
  code.text.push(Instr(
    EQ,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::Label(String::from("msg_null_deref")),
    ),
  ));
  /*  BEQ p_throw_runtime_error   //branch to runtime error if r0 equals 0 */
  code.text.push(Instr(
    EQ,
    Branch(true, String::from("p_throw_runtime_error")),
  ));
  /*  PUSH {r0}           //push r0 */
  code.text.push(Instr(AL, Push)); //todo!() fix Push to take in Reg parameter
                                   /*  LDR r0, [sp]        //load stack pointer address into r0 */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::MemAddress(MemAddress {
        reg: Reg::StackPointer,
        offset: None,
      }),
    ),
  ));
  /*  LDR r0, [r0, #4]    //load address of r0+4 into r0 */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::MemAddress(MemAddress {
        reg: Reg::RegNum(0),
        offset: Some(4),
      }),
    ),
  ));
  /*  BL free             //branch to free */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("free"))));
  /*  POP {r0}            //todo!() fix Pop to take in Reg parameter */
  code.text.push(Instr(AL, Pop));
  /*  BL free             //branch to free */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("free"))));
  /*  POP {pc}            //todo!() fix Pop to take in Reg parameter */
  code.text.push(Instr(AL, Pop));
}

fn throw_runtime_error(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Generate label to throw a runtime error for whatever's in registers */
  /* p_throw_runtime_error: */
  code
    .text
    .push(Directive(Label(String::from("p_throw_runtime_error"))));
  /* BL p_print_string        //branch to print a string */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("p_print_string"))));
  /* MOV r0, #-1              //move -1 into r0*/
  code.text.push(Instr(
    AL,
    Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Imm(-1), false),
  ));
  /* BL exit                  //exit with status code -1  */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("exit"))));
}

fn print_bool(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create the msg label to display string data for TRUE and add to the
  GeneratedCode data member: */

  /* msg_true: */
  code.data.push(Directive(Label(String::from("msg_true"))));
  /* .word 5                   //allocate space for a word of size 5 */
  code.data.push(Directive(Word(5)));
  /* .ascii "true\0"           //convert into ascii */
  code.data.push(Directive(Ascii(String::from("true\\0"))));

  /* Create the msg label to display string data for FALSE and add to the
  GeneratedCode data member: */

  /* msg_false: */
  code.data.push(Directive(Label(String::from("msg_false"))));
  /* .word 6                   //allocate space for a word of size 6 */
  code.data.push(Directive(Word(6)));
  /* .ascii "false\0"           //convert into ascii */
  code.data.push(Directive(Ascii(String::from("false\\0"))));

  /* Generate the p_print_bool label to print bool, predefined and the same
  for every program. */

  /*p_print_bool: */
  code
    .text
    .push(Directive(Label(String::from("p_print_bool"))));
  /*  PUSH {lr}             //push link reg */
  code.text.push(Instr(AL, Push));
  /*  CMP r0, #0            //compare the contents of r0 to 0 and set flags */
  code.text.push(Instr(
    AL,
    Unary(UnaryInstr::Cmp, Reg::RegNum(0), Op2::Imm(0), false),
  ));
  /*  LDRNE r0, =msg_true   //load result of msg_true if not equal to r0  */
  code.data.push(Instr(
    NE,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::Label(String::from("msg_true")),
    ),
  ));
  /*  LDREQ r0, =msg_false   //load result of msg_false if equal to r0  */
  code.text.push(Instr(
    EQ,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::Label(String::from("msg_false")),
    ),
  ));
  /*  ADD r0, r0, #4        //add 4 to r0 and store in r0 */
  code.text.push(Instr(
    AL,
    Binary(
      BinaryInstr::Add,
      Reg::RegNum(0),
      Reg::RegNum(0),
      Op2::Imm(4),
      false,
    ),
  ));
  /*  BL printf             //branch to printf */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("printf"))));
  /*  MOV r0, #0            //move 0 to r0 */
  code.text.push(Instr(
    AL,
    Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Imm(0), false),
  ));
  /*  BL fflush             //branch to fflush */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("fflush"))));
  /*  POP {pc}              //pop the pc register */
  code.text.push(Instr(AL, Pop));
}

fn print_string(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create the msg label to display string data and add to the GeneratedCode
  data member: */

  /* msg_string: */
  code.data.push(Directive(Label(String::from("msg_string"))));
  /* .word 5                   //allocate space for a word of size 5 */
  code.data.push(Directive(Word(5)));
  /* .ascii "%.*s\0"           //convert into ascii */
  code.data.push(Directive(Ascii(String::from("%.*s\\0"))));

  /* Generate the p_print_string label to print strings, predefined and the same
  for every program. */

  /*p_print_string: */
  code
    .text
    .push(Directive(Label(String::from("p_print_string"))));
  /*  PUSH {lr}             //push link reg */
  code.text.push(Instr(AL, Push));
  /*  LDR r1, [r0]          //load address at r0 into r1 */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(1),
      LoadArg::MemAddress(MemAddress {
        reg: Reg::RegNum(0),
        offset: None,
      }),
    ),
  ));
  /*  ADD r2, r0, #4        //add 4 to r0 and store in r2 */
  code.text.push(Instr(
    AL,
    Binary(
      BinaryInstr::Add,
      Reg::RegNum(2),
      Reg::RegNum(0),
      Op2::Imm(4),
      false,
    ),
  ));
  /*  LDR r0, =msg_string   //load the result of msg_string */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::Label(String::from("msg_string")),
    ),
  ));
  /*  ADD r0, r0, #4        //add 4 to r0 and store in r0 */
  code.text.push(Instr(
    AL,
    Binary(
      BinaryInstr::Add,
      Reg::RegNum(0),
      Reg::RegNum(0),
      Op2::Imm(4),
      false,
    ),
  ));
  /*  BL printf             //branch to printf */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("printf"))));
  /*  MOV r0, #0            //move 0 to r0 */
  code.text.push(Instr(
    AL,
    Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Imm(0), false),
  ));
  /*  BL fflush             //branch to fflush */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("fflush"))));
  /*  POP {pc}              //pop the pc register */
  code.text.push(Instr(AL, Pop));
}

#[derive(PartialEq)]
pub enum PrintFmt {
  Int,
  Ref,
}

impl Display for PrintFmt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PrintFmt::Int => write!(f, "int"),
      PrintFmt::Ref => write!(f, "reference"),
    }
  }
}

fn print_int_or_ref(code: &mut GeneratedCode, opt: PrintFmt) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Get symbol depending on which opt is specified */

  let symbol = if opt == PrintFmt::Ref { 'p' } else { 'd' };

  /* Create the msg label to display string data and add to the GeneratedCode
  data member: */

  /* msg_opt: */
  code.data.push(Directive(Label(format!("msg_{}", opt))));
  /* .word 3                   //allocate space for a word of size 3 */
  code.data.push(Directive(Word(3)));
  /* .ascii "%symbol\0"           //convert into ascii */
  code.data.push(Directive(Ascii(format!("%{}\\0", symbol))));

  /* Generate the p_print_opt label to print strings, predefined and the same
  for every program. */

  /*p_print_opt: */
  code.text.push(Directive(Label(format!("p_print_{}", opt))));
  /*  PUSH {lr}             //push link reg */
  code.text.push(Instr(AL, Push));
  /*  MOV r1, r0            //move r0 to r1 */
  code.text.push(Instr(
    AL,
    Unary(
      UnaryInstr::Mov,
      Reg::RegNum(1),
      Op2::Reg(Reg::RegNum(0), 0),
      false,
    ),
  ));

  /*  LDR r0, =msg_int      //load result of msg_int into r0 */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::Label(format!("msg_{}", opt)),
    ),
  ));
  /*  ADD r0, r0, #4        //add the 4 to r0, and store the result in r0 */
  code.text.push(Instr(
    AL,
    Binary(
      BinaryInstr::Add,
      Reg::RegNum(0),
      Reg::RegNum(0),
      Op2::Imm(4),
      false,
    ),
  ));
  /*  BL printf             //branch to printf */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("printf"))));
  /*  MOV r0, #0            //move 0 to r0 */
  code.text.push(Instr(
    AL,
    Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Imm(0), false),
  ));
  /*  BL fflush             //branch to fflush */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("fflush"))));
  /*  POP {pc}              //pop the pc register */
  code.text.push(Instr(AL, Pop));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn exit_statement() {
    let expr = Expr::IntLiter(0);
    let stat = Stat::Exit(expr.clone());
    let mut min_regs = 4;

    /* Actual output. */
    let mut actual_code = GeneratedCode {
      data: vec![],
      text: vec![],
      print_branches: PrintBranches {
        ints: false,
        strings: false,
        bools: false,
        refs: false,
      },
    };
    stat.generate(&mut actual_code, &mut min_regs);

    /* Expected output. */
    let mut expected_code = GeneratedCode {
      data: vec![],
      text: vec![],
      print_branches: todo!(),
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