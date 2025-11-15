//! Author: TheLazyFerret (<https://github.com/TheLazyFerret>)
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Turing machine struct module.

mod tape;
mod transition;

use crate::turing_machine::tape::Tape;
use crate::turing_machine::transition::Transition;
use std::collections::{HashMap, HashSet};

/// Struct representing a deterministic Turing machine.
#[derive(Clone, Debug)]
pub struct TuringMachine {
  /// Vector of one or more tapes.
  tapes: Vec<Tape>,
  /// Initial transition.
  initial: usize,
  /// - Vec: each position representing an state
  /// - Hashmap Key: readed character.
  /// - Hashmap Value: (write, next state, direction).
  transition: Vec<HashMap<char, Transition>>,
  /// Set of the final acceptance states.
  acceptance: HashSet<usize>,
}

/// Public implementation.
impl TuringMachine {

}

/// Private implementation.
impl TuringMachine {
  fn add_transition(&mut self, ) {

  }
}