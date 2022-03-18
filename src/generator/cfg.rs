use std::{
  cell::{Cell, RefCell},
  collections::HashSet,
  fmt::Display,
  ops::{Add, AddAssign},
};

use typed_arena::Arena;

use super::*;
use allocate::*;

/* The type passed around by everyone to refer to a block. */
pub type BlockRef<'cfg> = RefCell<Block<'cfg>>;

/* Describes a flow through the control flow graph, specified
by a reference to the start and end block. */
pub struct Flow<'cfg> {
  first: &'cfg BlockRef<'cfg>,
  last: &'cfg BlockRef<'cfg>,
}

impl<'cfg> Flow<'cfg> {
  /* Adds a successor to this flow WITHOUT
  extending it's exit point. */
  pub fn add_succ(&self, succ: &Flow<'cfg>) {
    self.last.borrow_mut().add_succ(succ.first);
  }

  pub fn add_succ_cond(&self, cond: CondCode, succ: &Flow<'cfg>) {
    self.last.borrow_mut().add_succ_cond(cond, succ.first);
  }

  /* Returns a new flow which connects two flows,
  but doesn't connect their links. */
  pub fn tunnel(&self, other: &Flow<'cfg>) -> Flow<'cfg> {
    Flow {
      first: self.first,
      last: other.last,
    }
  }
}

impl<'cfg> Add for Flow<'cfg> {
  type Output = Flow<'cfg>;

  /* Returns a flow which flows through both inputs.
  (A -> B) + (C -> D) == A -> D */
  fn add(self, rhs: Self) -> Self::Output {
    /* Attach the last node of this block to the first
    element of the next one. */
    self.add_succ(&rhs);

    /* Return a flow which starts  */
    Self {
      first: self.first,
      last: rhs.last,
    }
  }
}

impl<'cfg> AddAssign for Flow<'cfg> {
  /* Extends a flow through another flow.
  let x = (A -> B);
  x += (C -> D)
  results in
  x == (A -> D) */
  fn add_assign(&mut self, rhs: Self) {
    /* Attach the last node of this block to the first
    element of the next one. */
    self.add_succ(&rhs);

    /* Extend this flow to the end of the other flow.  */
    self.last = rhs.last;
  }
}

/* Represents a basic block in the control flow graph.
One instruction per block. */
pub struct Block<'cfg> {
  /* Auto-increment id. */
  id: usize,
  /* Stored assembly. */
  asm: Option<Asm>,
  /* This blocks relationship to the rest of the graph. */
  pub uses: HashSet<VegNum>,
  pub defines: HashSet<VegNum>,
  pub live_in: HashSet<VegNum>,
  pub live_out: HashSet<VegNum>,
  /* This blocks successors. */
  pub succs: Vec<(CondCode, &'cfg BlockRef<'cfg>)>,
  /* Whether or not this block should generate a label when it's linearised. */
  needs_label: bool,
  label: Option<Label>,
}

impl<'cfg> Block<'cfg> {
  fn add_succ(&mut self, block_ref: &'cfg BlockRef<'cfg>) {
    self.add_succ_cond(CondCode::AL, block_ref);
  }

  fn add_succ_cond(&mut self, cond: CondCode, block_ref: &'cfg BlockRef<'cfg>) {
    self.succs.push((cond, block_ref));

    /* If the successor isn't immediately after this block in the
    ordering, it needs a label. */
    let mut succ = block_ref.borrow_mut();
    if !succ.follows(self) {
      println!("NEEDS LABEL: {:?}", succ.asm);
      succ.needs_label = true;
    }
  }

  /* Returns true if this block immediately follows other in the ordering. */
  fn follows(&self, other: &Block<'cfg>) -> bool {
    self.id == other.id + 1
  }

  fn get_label(&mut self, code: &mut GeneratedCode) -> Label {
    if let Some(label) = &self.label {
      label.clone()
    } else {
      let label = code.get_label();
      self.label = Some(label.clone());
      label
    }
  }
}

