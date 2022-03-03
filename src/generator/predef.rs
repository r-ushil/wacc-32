use super::*;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct GeneratePredefs {
  pub print_ints: bool,
  pub print_strings: bool,
  pub print_bools: bool,
  pub print_refs: bool,
  pub println: bool,
  pub read_char: bool,
  pub read_int: bool,
  pub free_pair: bool,
  pub runtime_err: bool,
  pub overflow_err: bool,
  pub div_by_zero: bool,
}

#[derive(PartialEq, Debug)]
pub enum RequiredPredefs {
  PrintInt,
  PrintString,
  PrintBool,
  PrintChar, // TODO: Implement
  PrintRefs,
  PrintLn,
  ReadChar,
  ReadInt,
  FreePair,
  FreeArray, // TODO: Implement
  RuntimeError,
  OverflowError,
  DivideByZeroError,
}

/* Pushes a pre-defined function to the vector on GeneratedCode if it doesn't
already require this predef. */
impl RequiredPredefs {
  pub fn mark(self, code: &mut GeneratedCode) {
    if !code.required_predefs.contains(&self) {
      code.required_predefs.push(self);
    }
  }
}

impl Generatable for RequiredPredefs {
  fn generate(&self, _scope: &Scope, code: &mut GeneratedCode, regs: &[Reg]) {
    match *self {
      RequiredPredefs::PrintInt => print_int_or_ref(code, PrintFmt::Int),
      RequiredPredefs::PrintString => print_string(code),
      RequiredPredefs::PrintBool => print_bool(code),
      RequiredPredefs::PrintChar => todo!(), // TODO: Implement
      RequiredPredefs::PrintRefs => print_int_or_ref(code, PrintFmt::Ref),
      RequiredPredefs::PrintLn => println(code),
      RequiredPredefs::ReadChar => read(code, ReadFmt::Char),
      RequiredPredefs::ReadInt => read(code, ReadFmt::Int),
      RequiredPredefs::FreePair => free_pair(code),
      RequiredPredefs::FreeArray => todo!(), // TODO: Implement
      RequiredPredefs::RuntimeError => throw_runtime_error(code),
      RequiredPredefs::OverflowError => throw_overflow_error(code),
      RequiredPredefs::DivideByZeroError => check_divide_by_zero(code),
    }
  }
}

impl Default for GeneratePredefs {
  fn default() -> Self {
    Self {
      print_ints: false,
      print_strings: false,
      print_bools: false,
      print_refs: false,
      println: false,
      read_char: false,
      read_int: false,
      free_pair: false,
      runtime_err: false,
      overflow_err: false,
      div_by_zero: false,
    }
  }
}

#[derive(PartialEq)]
pub enum ReadFmt {
  Char,
  Int,
}

impl Display for ReadFmt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ReadFmt::Char => write!(f, "char"),
      ReadFmt::Int => write!(f, "int"),
    }
  }
}

fn read(code: &mut GeneratedCode, fmt: ReadFmt) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label for reading an integer or character */

  if fmt == ReadFmt::Int {
    /* msg_read_int: */
    code.data.push(Directive(Label("msg_read_int".to_string())));
    /* .word 3                   //allocate space for a word of size 3 FOR INT */
    code.data.push(Directive(Word(3)));
    /* .ascii "%d\0"         //convert into ascii */
    code.data.push(Directive(Ascii(String::from("%d\\0"))));
  } else if fmt == ReadFmt::Char {
    /* msg_read_char: */
    code
      .data
      .push(Directive(Label("msg_read_char".to_string())));
    /* .word 4                   //allocate space for a word of size 4 FOR CHAR */
    code.data.push(Directive(Word(4)));
    /* .ascii "%c\0"         //convert into ascii */
    code.data.push(Directive(Ascii(String::from("%c\\0"))));
  }

  /* Generate a p_read_{fmt} label to branch to when reading an int or a char */

  /* p_read_{fmt}: */
  code.data.push(Directive(Label(format!("p_read_{}", fmt))));
  /*  PUSH {lr}            //push link reg */
  code.text.push(Instr(AL, Push(Reg::Link)));
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

  /*  LDR r0, =msg_read_{fmt}   //load the result of msg_read_{fmt} */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::Label(format!("msg_read_{}", fmt)),
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
  /*  BL scanf             //branch to scanf */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("scanf"))));

  /*  POP {pc}              //pop the pc register */
  code.text.push(Instr(AL, Pop(Reg::PC)));
}

fn println(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label for the termination of a line */

  /* msg_println: */
  code.data.push(Directive(Label("msg_println".to_string())));

  /* .word 1                   //allocate space for a word of size 1 */
  code.data.push(Directive(Word(1)));
  /* .ascii "\0"         //convert into ascii */
  code.data.push(Directive(Ascii(String::from("\\0"))));

  /* Generate a p_print_ln label to branch to when printing a line */

  /* p_print_ln: */
  code.data.push(Directive(Label("p_print_ln".to_string())));
  /*  PUSH {lr}            //push link reg */
  code.text.push(Instr(AL, Push(Reg::Link)));
  /*  LDR r0, =msg_println   //load the result of msg_println */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::Label(String::from("msg_println")),
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
  /*  BL puts             //branch to puts */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("puts"))));
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
  code.text.push(Instr(AL, Pop(Reg::PC)));
}

