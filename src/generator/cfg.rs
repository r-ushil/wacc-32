use std::{
  cell::RefCell,
  ops::{Add, AddAssign},
};

use typed_arena::Arena;

use super::*;

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
  uses: Vec<Reg>,
  defines: Vec<Reg>,
  // live_in: Vec<Reg>,
  // live_out: Vec<Reg>,
  /* This blocks successors. */
  succs: Vec<(CondCode, &'cfg BlockRef<'cfg>)>,
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

/* Represents an entire control flow graph. */
pub struct CFG<'cfg> {
  pub code: &'cfg mut GeneratedCode,
  arena: &'cfg Arena<BlockRef<'cfg>>,
  ordering: Vec<&'cfg BlockRef<'cfg>>,
}

impl<'cfg> CFG<'cfg> {
  pub fn new(
    code: &'cfg mut GeneratedCode,
    arena: &'cfg Arena<BlockRef<'cfg>>,
  ) -> CFG<'cfg> {
    CFG {
      code,
      arena,
      ordering: Vec::new(),
    }
  }

  /* Creates a flow which starts and ends on a given instruction. */
  fn option_flow<'a>(&'a mut self, asm: Option<Asm>) -> Flow<'cfg> {
    /* Calculates which virtuals this instruction defines. */
    let (uses, defines) = if let Some(asm) = &asm {
      (asm.uses(), asm.defines())
    } else {
      (vec![], vec![])
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
    let interference = allocate::calculate_interference(&mut self);

    /* Colour interference graph. */
    let _colouring = allocate::colour(interference);

    /* Linearise while colouring. */
    self.linearise(Vec::push);
  }

  /* Writes the cfg to code, transforming it from a graph to a linear
  structure. Call must add the a given assembly instruction to the vector,
  expanding into multiple instructions if nessecary. */
  fn linearise<F>(&mut self, mut push: F)
  where
    F: FnMut(&mut Vec<Asm>, Asm),
  {
    for block in self.ordering.iter() {
      let mut block = block.borrow_mut();

      /* If anyone needs to jump to this block, add a label. */
      if block.needs_label {
        let label = block.get_label(self.code);
        self.code.text.push(Asm::Directive(Directive::Label(label)));
      }

      /* Generate block body. */
      if let Some(asm) = &block.asm {
        push(&mut self.code.text, asm.clone());
      }

      /* Generate a branch to each successor, if one is required. */
      for (succ_cond, succ_block) in block.succs.iter() {
        let mut succ_block = (**succ_block).borrow_mut();

        if !succ_block.follows(&block) {
          let label = succ_block.get_label(self.code);
          self.code.text.push(Asm::b(label).cond(*succ_cond));
        }
      }
    }
  }
}
