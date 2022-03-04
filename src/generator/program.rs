use std::collections::HashMap;

use super::*;

// #[derive(PartialEq, Debug, Clone)]
impl Generatable for Program {
  type Input = ();
  type Output = ();

  fn generate(&self, _: &Scope, code: &mut GeneratedCode, regs: &[Reg], aux: ()) {
    /* No registers should be in use by this point. */
    assert!(regs == GENERAL_REGS);

    /* Move into program's scope. */
    let scope = &Scope::new(&self.symbol_table);

    /* Generate code for every function, side affecting the code struct.
     * Each function is allowed to use the registers from min_regs variable
     * and up. */
    for function in &self.funcs {
      function.generate(scope, code, regs, ());
    }
    /* The statement of the program should be compiled as if it is in a
     * function called main, which takes nothing and returns an int exit code */
    Func {
      ident: String::from("main"),
      signature: FuncSig {
        params: Vec::new(),
        return_type: Type::Int,
      },
      body: *self.statement.1.clone(),
      params_st: SymbolTable::default(),
      body_st: self.statement.0.clone(),
    }
    .generate(scope, code, regs, ());

    /* Write all pre-defined functions that we require to the end of the
    GeneratedCode */
    let mut i = 0;
    while i < code.required_predefs.len() {
      let required_predef = code.required_predefs[i];
      required_predef.generate(scope, code, regs, ());
      i += 1;
    }
  }
}

const MAX_OP2_VALUE: i32 = 1024;

impl Generatable for Func {
  type Input = ();
  type Output = ();

  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg], aux: ()) {
    /* No registers should be in use by this point. */
    assert!(regs == GENERAL_REGS);

    // TODO: make this a more robust check
    let main = self.ident == "main";

    /* Comments reflect the following example:
    int foo(int x) is
      int y = 5;
      return x
    end */

    /* Function label.
    foo: */
    code.text.push(Asm::Directive(Directive::Label(if main {
      self.ident.to_string()
    } else {
      generate_function_name(self.ident.to_string())
    })));

    /* Save link register.
    PUSH {lr} */
    code.text.push(Asm::always(Instr::Push(Reg::Link)));

    /* Allocate space on stack for local vars. */
    let mut body_st_size = self.body_st.size;

    while body_st_size > 0 {
      code.text.push(Asm::always(Instr::Binary(
        BinaryInstr::Sub,
        Reg::StackPointer,
        Reg::StackPointer,
        Op2::Imm(MAX_OP2_VALUE.min(body_st_size)),
        false,
      )));

      body_st_size -= MAX_OP2_VALUE;
    }

    /* Move into parameter scope. */
    let scope = &scope.new_scope(&self.params_st);

    /* Make new 4 byte scope to reserve space for link register. */
    let mut lr_table = SymbolTable::default();
    lr_table.size = 4;
    let scope = &scope.new_scope(&lr_table);

    /* Move into function body scope. */
    let scope = &scope.new_scope(&self.body_st);

    /* Generate body.
    SUB sp, sp, #4
    LDR r4, =5
    STR r4, [sp]
    LDR r4, [sp, #8]
    MOV r0, r4
    ADD sp, sp, #4 */
    self.body.generate(scope, code, regs, ());

    /* Main function implicitly ends in return 0. */
    if main {
      /* Deallocate stack for main function. */
      let mut body_st_size = scope.get_total_offset();

      while body_st_size > 0 {
        code.text.push(Asm::always(Instr::Binary(
          BinaryInstr::Add,
          Reg::StackPointer,
          Reg::StackPointer,
          Op2::Imm(MAX_OP2_VALUE.min(body_st_size)),
          false,
        )));

        body_st_size -= MAX_OP2_VALUE;
      }

      code.text.push(Asm::always(Instr::Load(
        DataSize::Word,
        Reg::RegNum(0),
        LoadArg::Imm(0),
      )))
    }

    /* Jump back to caller.
    POP {pc} */
    code.text.push(Asm::always(Instr::Pop(Reg::PC)));

    /* Mark block for compilations.
    .ltorg */
    code.text.push(Asm::Directive(Directive::Assemble));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn skip_func() {
    /*
    int foo(int x) is
      skip
    end */
  }

  //   #[test]
  //   fn basic_func() {
  //     /*
  //     int foo(int x) is
  //       int y = 5;
  //       return x
  //     end */

  //     let body = Stat::sequence(
  //       Stat::declaration(Type::Int, "y", 5),
  //       Stat::return_(Expr::ident("x")),
  //     );

  //     let func = Func {
  //       // int foo(int x)
  //       ident: String::from("foo"),
  //       signature: FuncSig {
  //         params: vec![(Type::Int, String::from("x"))],
  //         return_type: Type::Int,
  //       },
  //       // is int y = 5; return x end
  //       body,
  //       symbol_table: SymbolTable::default(),
  //     };

  //     let st = SymbolTable::default();
  //     let scope = Scope::new(&st);

  //     let mut actual_code = GeneratedCode::default();
  //     func.generate(&scope, &mut actual_code, &mut 4);
  //     assert_eq!(
  //       format!("{}", actual_code),
  //       format!(
  //         ".data
  // .text
  // .global main
  // f_foo:
  //   PUSH {{lr}}{}POP {{pc}}
  //   POP {{pc}}
  //   .ltorg
  // main:
  //   PUSH {{lr}}
  //   LDR r0, =0
  //   POP {{pc}}
  //   .ltorg
  //     ",
  //         body.generate
  //       )
  //     );

  //     /*
  //     f_foo:
  //       PUSH {lr}
  //       SUB sp, sp, #4
  //       LDR r4, =5
  //       STR r4, [sp]
  //       LDR r4, [sp, #8]
  //       MOV r0, r4
  //       ADD sp, sp, #4
  //       POP {pc}
  //       POP {pc}
  //       .ltorg */
  //   }
}
