//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Turing machine struct module.

mod tape;
mod tape_function;

use core::fmt;
use std::collections::HashSet;

/// Maximum ammount of steps a single run can do before being cancelled.
const MAX_STEP: usize = 500;

/// Struct representing a deterministic Turing machine.
#[derive(Clone, Debug)]
pub struct TuringMachine {
  /// The number of tapes.
  tapes_count: usize,
  /// Initial transition.
  initial: usize,
  /// Transition function.
  // todo~!
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
    todo!()
  }
}

/// Enum for represting the multiple errors with TuringMachine.
#[derive(Clone, Copy, Debug)]
pub enum TuringMachineError {
  /// Multiple transitions with same (state, read).
  Indeterminancy,
  MaxStepsReached,
}

impl fmt::Display for TuringMachineError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      | TuringMachineError::Indeterminancy => {
        write!(f, "Multiple transitions for the same pair state-readed")?
      },
      | TuringMachineError::MaxStepsReached => {
        write!(f, "Run stopped, reached the maximum ammount of steps")?
      },
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {

  #[test]
  fn test_add_transition() {}
}
