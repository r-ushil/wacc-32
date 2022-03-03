use super::{predef::ReadFmt, predef::RequiredPredefs, *};
use Directive::*;
use Instr::*;

impl Generatable for AssignLhs {
  type Input = Type;
  type Output = ();

  /* Writes regs[0] to value specified by AssignLhs */
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg], t: Type) {
    match self {
      AssignLhs::Ident(id) => {
        let offset = scope.get_offset(id).unwrap();

        code.text.push(Asm::always(Instr::Store(
          t.size().into(),
          regs[0],
          (Reg::StackPointer, offset),
        )))
      }
      AssignLhs::ArrayElem(elem) => {
        /* Store address of array element into regs[1]. */
        let elem_size = elem.generate(scope, code, &regs[1..], ());

        /* *regs[1] = regs[0] */
        code
          .text
          .push(Asm::always(Instr::Store(elem_size, regs[0], (regs[1], 0))));
      }
      AssignLhs::PairElem(elem) => {
        /* Stores address of elem in regs[1]. */
        let elem_size = elem.generate(scope, code, &regs[1..], ());

        /* *regs[1] = regs[0] */
        code
          .text
          .push(Asm::always(Instr::Store(elem_size, regs[0], (regs[1], 0))));
      }
      _ => code.text.push(Asm::Directive(Directive::Label(format!(
        "{:?}.generate(...)",
        self
      )))),
    }
  }
}

/* Mallocs {bytes} bytes and leaves the address in {reg}. */
fn generate_malloc(bytes: i32, code: &mut GeneratedCode, reg: Reg) {
  /* LDR r0, ={bytes} */
  code.text.push(Asm::always(Instr::Load(
    DataSize::Word,
    Reg::RegNum(0),
    LoadArg::Imm(bytes),
  )));

  /* BL malloc */
  code
    .text
    .push(Asm::always(Instr::Branch(true, String::from("malloc"))));

  /* MOV {regs[0]}, r0 */
  if reg != Reg::RegNum(0) {
    code.text.push(Asm::always(Instr::Unary(
      UnaryInstr::Mov,
      reg,
      Op2::Reg(Reg::RegNum(0), 0),
      false,
    )));
  }
}