fn check_divide_by_zero(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label to display when divide by zero occurs. */

  /* msg_divide_by_zero: */
  code
    .data
    .push(Directive(Label("msg_divide_by_zero".to_string())));
  /* .word 45                   //allocate space for a word of size 45 */
  code.data.push(Directive(Word(45)));
  /* .ascii "DivideByZeroError: ...\n\0"         //convert into ascii */
  code.data.push(Directive(Ascii(String::from(
    "DivideByZeroError: divide or modulo by zero\\n\\0",
  ))));

  /* Generate label to throw a runtime error for whatever's in registers */
  /* p_check_divide_by_zero: */
  code
    .text
    .push(Directive(Label(String::from("p_check_divide_by_zero"))));

  /*  PUSH {lr}            //push link reg */
  code.text.push(Instr(AL, Push(Reg::Link)));
  /*  CMP r1, #0           //compare the contents of r1 to 0 and set flags */
  code.text.push(Instr(
    AL,
    Unary(UnaryInstr::Cmp, Reg::RegNum(1), Op2::Imm(0), false),
  ));
  /*  LDREQ r0, =msg_divide_by_zero   //load error msg if r0 equals 0 */
  code.text.push(Instr(
    EQ,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::Label(String::from("msg_divide_by_zero")),
    ),
  ));

  /*  BLEQ p_throw_runtime_error   //branch to runtime error if r0 equals 0 */
  code.text.push(Instr(
    EQ,
    Branch(true, String::from("p_throw_runtime_error")),
  ));
  //set runtime error generation to true
  RequiredPredefs::RuntimeError.mark(code);

  /*  POP {pc}            //pop pc register */
  code.text.push(Instr(AL, Pop(Reg::PC)));
}

fn throw_overflow_error(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label to display when integer overflow occurs. */

  /* msg_overflow_error: */
  code
    .data
    .push(Directive(Label("msg_overflow_error".to_string())));
  /* .word 83                   //allocate space for a word of size 83 */
  code.data.push(Directive(Word(83)));
  /* .ascii "OverflowError: ...\n\0"         //convert into ascii */
  code.data.push(Directive(Ascii(String::from(
    "OverflowError: the result is too small/large to store in a 4-byte signed-integer.\\n\\0",
  ))));

  /* Generate label to throw a runtime error for whatever's in registers */
  /* p_throw_overflow_error: */
  code
    .text
    .push(Directive(Label(String::from("p_throw_overflow_error"))));

  /* LDR r0, =msg_overflow_error     //load result of message overflow error into r0 */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::Label(String::from("msg_overflow_error")),
    ),
  ));
  /* BL p_throw_runtime_error        //branch to runtime error */
  RequiredPredefs::RuntimeError.mark(code);
  code.text.push(Instr(
    AL,
    Branch(true, String::from("p_throw_runtime_error")),
  ));
}

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
  code.text.push(Instr(AL, Push(Reg::Link)));
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
    Branch(false, String::from("p_throw_runtime_error")),
  ));

  //set runtime error generation to true
  /*  PUSH {r0}           //push r0 */
  RequiredPredefs::RuntimeError.mark(code);

  code.text.push(Instr(AL, Push(Reg::RegNum(0))));
  /*  LDR r0, [sp]        //load stack pointer address into r0 */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::MemAddress(Reg::StackPointer, 0),
    ),
  ));
  /*  LDR r0, [r0, #4]    //load address of r0+4 into r0 */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(0),
      LoadArg::MemAddress(Reg::RegNum(0), 4),
    ),
  ));
  /*  BL free             //branch to free */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("free"))));
  /*  POP {r0}            //pop r0 register */
  code.text.push(Instr(AL, Pop(Reg::RegNum(0))));
  /*  BL free             //branch to free */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("free"))));
  /*  POP {pc}            //pop pc register */
  code.text.push(Instr(AL, Pop(Reg::PC)));
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
  RequiredPredefs::PrintString.mark(code);
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
  code.text.push(Instr(AL, Push(Reg::Link)));
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
  code.text.push(Instr(AL, Pop(Reg::PC)));
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
  code.text.push(Instr(AL, Push(Reg::Link)));
  /*  LDR r1, [r0]          //load address at r0 into r1 */
  code.text.push(Instr(
    AL,
    Load(
      DataSize::Word,
      Reg::RegNum(1),
      LoadArg::MemAddress(Reg::RegNum(0), 0),
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
  code.text.push(Instr(AL, Pop(Reg::PC)));
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
  code.text.push(Instr(AL, Push(Reg::Link)));
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
  code.text.push(Instr(AL, Pop(Reg::PC)));
}
