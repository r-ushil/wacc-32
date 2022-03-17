use super::*;
use std::fmt::Display;

pub const PREDEF_SYS_EXIT: &str = "exit";
pub const PREDEF_SYS_FFLUSH: &str = "fflush";
pub const PREDEF_SYS_FREE: &str = "free";
pub const PREDEF_SYS_MALLOC: &str = "malloc";
pub const PREDEF_SYS_PRINTF: &str = "printf";
pub const PREDEF_SYS_PUTCHAR: &str = "putchar";
pub const PREDEF_SYS_SCANF: &str = "scanf";

pub const PREDEF_AEABI_IDIV: &str = "__aeabi_idiv";
pub const PREDEF_AEABI_IDIVMOD: &str = "__aeabi_idivmod";

pub const PREDEF_PRINT_INT: &str = "p_print_int";
pub const PREDEF_PRINT_STRING: &str = "p_print_string";
pub const PREDEF_PRINT_BOOL: &str = "p_print_bool";
pub const PREDEF_PRINT_REFS: &str = "p_print_reference";

pub const PREDEF_PRINTLN: &str = "p_print_ln";
pub const PREDEF_FREE_PAIR: &str = "p_free_pair";
pub const PREDEF_FREE: &str = "p_free";

pub const PREDEF_THROW_RUNTIME_ERR: &str = "p_throw_runtime_error";
pub const PREDEF_THROW_OVERFLOW_ERR: &str = "p_throw_overflow_error";
pub const PREDEF_CHECK_NULL_POINTER: &str = "p_check_null_pointer";
pub const PREDEF_CHECK_DIVIDE_BY_ZERO: &str = "p_check_divide_by_zero";

pub const PREDEF_CHECK_ARRAY_BOUNDS: &str = "p_check_array_bounds";

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RequiredPredefs {
  PrintInt,
  PrintString,
  PrintBool,
  PrintRefs,
  PrintLn,
  ReadChar,
  ReadInt,
  FreePair,
  FreeArray,
  FreeCustom,
  RuntimeError,
  OverflowError,
  DivideByZeroError,
  ArrayBoundsError,
  CheckNullPointer,
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
  type Input = ();
  type Output = ();
  fn generate(
    &self,
    _scope: &ScopeReader,
    code: &mut GeneratedCode,
    _regs: &[GenReg],
    _aux: (),
  ) {
    match *self {
      RequiredPredefs::PrintInt => print_int_or_ref(code, PrintFmt::Int),
      RequiredPredefs::PrintString => print_string(code),
      RequiredPredefs::PrintBool => print_bool(code),
      RequiredPredefs::PrintRefs => print_int_or_ref(code, PrintFmt::Ref),
      RequiredPredefs::PrintLn => println(code),
      RequiredPredefs::ReadChar => read(code, ReadFmt::Char),
      RequiredPredefs::ReadInt => read(code, ReadFmt::Int),
      RequiredPredefs::FreePair => free_pair(code),
      RequiredPredefs::FreeArray | RequiredPredefs::FreeCustom => free(code),
      RequiredPredefs::RuntimeError => throw_runtime_error(code),
      RequiredPredefs::OverflowError => throw_overflow_error(code),
      RequiredPredefs::DivideByZeroError => check_divide_by_zero(code),
      RequiredPredefs::ArrayBoundsError => check_array_bounds(code),
      RequiredPredefs::CheckNullPointer => check_null_pointer(code),
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

fn check_array_bounds(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* msg_0:                         //generate new msg label */
  let msg_0 = code.get_msg("ArrayIndexOutOfBoundsError: negative index\n\0");
  /* msg_1:                         //generate new msg label */
  let msg_1 = code.get_msg("ArrayIndexOutOfBoundsError: index too large\n\0");

  /* p_check_array_bounds: */
  code
    .text
    .push(Directive(Label(PREDEF_CHECK_ARRAY_BOUNDS.to_string())));

  /* PUSH {lr}                      //push link register */
  code.text.push(Asm::push(Reg::Link));
  /* CMP r0, #0                     //compare r0 to 0 */
  code.text.push(Asm::cmp(ArgReg::R0, Op2::Imm(0)));
  /* LDRLT r0, =msg_0               //load msg_0 if less than flag set into r0 */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_0)).lt());
  /* BLLT p_throw_runtime_error     //branch to runtime error as a result */
  RequiredPredefs::RuntimeError.mark(code);
  code.text.push(Instr(
    LT,
    Branch(true, PREDEF_THROW_RUNTIME_ERR.to_string()),
  ));
  /* LDR r1, [r1]                   //dereference r1 */
  code.text.push(Asm::ldr(
    ArgReg::R1,
    LoadArg::MemAddress(ArgReg::R1.into(), 0),
  ));
  /* CMP r0, r1                     //compare r0 and r1 */
  code.text.push(Asm::cmp(ArgReg::R0, ArgReg::R1));
  /* LDRCS r0, =msg_1               //load msg_1 into r0 if carry flag is set */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_1)).cs());
  /* BLCS p_throw_runtime_error     //branch to runtime error as a result */
  code.text.push(Instr(
    CS,
    Branch(true, PREDEF_THROW_RUNTIME_ERR.to_string()),
  ));
  /* POP {pc}                       //pop PC register */
  code.text.push(Asm::pop(Reg::PC));
}

