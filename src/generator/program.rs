use std::collections::HashMap;

use super::*;

// #[derive(PartialEq, Debug, Clone)]
impl Generatable for Program {
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg]) {
    /* No registers should be in use by this point. */
    assert!(regs == GENERAL_REGS);

    /* Move into program's scope. */
    let scope = &scope.new_scope(&self.symbol_table);

    /* Generate code for every function, side affecting the code struct.
     * Each function is allowed to use the registers from min_regs variable
     * and up. */
    for function in &self.funcs {
      function.generate(scope, code, regs);
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
      symbol_table: self.statement.0.clone(),
    }
    .generate(scope, code, regs);
  }
}

impl Generatable for Func {
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg]) {
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
    code.text.push(Asm::Directive(Directive::Label(format!(
      "{}{}",
      if main { "" } else { "f_" },
      self.ident
    ))));

    /* Save link register.
    PUSH {lr} */
    code.text.push(Asm::always(Instr::Push(Reg::Link)));

    /* Make new 4 byte scope to reserve space for link register. */
    let mut lr_table = SymbolTable::default();
    lr_table.size = 4;
    let scope = &scope.new_scope(&lr_table);

    /* Allocate space on stack for local vars. */
    if self.symbol_table.size != 0 {
      /* Don't modify sp if we're only doing to decrement by 0. */
      code.text.push(Asm::always(Instr::Binary(
        BinaryInstr::Sub,
        Reg::StackPointer,
        Reg::StackPointer,
        Op2::Imm(self.symbol_table.size),
        false,
      )));
    }

    /* Move into function scope. */
    let scope = &scope.new_scope(&self.symbol_table);

    /* Generate body.
    SUB sp, sp, #4
    LDR r4, =5
    STR r4, [sp]
    LDR r4, [sp, #8]
    MOV r0, r4
    ADD sp, sp, #4 */
    self.body.generate(scope, code, regs);

    /* Main function implicitly ends in return 0. */
    if main {
      code.text.push(Asm::always(Instr::Load(
        DataSize::Word,
        Reg::RegNum(0),
        LoadArg::Imm(0),
      )))
    }

    /* Jump back to caller.
    POP {pc} */
    code.text.push(Asm::always(Instr::Pop(Reg::PC)));

    /* Put a second jump if not in main to mimick refcompile behaviour.
    POP {pc} */
    if !main {
      code.text.push(Asm::always(Instr::Pop(Reg::PC)));
    }

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