impl Display for CFG<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut visited: Vec<usize> = Vec::new();
    write!(f, "digraph {}", "{")?;
    dfs(self.ordering[0], &mut visited);
    write!(f, "{}", "}")?;
    Ok(())
  }
}

impl Display for Block<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} [label=\"{}: Uses={:?} Defines={:?}\"] {} -> {} ",
      self.id,
      // self.id,
      self
        .asm
        .clone()
        .unwrap_or(Asm::Directive(Directive::Label(format!("")))),
      self.uses,
      self.defines,
      self.id,
      // self.data.replace("\"", "\'"),
      "{"
    )
    .unwrap();
    for (_, child) in self.succs.iter() {
      write!(f, "{} ", child.borrow().id).unwrap();
    }
    write!(f, "{}", "}")
  }
}

pub fn dfs(entry: &BlockRef, visited: &mut Vec<usize>) {
  let node = &entry.borrow();
  println!("{}", node);
  visited.push(node.id);
  if node.succs.len() != 0 {
    for (_, child) in node.succs.iter() {
      if !visited.contains(&child.borrow().id) {
        dfs(child, visited);
      }
    }
  }
}

/* Represents an entire control flow graph. */
pub struct CFG<'cfg> {
  /* The GeneratedCode we are generating into. */
  pub code: &'cfg mut GeneratedCode,
  /* Arena to allocate blocks into. */
  arena: &'cfg Arena<BlockRef<'cfg>>,
  /* Ordered list of the blocks. */
  pub ordering: Vec<&'cfg BlockRef<'cfg>>,
  /* How many vegs have been used so far. */
  pub vegs: usize,
  /* The label used to jump to this CFG. */
  label: Label,
}

