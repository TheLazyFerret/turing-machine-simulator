//! Author: TheLazyFerret (<https://github.com/TheLazyFerret>)
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Turing machine struct module.

mod tape;
mod transition;

use crate::turing_machine::transition::Transition;
use crate::turing_machine::{tape::Tape, transition::Direction};
use core::fmt;
use std::collections::{HashMap, HashSet};

/// Struct representing a deterministic Turing machine.
#[derive(Clone, Debug, Default)]
pub struct TuringMachine {
  /// Vector of one or more tapes.
  tapes: Vec<Tape>,
  /// Initial transition.
  initial: usize,
  /// - Vec: each position representing an state
  /// - Hashmap Key: readed character.
  /// - Hashmap Value: (write, next state, direction).
  tr_func: Vec<HashMap<char, Transition>>,
  /// Set of the final acceptance states.
  acceptance: HashSet<usize>,
}

/// Public implementation.
impl TuringMachine {
  /// Returns a new, empty TuringMachine Instance.
  pub fn new() -> Self {
    TuringMachine::default()
  }
}

/// Private implementation.
impl TuringMachine {
  /// Add a transition to the Turing Machine.
  /// If already exist a transition with the same pair (state, read),
  /// returns an error.
  fn add_transition(
    &mut self, state: usize, tr: (char, char, usize, Direction),
  ) -> Result<(), TuringMachineError> {
    if self.tr_func.len() <= state {
      self.tr_func.resize_with(state + 1, Default::default);
    }
    if let Some(_) = self
      .tr_func
      .get_mut(state)
      .expect("unexpected error unwraping tr_func")
      .insert(tr.0, Transition::new(tr))
    {
      Err(TuringMachineError::Indeterminancy)
    } else {
      Ok(())
    }
  }
}

/// Enum for represting the multiple errors with TuringMachine.
#[derive(Clone, Copy, Debug)]
pub enum TuringMachineError {
  /// Multiple transitions with same (state, read).
  Indeterminancy,
}

impl fmt::Display for TuringMachineError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      | TuringMachineError::Indeterminancy => {
        write!(f, "Multiple transitions for the same pair (state, readed)")?
      },
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::turing_machine::{TuringMachine, transition::Direction};

  #[test]
  fn test_add_transition() {
    let mut x = TuringMachine::new();
    let tr1 = ('a', 'b', 4, Direction::Left);
    let tr2 = ('b', 'b', 4, Direction::Left);
    x.add_transition(0, tr1).unwrap();
    x.add_transition(0, tr2).unwrap();
    x.add_transition(1, tr1).unwrap();
    x.add_transition(1, tr1)
      .expect_err("Should have been an error here");
  }
}