fn read(code: &mut GeneratedCode, fmt: ReadFmt) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label for reading an integer or character */

  let msg = code.get_msg(if fmt == ReadFmt::Int { "%d\0" } else { " %c\0" });

  /* Generate a p_read_{fmt} label to branch to when reading an int or a char */

  /* p_read_{fmt}: */
  code.text.push(Directive(Label(format!("p_read_{}", fmt))));
  /*  PUSH {lr}            //push link reg */
  code.text.push(Asm::push(Reg::Link));
  /*  MOV r1, r0            //move r0 to r1 */
  code.text.push(Asm::mov(Reg::Arg(ArgReg::R1), ArgReg::R0));

  /*  LDR r0, =msg_read_{fmt}   //load the result of msg_read_{fmt} */
  code.text.push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg)));

  /*  ADD r0, r0, #4        //add 4 to r0 and store in r0 */
  code
    .text
    .push(Asm::add(ArgReg::R0, ArgReg::R0, Op2::Imm(ARM_DSIZE_WORD)));
  /*  BL scanf             //branch to scanf */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_SCANF.to_string())));

  /*  POP {pc}              //pop the pc register */
  code.text.push(Asm::pop(Reg::PC));
}

fn println(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label for the termination of a line */
  /* msg_println: */
  let msg_label = code.get_msg("\0");

  /* Generate a p_print_ln label to branch to when printing a line */

  /* p_print_ln: */
  code.text.push(Directive(Label(PREDEF_PRINTLN.to_string())));
  /*  PUSH {lr}            //push link reg */
  code.text.push(Asm::push(Reg::Link));
  /*  LDR r0, =msg_println   //load the result of msg_println */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label)));
  /*  ADD r0, r0, #4        //add 4 to r0 and store in r0 */
  code
    .text
    .push(Asm::add(ArgReg::R0, ArgReg::R0, Op2::Imm(ARM_DSIZE_WORD)));
  /*  BL puts             //branch to puts */
  code
    .text
    .push(Instr(AL, Branch(true, String::from("puts"))));
  /*  MOV r0, #0            //move 0 to r0 */
  code.text.push(Asm::mov(ArgReg::R0, Op2::Imm(0)));
  /*  BL fflush             //branch to fflush */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_FFLUSH.to_string())));
  /*  POP {pc}              //pop the pc register */
  code.text.push(Asm::pop(Reg::PC));
}