impl Generatable for AssignRhs {
  type Input = Type;
  type Output = ();

  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg], t: Type) {
    match self {
      AssignRhs::Expr(expr) => expr.generate(scope, code, regs, ()),
      AssignRhs::ArrayLiter(ArrayLiter(exprs)) => {
        /* Calculate size of elements. */
        let elem_size = match t {
          Type::Array(elem_type) => elem_type.size(),
          /* Semantic analyser should ensure this is an array. */
          _ => unreachable!(),
        };

        /* Malloc space for array. */
        generate_malloc(4 + elem_size * exprs.len() as i32, code, regs[0]);

        /* Write each expression to the array. */
        for (i, expr) in exprs.iter().enumerate() {
          /* Evaluate expr to r5. */
          expr.generate(scope, code, &regs[1..], ());

          /* Write r5 array. */
          code.text.push(Asm::always(Instr::Store(
            elem_size.into(),
            regs[1],
            (regs[0], 4 + (i as i32) * elem_size),
          )));
        }

        /* Write length to first byte.
        LDR r5, =3
        STR r5, [r4] */
        code.text.push(Asm::always(Instr::Load(
          DataSize::Word,
          regs[1],
          LoadArg::Imm(exprs.len() as i32),
        )));
        code.text.push(Asm::always(Instr::Store(
          DataSize::Word,
          regs[1],
          (regs[0], 0),
        )));
      }
      AssignRhs::Pair(e1, e2) => {
        let (e1_size, e2_size) = match t {
          Type::Pair(t1, t2) => (t1.size(), t2.size()),
          /* Semantic analyser should ensure this is a pair. */
          _ => unreachable!(),
        };

        /* Malloc for the pair.
        regs[0] = malloc(8) */
        generate_malloc(8, code, regs[0]);

        /* Evaluate e1.
        regs[1] = eval(e1) */
        e1.generate(scope, code, &regs[1..], ());

        /* Malloc for e1.
        r0 = malloc(e1_size) */
        generate_malloc(e1_size, code, Reg::RegNum(0));

        /* Write e1 to malloced space. */
        code.text.push(Asm::always(Instr::Store(
          e1_size.into(),
          regs[1],
          (Reg::RegNum(0), 0),
        )));

        /* Write pointer to e1 to pair. */
        code.text.push(Asm::always(Instr::Store(
          DataSize::Word,
          Reg::RegNum(0),
          (regs[0], 0),
        )));

        /* Evaluate e2.
        regs[1] = eval(e2) */
        e2.generate(scope, code, &regs[1..], ());

        /* Malloc for e2.
        r0 = malloc(e2_size) */
        generate_malloc(e2_size, code, Reg::RegNum(0));

        /* Write e2 to malloced space. */
        code.text.push(Asm::always(Instr::Store(
          e2_size.into(),
          regs[1],
          (Reg::RegNum(0), 0),
        )));

        /* Write pointer to e2 to pair. */
        code.text.push(Asm::always(Instr::Store(
          DataSize::Word,
          Reg::RegNum(0),
          (regs[0], 4),
        )));
      }
      AssignRhs::PairElem(elem) => {
        /* Puts element address in regs[0]. */
        let elem_size = elem.generate(scope, code, regs, ());

        /* Dereference. */
        code.text.push(Asm::always(Instr::Load(
          elem_size,
          regs[0],
          LoadArg::MemAddress(regs[0], 0),
        )));
      }
      _ => code.text.push(Asm::Directive(Directive::Label(format!(
        "{:?}.generate(...)",
        self
      )))),
    }
  }
}

fn generate_print(t: &Type, expr: &Expr, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg]) {
  expr.generate(scope, code, regs, ());

  match t {
    Type::Int => RequiredPredefs::PrintInt.mark(code),
    Type::Bool => RequiredPredefs::PrintBool.mark(code),
    Type::String => RequiredPredefs::PrintString.mark(code),
    Type::Char => RequiredPredefs::PrintString.mark(code),
    Type::Array(_) => RequiredPredefs::PrintString.mark(code),
    Type::Pair(_, _) => RequiredPredefs::PrintRefs.mark(code),
    _ => (),
  };

  let print_label = match t {
    Type::Int => predef::PREDEF_PRINT_INT,
    Type::Bool => predef::PREDEF_PRINT_BOOL,
    Type::String => predef::PREDEF_PRINT_STRING,
    Type::Char => predef::PREDEF_PRINT_STRING,
    Type::Array(_) => predef::PREDEF_PRINT_STRING,
    Type::Pair(_, _) => predef::PREDEF_PRINT_REFS,
    _ => unreachable!(),
  };

  code.text.push(Asm::always(Unary(
    UnaryInstr::Mov,
    Reg::RegNum(0),
    Op2::Reg(regs[0], 0),
    false,
  )));

  code
    .text
    .push(Asm::always(Branch(true, print_label.to_string())));
}

impl Generatable for PairElem {
  type Input = ();
  type Output = DataSize;

  /* Puts the address of the element in regs[0], returns size pointed to. */
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg], _aux: ()) -> DataSize {
    /*  */
    let (t, pair, offset) = match self {
      PairElem::Fst(t, pair) => (t, pair, 0),
      PairElem::Snd(t, pair) => (t, pair, 4),
    };

    /* Store address of pair in regs[0]. */
    pair.generate(scope, code, regs, ());

    /* CHECK: regs[0] != NULL */
    code.text.push(Asm::always(Instr::Unary(
      UnaryInstr::Mov,
      Reg::RegNum(0),
      Op2::Reg(regs[0], 0),
      false,
    )));
    code.text.push(Asm::always(Instr::Branch(
      true,
      String::from("p_check_null_pointer"),
    )));
    RequiredPredefs::CheckNullPointer.mark(code);

    /* Dereference. */
    code.text.push(Asm::always(Instr::Load(
      DataSize::Word,
      regs[0],
      LoadArg::MemAddress(regs[0], offset),
    )));

    /* Return how much data needs to be read from regs[0]. */
    t.size().into()
  }
}

