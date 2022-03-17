use std::collections::HashSet;

use super::*;

/* ======== LIVENESS ANALYSIS ======== */

pub fn calculate_liveness<'cfg>(_cfg: &mut CFG<'_>) {
  println!("Populating live ins and live outs!");
}

/* ======== INTERFERENCE CALCULATION ======== */

pub type Interference = Vec<(RegRef, HashSet<RegRef>)>;

pub fn calculate_interference(_cfg: &mut CFG<'_>) -> Interference {
  println!("Calculating interferences!");
  vec![]
}

/* ======== GRAPH COLOURING ======== */

pub struct Colouring;

pub fn colour(_interference: Interference) -> Colouring {
  Colouring
}
