use std::cell::Cell;

use typed_arena::Arena;

use super::*;

#[derive(PartialEq, Debug, Clone)]
pub enum LabelPrefix {
  Func,
  AnonFunc,
}

// #[derive(PartialEq, Debug, Clone)]
impl Generatable for Program {
  type Input = ();
  type Output = ();

  fn generate(
    &self,
    _: &ScopeReader,
    code: &mut GeneratedCode,
    regs: &[GenReg],
    _aux: (),
  ) {
    /* No registers should be in use by this point. */
    assert!(regs == GENERAL_REGS);

    /* Move into program's scope. */
    let scope = &ScopeReader::new(&self.symbol_table);

    /* Generate code for every function, side affecting the code struct.
     * Each function is allowed to use the registers from min_regs variable
     * and up. */
    for function in &self.funcs {
      function.generate(scope, code, regs, LabelPrefix::Func);
    }
    /* The statement of the program should be compiled as if it is in a
     * function called main, which takes nothing and returns an int exit code */

    (
      WACC_PROGRAM_MAIN_LABEL.to_string(),
      Func {
        signature: FuncSig {
          param_types: Vec::new(),
          return_type: Type::Int,
        },
        body: *self.statement.1.clone(),
        params_st: SymbolTable::default(),
        body_st: self.statement.0.clone(),
        param_ids: Vec::new(),
        vegs: Cell::new(0),
      },
    )
      .generate(scope, code, regs, LabelPrefix::Func);

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

// impl CFGable for NamedFunc {
//   type Input = LabelPrefix;

//   #[must_use]
//   fn cfg_generate<'a, 'cfg>(
//     &self,
//     scope: &ScopeReader,
//     cfg: &'a mut CFG<'cfg>,
//     regs: &[GenReg],
//     aux: Self::Input,
//   ) -> Flow<'cfg> {
//     todo!()
//   }
// }

impl Generatable for NamedFunc {
  type Input = LabelPrefix;
  type Output = ();

  fn generate(
    &self,
    scope: &ScopeReader,
    code: &mut GeneratedCode,
    _regs: &[GenReg],
    aux: Self::Input,
  ) {
    let (ident, func) = self;

    // TODO: make this a more robust check
    let main = ident == WACC_PROGRAM_MAIN_LABEL;

    /* Make control flow graph to write this function into. */
    let arena = Arena::new();

    let prefixxxed_ident = if main {
      ident.to_string()
    } else {
      match aux {
        LabelPrefix::Func => generate_function_name(ident.to_string()),
        LabelPrefix::AnonFunc => generate_anon_func_name(ident.to_string()),
      }
    };

    let mut cfg = CFG::new(code, &arena, prefixxxed_ident);

    /* Comments reflect the following example:
    int foo(int x) is
      int y = 5;
      return x
    end */
    /* Function label.
    foo: */
    // let mut flow = cfg.flow(Asm::Directive(Directive::Label(if main {
    //   ident.to_string()
    // } else {
    //   match aux {
    //     LabelPrefix::Func => generate_function_name(ident.to_string()),
    //     LabelPrefix::AnonFunc => generate_anon_func_name(ident.to_string()),
    //   }
    // })));

    /* Save link register.
    PUSH {lr} */
    // flow += cfg.flow(Asm::push(Reg::Link));

    /* Allocate space on stack for local vars. */
    // flow += cfg.imm_unroll(
    //   |offset| Asm::sub(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset)),
    //   body_st_size,
    // );

    /* Move into parameter scope. */
    let scope = &scope.new_scope(&func.params_st);

    /* Move into function body scope. */
    let scope = &scope.new_scope(&func.body_st);

    /* Generate body.
    SUB sp, sp, #4
    LDR r4, =5
    STR r4, [sp]
    LDR r4, [sp, #8]
    MOV r0, r4
    ADD sp, sp, #4 */
    let mut flow = func.body.cfg_generate(scope, &mut cfg, ());

    /* Main function implicitly ends in return 0. */
    if main {
      /* Deallocate stack for main function. */
      // let body_st_size = scope.get_total_offset();

      // flow += cfg.imm_unroll(
      //   |offset: i32| {
      //     Asm::add(Reg::StackPointer, Reg::StackPointer, Op2::Imm(offset))
      //   },
      //   body_st_size,
      // );

      flow += cfg.flow(Asm::ldr(Reg::Arg(ArgReg::R0), 0))
    }

    /* Jump back to caller.
    POP {pc} */
    // flow += cfg.flow(Asm::pop(Reg::PC));

    /* Mark block for compilations.
    .ltorg */
    // flow += cfg.flow(Asm::Directive(Directive::Assemble));

    /* We don't actually use flow but the process of creating
    it creates all the cfg links that we do need. */

    /* Linearise CFG. (Saving it into generated code) */
    cfg.save();
  }
}
