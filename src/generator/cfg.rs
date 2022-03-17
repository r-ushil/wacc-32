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

impl<'cfg> Add for Flow<'cfg> {
  type Output = Flow<'cfg>;

  /* Returns a flow which flows through both inputs.
  (A -> B) + (C -> D) == A -> D */
  fn add(self, rhs: Self) -> Self::Output {
    /* Attach the last node of this block to the first
    element of the next one. */
    self.last.borrow_mut().succs.push(rhs.first);

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
    self.last.borrow_mut().succs.push(rhs.first);

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
  // uses: Vec<Reg>,
  // defines: Vec<Reg>,
  // live_in: Vec<Reg>,
  // live_out: Vec<Reg>,
  /* This blocks successors. */
  succs: Vec<&'cfg BlockRef<'cfg>>,
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
    /* Create new block out of asm. */
    let block = Block {
      id: self.ordering.len(),
      asm,
      succs: vec![],
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

  /* Consumes this cfg, linearising all instructions into the text segment. */
  pub fn linearise(&mut self) {
    self.code.text.extend(
      self
        .ordering
        .iter()
        .filter_map(|r| Some(r.borrow().asm.as_ref()?.clone())),
    )
  }
}