fn check_null_pointer(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label to display when derefencing a null pointer. */
  let msg_label =
    code.get_msg("NullReferenceError: dereference a null reference\n\0");

  /* Generate label to throw a runtime error for whatever's in registers */
  /* p_check_null_pointer: */
  code
    .text
    .push(Directive(Label(String::from(PREDEF_CHECK_NULL_POINTER))));

  /*  PUSH {lr}            //push link reg */
  code.text.push(Asm::push(Reg::Link));
  /*  CMP r0, #0           //compare the contents of r0 to 0 and set flags */
  code.text.push(Asm::cmp(ArgReg::R0, Op2::Imm(0)));
  /*  LDREQ r0, =msg_label   //load error msg if r0 equals 0 */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label)).eq());

  /*  BLEQ p_throw_runtime_error   //branch to runtime error if r0 equals 0 */
  code.text.push(Instr(
    EQ,
    Branch(true, PREDEF_THROW_RUNTIME_ERR.to_string()),
  ));
  //set runtime error generation to true
  RequiredPredefs::RuntimeError.mark(code);

  /*  POP {pc}            //pop pc register */
  code.text.push(Asm::pop(Reg::PC));
}

fn check_divide_by_zero(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label to display when divide by zero occurs. */
  /* msg_divide_by_zero: */
  let msg_label =
    code.get_msg("DivideByZeroError: divide or modulo by zero\n\0");

  /* Generate label to throw a runtime error for whatever's in registers */
  /* p_check_divide_by_zero: */
  code
    .text
    .push(Directive(Label(PREDEF_CHECK_DIVIDE_BY_ZERO.to_string())));

  /*  PUSH {lr}            //push link reg */
  code.text.push(Asm::push(Reg::Link));
  /*  CMP r1, #0           //compare the contents of r1 to 0 and set flags */
  code.text.push(Asm::cmp(Reg::Arg(ArgReg::R1), Op2::Imm(0)));
  /*  LDREQ r0, =msg_divide_by_zero   //load error msg if r0 equals 0 */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label)).eq());

  /*  BLEQ p_throw_runtime_error   //branch to runtime error if r0 equals 0 */
  code.text.push(Instr(
    EQ,
    Branch(true, PREDEF_THROW_RUNTIME_ERR.to_string()),
  ));
  //set runtime error generation to true
  RequiredPredefs::RuntimeError.mark(code);

  /*  POP {pc}            //pop pc register */
  code.text.push(Asm::pop(Reg::PC));
}

fn throw_overflow_error(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label to display when integer overflow occurs. */
  /* msg_overflow_error: */
  let msg_label = code.get_msg(
    "OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0",
  );

  /* Generate label to throw a runtime error for whatever's in registers */
  /* p_throw_overflow_error: */
  code
    .text
    .push(Directive(Label(PREDEF_THROW_OVERFLOW_ERR.to_string())));

  /* LDR r0, =msg_overflow_error     //load result of message overflow error into r0 */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label)));
  /* BL p_throw_runtime_error        //branch to runtime error */
  RequiredPredefs::RuntimeError.mark(code);
  code.text.push(Instr(
    AL,
    Branch(true, PREDEF_THROW_RUNTIME_ERR.to_string()),
  ));
}

fn free(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  let msg_label =
    code.get_msg("NullReferenceError: dereference a null reference\n\0");

  /* p_free_pair: */
  code.text.push(Directive(Label(PREDEF_FREE.to_string())));
  /*  PUSH {lr}            //push link reg */
  code.text.push(Asm::push(Reg::Link));
  /*  CMP r0, #0           //compare the contents of r0 to 0 and set flags */
  code.text.push(Asm::cmp(ArgReg::R0, Op2::Imm(0)));
  /*  LDREQ r0, =msg_null_deref   //load deref msg if r0 equals 0 */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label)).eq());
  /*  BEQ p_throw_runtime_error   //branch to runtime error if r0 equals 0 */
  code.text.push(Instr(
    EQ,
    Branch(false, PREDEF_THROW_RUNTIME_ERR.to_string()),
  ));

  //set runtime error generation to true
  RequiredPredefs::RuntimeError.mark(code);

  /* BL free                      //branch to free */
  code
    .text
    .push(Instr(EQ, Branch(false, PREDEF_SYS_FREE.to_string())));

  /*  POP {pc}            //pop pc register */
  code.text.push(Asm::pop(Reg::PC));
}

