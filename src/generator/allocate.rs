use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use super::*;

/* ======== LIVENESS ANALYSIS ======== */

pub fn calculate_liveness<'cfg>(cfg: &mut CFG<'_>) {
  let allocation_order = cfg.ordering.clone();

  // Until things stop changing
  let mut updated = true;
  while updated {
    updated = false;
    for cell in allocation_order.iter().rev() {
      let new_live_out: HashSet<VegNum>;
      let new_live_in: HashSet<VegNum>;
      {
        let uses = &cell.borrow().uses;
        let defines: &HashSet<VegNum> = &cell.borrow().defines;
        let live_out: &HashSet<VegNum> = &cell.borrow().live_out;
        let live_in = &cell.borrow().live_in;

        let succs = &cell.borrow().succs;

        // LiveIn(n) = uses(n) or (LiveOut(n), - defines(n))
        let live_out_without_defines: HashSet<VegNum> = live_out
          .difference(defines)
          .into_iter()
          .map(|x| *x)
          .collect();

        new_live_in = uses
          .union(&live_out_without_defines)
          .into_iter()
          .map(|x| *x)
          .collect();

        // LiveOut(n) = OR LiveIn(succs(n))
        new_live_out = succs
          .iter()
          .flat_map(|(_, succ_cell)| succ_cell.borrow().live_in.clone())
          .collect();

        updated |= (*live_in != new_live_in) || (*live_out != new_live_out);
      }
      cell.borrow_mut().live_in = new_live_in;
      cell.borrow_mut().live_out = new_live_out;
    }
  }
}

/* ======== INTERFERENCE CALCULATION ======== */

pub type Interference = Vec<(VegNum, HashSet<VegNum>)>;

pub fn calculate_interference(cfg: &mut CFG<'_>) -> Interference {
  let allocation_order = cfg.ordering.clone();

  // Pre-condition: We don't want to be colouring any non-general VegNumisters
  let mut interferences: Vec<(VegNum, HashSet<VegNum>)> = Vec::new();
  for veg_num in 1..(cfg.vegs + 1) {
    let mut current_interferences: HashSet<VegNum> = HashSet::from([veg_num]);

    for cell in allocation_order.iter() {
      if cell.borrow().live_out.contains(&veg_num) {
        current_interferences = current_interferences
          .union(&cell.borrow().live_out)
          .into_iter()
          .map(|x| *x)
          .collect()
      }
    }
    if current_interferences.len() > 0 {
      interferences.push((veg_num, current_interferences));
    }
  }

  interferences
}

/* ======== GRAPH COLOURING ======== */

#[derive(Debug, Clone)]
pub enum Location {
  Spill(usize),
  Reg(usize),
}

impl Location {
  pub fn _reg(&self) -> Reg {
    if let Location::Reg(rn) = self {
      Reg::General(GENERAL_REGS[*rn])
    } else {
      unimplemented!("spilling not done")
    }
  }
}

pub struct Colouring<T>(pub HashMap<T, Location>);

impl<T> Colouring<T> {
  pub fn num_slots(&self) -> usize {
    let max_slot_n = self
      .0
      .values()
      .filter_map(|loc| {
        if let Location::Spill(n) = loc {
          Some(*n)
        } else {
          None
        }
      })
      .max();

    if let Some(n) = max_slot_n {
      n + 1
    } else {
      0
    }
  }
}

pub fn colour<T>(
  interference: Vec<(T, HashSet<T>)>,
  colours: usize,
) -> Colouring<T>
where
  T: Eq + Clone + Hash + Debug,
{
  Colouring(colour_inner(interference, colours))
}

pub fn colour_inner<T>(
  interference: Vec<(T, HashSet<T>)>,
  colours: usize,
) -> HashMap<T, Location>
where
  T: Eq + Clone + Hash + Debug,
{
  // Note: Every node is a neighbor of itself.

  // Easy neighbor  - Node with fewer vertices than the number of colours.
  // Coloured stack - Stack of nodes that will be simply associated with a
  //                  colour.

  // Create a copy of the interference graph which we will be side affecting
  // The original interference graph will be required if any nodes are
  // spilled
  let mut interference_copy = interference.clone();

  // TODO: Change this from string to either concrete reg or stack offset
  let mut allocation: HashMap<T, Location> = HashMap::new();
  let mut colourable_stack: Vec<(T, HashSet<T>)> = vec![];

  let mut spill_location = 0;

  while !interference_copy.is_empty() {
    // Remove all easy neighbors
    remove_easy_neighbors(
      &mut interference_copy,
      &mut colourable_stack,
      colours,
    );
    if !interference_copy.is_empty() {
      println!("No easy neighbors left. ");
      // Removes a node from the interference copy, gives it a spill location
      // and adds it to the allocation.
      spill_node(&mut interference_copy, &mut allocation, &mut spill_location);
      println!("Node spilled");
    }
  }

  if !allocation.is_empty() {
    // Re-start the algorithm except remove the spilled nodes
    // return the result of this. Will not run below code
    // Join the resultant HashMap use .extend interference_copy = interference.clone();
    interference_copy = interference.clone();
    let mut i = 0;
    while i < interference_copy.len() {
      if allocation.contains_key(&interference_copy[i].0) {
        remove_node(&mut interference_copy, i);
      } else {
        i = i + 1;
      }
    }

    let result = colour_inner(interference_copy, colours);
    allocation.extend(result);
  } else {
    assert!(interference_copy.is_empty());

    // TODO: Pop all the nodes from the stack, bringing back the original connections
    colour_stack(&mut colourable_stack, &mut allocation, colours);
  }
  allocation
}

