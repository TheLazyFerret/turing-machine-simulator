//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
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
use std::fmt;

/// Represents a blank in a tape's cell.
const BLANK: char = '\0';
/// How the blanks will be printed.
const BLANK_REP: char = 'β';

/// Maximum ammount of steps a single run can do before being cancelled.
const MAX_STEP: usize = 500;

/// Struct representing a deterministic Turing machine.
/// Inside the struct only will be present the definition of the TuringMachine.
/// The tapes will be independent of each run.
#[derive(Clone, Debug)]
pub struct TuringMachine {
  /// Initial transition.
  initial: usize,
  /// Transition function.
  function: Vec<HashMap<String, Transition>>,
  /// Set of the final acceptance states.
  acceptance: HashSet<usize>,
}

/// Public implementation.
impl TuringMachine {
  /// Creates a new TuringMachine instance.
  pub fn new_load() {
    todo!()
  }
}

/// Private implementation.
impl TuringMachine {
  /// Auxiliar function, representing each one of the steps of test().
  /// Returns true if it finished, false otherwise.
  fn step(&self, state: &mut usize, tapes: &mut Vec<Tape>) -> bool {
    todo!()
  }

  /// Add a transition to the Turing machine.
  /// Due the ammount of Tapes is known compile-time, it will take as parameters an array of
  fn add_transition(
    &mut self, state: usize, read: &str, tr: &Transition,
  ) -> Result<(), TuringMachineError> {
    self.resize_func_vec(state);
    if let Some(_) = self.function.get_mut(state).unwrap().insert(read.to_owned(), tr.clone()) {
      return Err(TuringMachineError::Indeterminancy);
    }
    todo!()
  }

  /// Resize the function Vector.
  fn resize_func_vec(&mut self, u: usize) {
    if self.function.len() <= u {
      self.function.resize_with(u + 1, || HashMap::new());
    }
  }
}

/// Enum for represting the multiple errors with TuringMachine.
#[derive(Clone, Copy, Debug)]
pub enum TuringMachineError {
  /// Multiple transitions with same (state, read).
  Indeterminancy,
  MaxStepsReached,
  UnmatchingSizes,
}

impl fmt::Display for TuringMachineError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      | TuringMachineError::Indeterminancy => {
        write!(f, "Multiple transitions for the same pair state-readed")
      },
      | TuringMachineError::MaxStepsReached => {
        write!(f, "Run stopped, reached the maximum ammount of steps")
      },
      | TuringMachineError::UnmatchingSizes => {
        write!(f, "The number of tapes doesn't coincide with the transition")
      },
    }
  }
}

/// Auxiliar function for Display: returns a visual representation of a symbol.
/// Basically prints β if the symbol is a blank, itself otherwise.
fn print_sym(x: char) -> char {
  if x == BLANK { BLANK_REP } else { x }
}

#[cfg(test)]
mod test {

  #[test]
  fn test_new_load() {}
}
