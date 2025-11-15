//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Turing machine struct module.

mod tape;
mod tape_function;

use crate::turing_machine::tape::Tape;
use core::fmt;
use std::collections::{HashMap, HashSet};

/// Maximum ammount of steps a single run can do before being cancelled.
const MAX_COUNT: usize = 500;

/// Struct representing a deterministic Turing machine.
#[derive(Clone, Debug)]
pub struct TuringMachine {
  /// The number of tapes.
  tapes_count: usize,
  /// Initial transition.
  initial: usize,
  /// - Vec: each position representing an state
  /// - Hashmap Key: readed character.
  /// - Hashmap Value: (write, next state, direction).

  //tr_func: [TapeFunction; TAPES_COUNT],
  /// Set of the final acceptance states.
  acceptance: HashSet<usize>,
}

/// Public implementation.
impl TuringMachine {
  /// Returns a new, empty TuringMachine Instance.
  pub fn new() -> Self {
    TuringMachine::default()
  }

  pub fn run(&self, s: &str) -> Result<bool, TuringMachineError> {
    let mut tapes: Vec<Tape> = Vec::new();
    tapes.resize_with(self.tapes_count, Default::default);
    let mut current_state = self.initial;
    let mut count = 0;
    while self.run_step(&mut current_state)? {
      count += 1;
      if count >= MAX_COUNT {
        return Err(TuringMachineError::MaxCount);
      };
    }

    todo!()
  }
}

/// Private implementation.
impl TuringMachine {
  /// Add a transition to the Turing machine.
  /// Due the ammount of Tapes is known compile-time, it will take as parameters an array of
  fn add_transition() -> Result<(), TuringMachineError> {
    todo!()
  }

  /// Auxiliar function, representing each one of the steps of run().
  /// Returns true if it finished, false otherwise.
  fn run_step(&self, state: &mut usize) -> Result<bool, TuringMachineError> {
    todo!()
  }
}

impl Default for TuringMachine {
  fn default() -> Self {
    TuringMachine {
      tapes_count: 1,
      initial: 0,
      acceptance: HashSet::new(),
    }
  }
}

/// Enum for represting the multiple errors with TuringMachine.
#[derive(Clone, Copy, Debug)]
pub enum TuringMachineError {
  /// Multiple transitions with same (state, read).
  Indeterminancy,
  MaxCount,
}

impl fmt::Display for TuringMachineError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      | TuringMachineError::Indeterminancy => {
        write!(f, "Multiple transitions for the same pair (state, readed)")?
      },
      | TuringMachineError::MaxCount => write!(f, "Reached the maximum ammount of transitions")?,
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {

  #[test]
  fn test_add_transition() {}
}