fn spill_node<T>(
  interference: &mut Vec<(T, HashSet<T>)>,
  allocation: &mut HashMap<T, Location>,
  spill_location: &mut usize,
) where
  T: Eq,
  T: Hash,
  T: Clone,
  T: Debug,
{
  // We know all nodes will not be easy neighbors, so we can spill any of them
  let (node, _) = remove_node(interference, 0);
  allocation.insert(node, Location::Spill(*spill_location));
  *spill_location = *spill_location + 1;
}

fn colour_stack<T>(
  colourable_stack: &mut Vec<(T, HashSet<T>)>,
  allocation: &mut HashMap<T, Location>,
  colours: usize,
) where
  T: Eq,
  T: Hash,
  T: Clone,
  T: Debug,
{
  // let mut coloured_interference = Vec::new();
  let mut node_colours: HashMap<T, usize> = HashMap::new();
  while !colourable_stack.is_empty() {
    let (node, neighbors) = colourable_stack.pop().unwrap();
    // Get all of the colours used by the neighbors or the current node
    // this should be guaranteed to be populated
    let neighbor_colours: Vec<usize> = neighbors
      .iter()
      .map(|neighbor| node_colours.get(neighbor))
      .filter(|x| matches!(x, Some(_)))
      .map(|x| x.unwrap().clone())
      .collect();

    let mut attempted_colour = 0;
    'inner: while attempted_colour < colours {
      if !neighbor_colours.contains(&attempted_colour) {
        node_colours.insert(node.clone(), attempted_colour);
        break 'inner;
      }
      attempted_colour = attempted_colour + 1;
    }

    // coloured_interference.push((colourable_stack.pop().unwrap(), "Colour"));
  }

  let new_allocations: HashMap<T, Location> = node_colours
    .iter()
    .map(|(k, v)| (k.clone(), Location::Reg(*v)))
    .collect();

  allocation.extend(new_allocations);
}

fn remove_easy_neighbors<T>(
  interference: &mut Vec<(T, HashSet<T>)>,
  colourable_stack: &mut Vec<(T, HashSet<T>)>,
  colours: usize,
) where
  T: Eq,
  T: Hash,
  T: Clone,
  T: Debug,
{
  // There could exist an easy neighbor
  let mut easy_neighbors = true;

  // Repeatedly loop through the nodes of the graph, removing easy neighbors
  // until we run out of them.
  while easy_neighbors {
    // There could only be easy neighbors left if we remove nodes during
    // this iteration.

    // If we complete the entire iteration without removing any nodes,
    // there are no easy neighbors left.
    easy_neighbors = false;

    // We will potentially remove from the interference graph if there
    // are easy neighbors in the current state of the graph.
    let mut i = 0;
    while i < interference.len() {
      // number of neighbors -1 as every node is a neighbor of itself
      if interference[i].1.len() - 1 < colours.into() {
        // This node is an easy neighbor, remove it.
        let curr_node = remove_node(interference, i);

        // Push the node to the coloured stack
        colourable_stack.push(curr_node);

        // There could be more easy_neighbors now.
        easy_neighbors = true;
      } else {
        // Only update the index if we didn't remove a node
        i = i + 1;
      }
    }
  }
}

fn remove_node<T>(
  interference: &mut Vec<(T, HashSet<T>)>,
  i: usize,
) -> (T, HashSet<T>)
where
  T: Eq,
  T: Debug,
  T: Clone,
  T: Hash,
{
  // Remove this node from the interference graph
  let (curr_node, neighbors) = interference.remove(i);

  // Remove this node from being adjacent to any other node
  *interference = interference
    .iter_mut()
    .map(|(node, adj)| {
      adj.remove(&curr_node);
      (node.clone(), adj.clone())
    })
    .collect();

  (curr_node, neighbors)
}