impl<'cfg> CFG<'cfg> {
  pub fn new(
    code: &'cfg mut GeneratedCode,
    arena: &'cfg Arena<BlockRef<'cfg>>,
    label: Label,
  ) -> CFG<'cfg> {
    CFG {
      code,
      arena,
      ordering: Vec::new(),
      vegs: 0,
      label,
    }
  }

  /* Creates a flow which starts and ends on a given instruction. */
  fn option_flow<'a>(&'a mut self, asm: Option<Asm>) -> Flow<'cfg> {
    /* Calculates which virtuals this instruction defines. */
    let (uses, defines) = if let Some(mut asm) = asm.clone() {
      (asm.uses(), asm.defines())
    } else {
      (HashSet::new(), HashSet::new())
    };

    /* Create new block out of asm. */
    let block = Block {
      id: self.ordering.len(),
      asm,
      succs: vec![],
      needs_label: false,
      label: None,
      uses,
      defines,
      live_in: HashSet::new(),
      live_out: HashSet::new(),
    };

    /* Allocate it in the arena. */
    let block_ref = self.arena.alloc(RefCell::new(block));

    /* Put this block in the ordering vector. */
    self.ordering.push(block_ref);

    /* Make a flow starting and ending from this block. */
    Flow {
      first: block_ref,
      last: block_ref,
    }
  }

  pub fn get_veg(&mut self) -> Reg {
    self.vegs += 1;
    Reg::Virtual(self.vegs)
  }

  #[must_use]
  pub fn flow<'a>(&'a mut self, asm: Asm) -> Flow<'cfg> {
    self.option_flow(Some(asm))
  }

  #[must_use]
  pub fn dummy_flow<'a>(&'a mut self) -> Flow<'cfg> {
    self.option_flow(None)
  }

  pub fn imm_unroll<'a, F>(
    &'a mut self,
    mut instr_builder: F,
    imm: Imm,
  ) -> Flow<'cfg>
  where
    F: FnMut(Imm) -> Asm,
  {
    let mut flow = self.dummy_flow();

    let imm_sign = imm.signum();
    let mut imm_abs = imm;

    while imm_abs > 0 {
      flow += self.flow(instr_builder(imm_sign * OP2_MAX_VALUE.min(imm_abs)));
      imm_abs -= OP2_MAX_VALUE;
    }

    flow
  }

  /* This takes ownership of the cfg so this is guarenteed to be the last
  operation on the cfg. */
  pub fn save(mut self) {
    /* Populate live ins and live outs. */
    allocate::calculate_liveness(&mut self);

    /* Create interference graph. */
    let mut interference = allocate::calculate_interference(&mut self);

    /* Colour interference graph. */
    let colouring = allocate::colour(interference, GENERAL_REGS.len());

    /* Calculate stack size for local variables. */
    let stack_size = (colouring.num_slots() * 4) as i32;

    /* Define functions which use colouring to load and save values. */
    let mut use_r4_load = true;
    let mut load_reg = |text: &mut Vec<Asm>, reg: Reg, offset: i32| match reg {
      Reg::Virtual(vn) => {
        let loc = colouring.0.get(&vn).unwrap().clone();
        match loc {
          Location::Reg(grn) => Reg::General(GENERAL_REGS[grn]),
          Location::Spill(slot) => {
            // todo
            use_r4_load = !use_r4_load;
            let tmp_reg =
              Reg::General(if use_r4_load { GenReg::R4 } else { GenReg::R5 });
            text.push(Asm::ldr(
              tmp_reg,
              LoadArg::MemAddress(
                Reg::StackPointer,
                (slot as i32) * 4 + offset,
              ),
            ));
            tmp_reg
          }
        }
      }
      Reg::FuncArg(arg_num) => {
        use_r4_load = !use_r4_load;
        let tmp_reg =
          Reg::General(if use_r4_load { GenReg::R4 } else { GenReg::R5 });
        text.push(Asm::ldr(
          tmp_reg,
          LoadArg::MemAddress(
            Reg::StackPointer,
            stack_size + 4 + (arg_num as i32) * 4,
          ),
        ));
        tmp_reg
      }
      _ => unimplemented!(),
    };

    let mut use_r4_save = true;
    let mut save_reg = |text: &mut Vec<Asm>, reg: Reg, offset: i32| match reg {
      Reg::Virtual(vn) => {
        let loc = colouring.0.get(&vn).unwrap().clone();
        match loc {
          Location::Reg(grn) => Reg::General(GENERAL_REGS[grn]),
          Location::Spill(slot) => {
            // todo
            use_r4_save = !use_r4_save;
            let tmp_reg =
              Reg::General(if use_r4_save { GenReg::R4 } else { GenReg::R5 });
            text.push(Asm::str(
              tmp_reg,
              (Reg::StackPointer, slot as i32 * 4 + offset),
            ));
            tmp_reg
          }
        }
      }
      Reg::FuncArg(arg_num) => {
        let tmp_reg = Reg::General(GenReg::R4);
        text.push(Asm::str(
          tmp_reg,
          (Reg::StackPointer, stack_size + 4 + (arg_num as i32) * 4),
        ));
        tmp_reg
      }
      _ => unimplemented!(),
    };

    let mut dealloc_stack = |text: &mut Vec<Asm>| {
      if stack_size != 0 {
        text.push(Asm::add(
          Reg::StackPointer,
          Reg::StackPointer,
          stack_size as i32,
        ));
      }
    };

    /* Label */
    self
      .code
      .text
      .push(Asm::Directive(Directive::Label(self.label.clone())));

    /* Save link register. */
    self.code.text.push(Asm::push(Reg::Link));

    /* Allocate space for stack. */
    /* TODO: make this an unroll. */
    if stack_size != 0 {
      self.code.text.push(Asm::sub(
        Reg::StackPointer,
        Reg::StackPointer,
        stack_size as i32,
      ));
    }

    /* Linearise while colouring. */
    self.linearise(&mut load_reg, &mut save_reg, &mut dealloc_stack);

    /* Mark block for compilations.
    .ltorg */
    self.code.text.push(Asm::Directive(Directive::Assemble));
  }

  /* Writes the cfg to code, transforming it from a graph to a linear
  structure. Call must add the a given assembly instruction to the vector,
  expanding into multiple instructions if nessecary. */
  fn linearise<F, G, H>(
    &mut self,
    mut load_reg: F,
    mut save_reg: G,
    mut dealloc_stack: H,
  ) where
    F: FnMut(&mut Vec<Asm>, Reg, i32) -> Reg,
    G: FnMut(&mut Vec<Asm>, Reg, i32) -> Reg,
    H: FnMut(&mut Vec<Asm>),
  {
    let Self {
      code,
      arena,
      ordering,
      vegs,
      label,
    } = self;

    for block in ordering.iter() {
      let mut block = block.borrow_mut();

      /* If anyone needs to jump to this block, add a label. */
      if block.needs_label {
        let label = block.get_label(code);
        code.text.push(Asm::Directive(Directive::Label(label)));
      }

      /* Generate block body. */
      if let Some(asm) = &block.asm {
        if let Asm::Call(return_reg, func_reg, arg_regs) = asm {
          let mut save_offset = 0;

          /* 1. Save all registers. */
          for gen_reg in GENERAL_REGS.iter() {
            code.text.push(Asm::push(Reg::General(*gen_reg)));
            save_offset += 4;
          }

          let mut total_offset = save_offset;

          /* 2. Put arguments on stack. */
          for arg_reg in arg_regs {
            let actual_reg = load_reg(&mut code.text, *arg_reg, total_offset);

            code.text.push(Asm::push(actual_reg));

            total_offset += 4;
          }

          /* 3. Branch. */
          /* 3.1. Load function register. */
          let actual_reg = load_reg(&mut code.text, *func_reg, total_offset);

          /* 3.2. BLX to function register. */
          code.text.push(Asm::bx(actual_reg).link());

          /* Get rid of argument that were pushed onto stack. */
          code.text.push(Asm::add(
            Reg::StackPointer,
            Reg::StackPointer,
            total_offset - save_offset,
          ));

          /* 4. Restore registers. */
          for gen_reg in GENERAL_REGS.iter().rev() {
            code.text.push(Asm::pop(Reg::General(*gen_reg)));
          }

          /* 5. Put r0 into return_reg. */
          let mut added = Vec::new();
          let actual_reg = save_reg(&mut added, *return_reg, 0);
          code.text.push(Asm::mov(actual_reg, ArgReg::R0));
          code.text.extend(added);
        } else if let Asm::Ret = asm {
          dealloc_stack(&mut code.text);
          code.text.push(Asm::pop(Reg::PC));
        } else if !asm.is_useless() {
          /* Take a copy so we can mutate it. */
          let mut asm: Asm = asm.clone();

          /* Load the used registers. */
          {
            asm.map_uses(|reg: &mut Reg| {
              *reg = load_reg(&mut code.text, *reg, 0);
            });
          }

          /* Add register to code block. */
          code.text.push(asm.clone());

          let asm = code.text.last_mut().unwrap();

          /* Save the defined registers. */
          let mut added = Vec::new();
          asm.map_defines(|reg| {
            *reg = save_reg(&mut added, *reg, 0);
          });
          code.text.extend(added);
        }
      }

      /* If block has no succs, return. */
      if block.succs.len() == 0 {
        /* De-allocate stack. */
        dealloc_stack(&mut code.text);

        /* LDR r0 #0 */
        // code.text.push(Asm::ldr(Reg::Arg(ArgReg::R0), 0));

        /* Jump to caller. */
        code.text.push(Asm::pop(Reg::PC));
      }

      /* Generate a branch to each successor, if one is required. */
      for (succ_cond, succ_block) in block.succs.iter() {
        let mut succ_block = (**succ_block).borrow_mut();

        if !succ_block.follows(&block) {
          let label = succ_block.get_label(code);
          code.text.push(Asm::b(label).cond(*succ_cond));
        }
      }
    }
  }
}