impl Generatable for ScopedStat {
  type Input = ();
  type Output = ();
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg], aux: ()) {
    let ScopedStat(st, statement) = self;

    /* No need to decrement stack pointer if no vars declared. */
    let skip_decrement = st.size == 0;

    /* Allocate space on stack for variables declared in this scope. */
    if !skip_decrement {
      code.text.push(Asm::always(Instr::Binary(
        BinaryInstr::Sub,
        Reg::StackPointer,
        Reg::StackPointer,
        Op2::Imm(st.size),
        false,
      )));
    }

    /* Enter new scope. */
    let scope = scope.new_scope(st);

    /* Generated statement. */
    statement.generate(&scope.new_scope(st), code, regs, ());

    /* Increment stack pointer to old position. */
    if !skip_decrement {
      code.text.push(Asm::always(Instr::Binary(
        BinaryInstr::Add,
        Reg::StackPointer,
        Reg::StackPointer,
        Op2::Imm(st.size),
        false,
      )));
    }
  }
}

impl Generatable for Stat {
  type Input = ();
  type Output = ();
  fn generate(&self, scope: &Scope, code: &mut GeneratedCode, regs: &[Reg], aux: ()) {
    match self {
      Stat::Skip => (),
      Stat::Declaration(t, id, rhs) => {
        Stat::Assignment(AssignLhs::Ident(id.clone()), t.clone(), rhs.clone()).generate(
          scope,
          code,
          regs,
          (),
        );
      }
      Stat::Assignment(lhs, t, rhs) => {
        /* regs[0] = eval(rhs) */
        rhs.generate(scope, code, regs, t.clone());

        /* stores value of regs[0] into lhs */
        lhs.generate(scope, code, regs, t.clone());
      }
      // Stat::Read(expr) => {
      // TODO: expr is not and Expr or an Ident, function needs re-writing.
      // // expr is expected to be an identifier, needs to read into a variable
      // expr.generate(scope, code, regs, ()); //generate expr, load into min_reg

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
      //   ReadChar.mark(code);
      //   code.predefs.read_char = true; // TODO: Remove after switch.
      //   ReadFmt::Char
      // } else {
      //   code.predefs.read_int = true; // TODO: Remove after switch
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

      // expr.generate(scope, code, regs, ()); //load pair address into min_reg
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
      //set free_pair flag to true
      /* BL p_free_pair */
      // code.predefs.free_pair = true; // TODO: Remove after switch
      // FreePair.mark(code);
      //
      // code.text.push(Asm::Instr(
      //   CondCode::AL,
      //   Instr::Branch(true, String::from("p_free_pair")),
      // ));

      // *regs = *regs - 1; //decrement min_reg by 1, no longer needed
      // }
      Stat::Return(expr) => {
        /* regs[0] = eval(expr) */
        expr.generate(scope, code, regs, ());

        /* r0 = regs[0] */
        code.text.push(Asm::Instr(
          CondCode::AL,
          Instr::Unary(UnaryInstr::Mov, Reg::RegNum(0), Op2::Reg(regs[0], 0), false),
        ));

        // todo!()
        // total_offset = somehow get total stack offset for all local vars
        let total_offset = scope.get_total_offset();

        /* ADD sp, sp, #{total_offset} */
        if total_offset != 0 {
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
        }

        /* POP {pc} */
        code
          .text
          .push(Asm::Instr(CondCode::AL, Instr::Pop(Reg::PC)));
      }
      Stat::Exit(expr) => {
        /* regs[0] = eval(expr) */
        expr.generate(scope, code, regs, ());

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

      Stat::Print(t, expr) => {
        generate_print(t, expr, scope, code, regs);
      }

      Stat::Println(t, expr) => {
        generate_print(t, expr, scope, code, regs);

        /* BL println */
        RequiredPredefs::PrintLn.mark(code);
        code.text.push(Asm::always(Instr::Branch(
          true,
          predef::PREDEF_PRINTLN.to_string(),
        )));
      }
      Stat::If(cond, true_body, false_body) => {
        let false_label = code.get_label();
        let exit_label = code.get_label();

        /* regs[0] = eval(cond) */
        cond.generate(scope, code, regs, ());

        /* cmp(regs[0], 0) */
        code.text.push(Asm::always(Unary(
          UnaryInstr::Cmp,
          regs[0],
          Op2::Imm(0),
          false,
        )));

        /* Branch to false case if cond == 0. */
        code
          .text
          .push(Asm::Instr(CondCode::EQ, Branch(false, false_label.clone())));

        /* True body. */
        true_body.generate(scope, code, regs, ());

        /* Exit if statement. */
        code
          .text
          .push(Asm::always(Branch(false, exit_label.clone())));

        /* Label for false case to skip to. */
        code.text.push(Asm::Directive(Label(false_label)));

        /* False body. */
        false_body.generate(scope, code, regs, ());

        /* Label to exit if statement. */
        code.text.push(Asm::Directive(Label(exit_label)));
      }
      Stat::While(cond, body) => {
        let cond_label = code.get_label();
        let body_label = code.get_label();

        /* Jump to condition evaluation. */
        code
          .text
          .push(Asm::always(Instr::Branch(false, cond_label.clone())));

        /* Loop body label. */
        code.text.push(Asm::Directive(Label(body_label.clone())));

        /* Loop body. */
        body.generate(scope, code, regs, ());

        /* Cond label */
        code.text.push(Asm::Directive(Label(cond_label)));

        /* regs[0] = eval(cond) */
        cond.generate(scope, code, regs, ());

        /* cmp(regs[0], 1) */
        code.text.push(Asm::always(Unary(
          UnaryInstr::Cmp,
          regs[0],
          Op2::Imm(1),
          false,
        )));

        /* If regs[0] == 1, jump back to loop body. */
        code
          .text
          .push(Asm::Instr(CondCode::EQ, Branch(false, body_label.clone())));
      }
      Stat::Scope(stat) => stat.generate(scope, code, regs, ()),
      Stat::Sequence(head, tail) => {
        head.generate(scope, code, regs, ());
        tail.generate(scope, code, regs, ());
      }
      _ => code.text.push(Asm::Directive(Directive::Label(format!(
        "{:?}.generate(...)",
        self
      )))),
    }
  }
}

// todo!(), add parameter for expr_type
fn print_stat_gen(code: &mut GeneratedCode, min_reg: &mut RegNum) {

  //   let branch_name = match expr_type {
  //     Type::String => {
  //       PrintString.mark(code);
  //       code.predefs.print_strings = true; // TODO: Remove after switch
  //       String::from("p_print_string")
  //     }
  //     Type::Bool => {
  //       PrintBool.mark(code);
  //       code.predefs.print_bools = true; // TODO: Remove after switch
  //       String::from("p_print_bool")
  //     }
  //     Type::Int => {
  //       PrintInt.mark(code);
  //       code.predefs.print_ints = true; // TODO: Remove after switch
  //       String::from("p_print_int")
  //     }
  //     Type::Ref => {
  //       PrintRefs.mark(code):
  //       code.predefs.print_refs = true; // TODO: Remove after switch
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
    stat.generate(scope, &mut actual_code, regs, ());

    /* Expected output. */
    let mut expected_code = GeneratedCode::default();
    expr.generate(scope, &mut expected_code, regs, ());

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
      Instr::Branch(true, String::from("exit")),
    ));

    assert_eq!(format!("{}", actual_code), format!("{}", expected_code));
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