fn free_pair(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create a msg label to display in an attempt to free a null pair */
  /* msg_null_deref: */
  let msg_label =
    code.get_msg("NullReferenceError: dereference a null reference\n\0");

  /* Generate the p_free_pair label to free the pair in r0, predefined */

  /* p_free_pair: */
  code
    .text
    .push(Directive(Label(PREDEF_FREE_PAIR.to_string())));
  /*  PUSH {lr}            //push link reg */
  code.text.push(Asm::push(Reg::Link));
  /*  CMP r0, #0           //compare the contents of r0 to 0 and set flags */
  code.text.push(Asm::cmp(ArgReg::R0, Op2::Imm(0)));
  /*  LDREQ r0, =msg_null_deref   //load deref msg if r0 equals 0 */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label)).eq());
  /*  BEQ p_throw_runtime_error   //branch to runtime error if r0 equals 0 */
  code.text.push(Instr(
    EQ,
    Branch(false, PREDEF_THROW_RUNTIME_ERR.to_string()),
  ));

  //set runtime error generation to true
  RequiredPredefs::RuntimeError.mark(code);

  /*  PUSH {r0}           //push r0 */
  code.text.push(Asm::push(ArgReg::R0));

  code.text.push(Asm::ldr(
    ArgReg::R0,
    LoadArg::MemAddress(ArgReg::R0.into(), 0),
  ));

  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_FREE.to_string())));

  /*  LDR r0, [sp]        //load stack pointer address into r0 */
  code.text.push(Asm::ldr(ArgReg::R0, Reg::StackPointer));
  /*  LDR r0, [r0, #4]    //load address of r0+4 into r0 */
  code.text.push(Asm::ldr(
    ArgReg::R0,
    LoadArg::MemAddress(ArgReg::R0.into(), 4),
  ));
  /*  BL free             //branch to free */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_FREE.to_string())));
  /*  POP {r0}            //pop r0 register */
  code.text.push(Asm::pop(ArgReg::R0));
  /*  BL free             //branch to free */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_FREE.to_string())));
  /*  POP {pc}            //pop pc register */
  code.text.push(Asm::pop(Reg::PC));
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
    .push(Directive(Label(PREDEF_THROW_RUNTIME_ERR.to_string())));
  /* BL p_print_string        //branch to print a string */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_PRINT_STRING.to_string())));
  /* MOV r0, #-1              //move -1 into r0*/
  RequiredPredefs::PrintString.mark(code);
  code.text.push(Asm::mov(ArgReg::R0, Op2::Imm(-1)));
  /* BL exit                  //exit with status code -1  */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_EXIT.to_string())));
}

fn print_bool(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create the msg labels to display string data for TRUE and FALSE, and add to the
  GeneratedCode data member: */

  /* msg_true: */
  let msg_label_true = code.get_msg("true\0");

  /* msg_false: */
  let msg_label_false = code.get_msg("false\0");

  /* Generate the p_print_bool label to print bool, predefined and the same
  for every program. */

  /*p_print_bool: */
  code
    .text
    .push(Directive(Label(PREDEF_PRINT_BOOL.to_string())));
  /*  PUSH {lr}             //push link reg */
  code.text.push(Asm::push(Reg::Link));
  /*  CMP r0, #0            //compare the contents of r0 to 0 and set flags */
  code.text.push(Asm::cmp(ArgReg::R0, Op2::Imm(0)));
  /*  LDRNE r0, =msg_true   //load result of msg_true if not equal to r0  */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label_true)).ne());
  /*  LDREQ r0, =msg_false   //load result of msg_false if equal to r0  */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label_false)).eq());
  /*  ADD r0, r0, #4        //add 4 to r0 and store in r0 */
  code
    .text
    .push(Asm::add(ArgReg::R0, ArgReg::R0, Op2::Imm(ARM_DSIZE_WORD)));
  /*  BL printf             //branch to printf */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_PRINTF.to_string())));
  /*  MOV r0, #0            //move 0 to r0 */
  code.text.push(Asm::mov(ArgReg::R0, Op2::Imm(0)));
  /*  BL fflush             //branch to fflush */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_FFLUSH.to_string())));
  /*  POP {pc}              //pop the pc register */
  code.text.push(Asm::pop(Reg::PC));
}

