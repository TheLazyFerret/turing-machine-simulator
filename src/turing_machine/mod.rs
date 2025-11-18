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
  /// Tape number.
  ntapes: usize,
  /// Transition function.
  function: Vec<HashMap<Vec<char>, Transition>>,
  /// Set of the final acceptance states.
  acceptance: HashSet<usize>,
}

impl TuringMachine {
  /// Creates a new TuringMachine instance.
  pub fn new(initial: usize, ntapes: usize, accept: &HashSet<usize>) -> Self {
    TuringMachine {
      initial: initial,
      ntapes: ntapes,
      function: Vec::new(),
      acceptance: accept.clone(),
    }
  }

  pub fn test(s: &str) -> Result<bool, TuringMachineError> {
    todo!()
  }

  /// Auxiliar function, representing each one of the steps of test().
  /// Returns true if it finished, false otherwise.
  fn step(&self, state: &mut usize, tapes: &mut Vec<Tape>) -> bool {
    todo!()
  }

  /// Add a transition to the Turing machine.
  /// Due the ammount of Tapes is known compile-time, it will take as parameters an array of
  fn add_transition(
    &mut self, state: usize, read: &[char], tr: &Transition,
  ) -> Result<(), TuringMachineError> {
    self.resize_func_vec(state);
    // Check the number of tapes is the same as the number of operations in the transition.
    if (self.ntapes != tr.oper().len()) || (self.ntapes != read.len()) {
      return Err(TuringMachineError::UnmatchingSizes);
    }
    if let Some(_) = self.function.get_mut(state).unwrap().insert(read.to_owned(), tr.clone()) {
      return Err(TuringMachineError::Indeterminancy);
    }
    Ok(())
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
  use std::collections::HashSet;

  use crate::turing_machine::{
    TuringMachine,
    transition::{Direction, Transition},
  };

  #[test]
  fn test_add_transition() {
    let trans0 = Transition::new(&vec![('a', Direction::Left), ('b', Direction::Right)], 1);
    let trans1 = Transition::new(&vec![('b', Direction::Left), ('c', Direction::Left)], 2);
    let mut tr: TuringMachine = TuringMachine::new(0, 2, &HashSet::from([0, 1, 2]));
    tr.add_transition(0, &vec!['a', 'a'], &trans0).expect("Error shouldn't have been here");
    tr.add_transition(15, &vec!['\0', '\0'], &trans1).expect("Error shouldn't have been here");
    tr.add_transition(0, &vec!['a', 'a'], &trans0).expect_err("Here should have been an error");
  }
}
