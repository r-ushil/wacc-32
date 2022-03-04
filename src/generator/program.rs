use super::*;

// #[derive(PartialEq, Debug, Clone)]
impl Generatable for Program {
  type Input = ();
  type Output = ();

  fn generate(&self, _: &Scope, code: &mut GeneratedCode, regs: &[GenReg], _aux: ()) {
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

impl Generatable for Func {
  type Input = ();
  type Output = ();

  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[GenReg], _aux: ()) {
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

    let body_st_size = self.body_st.size;

    /* Allocate space on stack for local vars. */
    code.text.append(&mut Op2::imm_unroll(
      |offset| {
        Asm::always(Instr::Binary(
          BinaryInstr::Sub,
          Reg::StackPointer,
          Reg::StackPointer,
          Op2::Imm(offset),
          false,
        ))
      },
      body_st_size,
    ));

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
      let body_st_size = scope.get_total_offset();

      code.text.append(&mut Op2::imm_unroll(
        |offset: i32| {
          Asm::always(Instr::Binary(
            BinaryInstr::Add,
            Reg::StackPointer,
            Reg::StackPointer,
            Op2::Imm(offset),
            false,
          ))
        },
        body_st_size,
      ));

      code.text.push(Asm::always(Instr::Load(
        DataSize::Word,
        Reg::Arg(ArgReg::R0),
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