fn print_string(code: &mut GeneratedCode) {
  use self::CondCode::*;
  use self::Directive::*;
  use self::Instr::*;
  use Asm::*;

  /* Create the msg label to display string data and add to the GeneratedCode
  data member: */
  /* msg_string: */
  let msg_label = code.get_msg("%.*s\0");

  /* Generate the p_print_string label to print strings, predefined and the same
  for every program. */

  /*p_print_string: */
  code
    .text
    .push(Directive(Label(PREDEF_PRINT_STRING.to_string())));
  /*  PUSH {lr}             //push link reg */
  code.text.push(Asm::push(Reg::Link));
  /*  LDR r1, [r0]          //load address at r0 into r1 */
  code.text.push(Asm::ldr(
    Reg::Arg(ArgReg::R1),
    LoadArg::MemAddress(ArgReg::R0.into(), 0),
  ));
  /*  ADD r2, r0, #4        //add 4 to r0 and store in r2 */
  code.text.push(Asm::add(
    Reg::Arg(ArgReg::R2),
    ArgReg::R0,
    Op2::Imm(ARM_DSIZE_WORD),
  ));
  /*  LDR r0, =msg_string   //load the result of msg_string */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label)));
  /*  ADD r0, r0, #4        //add 4 to r0 and store in r0 */
  code
    .text
    .push(Asm::add(ArgReg::R0, ArgReg::R0, Op2::Imm(ARM_DSIZE_WORD)));
  /*  BL printf             //branch to printf */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_PRINTF.to_string())));
  /*  MOV r0, #0            //move 0 to r0 */
  code.text.push(Asm::mov(ArgReg::R0, Op2::Imm(0)));
  /*  BL fflush             //branch to fflush */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_FFLUSH.to_string())));
  /*  POP {pc}              //pop the pc register */
  code.text.push(Asm::pop(Reg::PC));
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
  // /* msg_opt: */
  let msg_content = format!("%{}\0", symbol);
  let msg_label = code.get_msg(msg_content.as_str());

  /* Generate the p_print_opt label to print strings, predefined and the same
  for every program. */

  /*p_print_opt: */
  let print_label = match opt {
    PrintFmt::Int => PREDEF_PRINT_INT,
    PrintFmt::Ref => PREDEF_PRINT_REFS,
  };

  code.text.push(Directive(Label(print_label.to_string())));
  /*  PUSH {lr}             //push link reg */
  code.text.push(Asm::push(Reg::Link));
  /*  MOV r1, r0            //move r0 to r1 */
  code.text.push(Asm::mov(Reg::Arg(ArgReg::R1), ArgReg::R0));

  /*  LDR r0, =msg_int      //load result of msg_int into r0 */
  code
    .text
    .push(Asm::ldr(ArgReg::R0, LoadArg::Label(msg_label)));
  /*  ADD r0, r0, #4        //add the 4 to r0, and store the result in r0 */
  code
    .text
    .push(Asm::add(ArgReg::R0, ArgReg::R0, Op2::Imm(ARM_DSIZE_WORD)));
  /*  BL printf             //branch to printf */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_PRINTF.to_string())));
  /*  MOV r0, #0            //move 0 to r0 */
  code.text.push(Asm::mov(ArgReg::R0, Op2::Imm(0)));
  /*  BL fflush             //branch to fflush */
  code
    .text
    .push(Instr(AL, Branch(true, PREDEF_SYS_FFLUSH.to_string())));
  /*  POP {pc}              //pop the pc register */
  code.text.push(Asm::pop(Reg::PC));
}
