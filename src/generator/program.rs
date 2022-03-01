use super::*;

// #[derive(PartialEq, Debug, Clone)]
impl Generatable for Program {
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, min_regs: &mut u8) {
    /* Move into program's scope. */
    let scope = &scope.new_scope(&self.symbol_table);

    /* Generate code for every function, side affecting the code struct.
     * Each function is allowed to use the registers from min_regs variable
     * and up. */
    for function in &self.funcs {
      function.generate(scope, code, min_regs);
    }
    /* The statement of the program should be compiled as if it is in a
     * function called main, which takes nothing and returns an int exit code */
    Func {
      ident: String::from("main"),
      signature: FuncSig {
        params: Vec::new(),
        return_type: Type::Int,
      },
      body: self.statement.clone(),
      symbol_table: self.symbol_table.clone(),
    }
    .generate(scope, code, min_regs);
  }
}

impl Generatable for Func {
  // fn generate(&self, _code: &mut Vec<Instr>, min_regs: &mut i32) {}
}

#[cfg(test)]
mod tests {}
