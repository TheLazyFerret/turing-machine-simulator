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

  /// Read the current symbol of each tape, and return them.
  fn read(tapes: &[Tape]) -> Vec<char> {
    let mut x = Vec::new();
    tapes.iter().for_each(|tape| x.push(tape.read()));
    x
  }

  /// Add a transition to the Turing machine.
  /// Due the ammount of Tapes is known compile-time, it will take as parameters an array of
  fn insert_transition(
    &mut self, state: usize, read: &[char], tr: &Transition,
  ) -> Result<(), TuringMachineError> {
    if (tr.len() != self.ntapes) || (read.len() != self.ntapes) {
      return Err(TuringMachineError::UnmatchingSizes);
    }
    self.resize_func_vec(state);
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TuringMachineError {
  /// Multiple transitions with same (state, read).
  Indeterminancy,
  /// Reached the maximum number of allowed steps.
  MaxStepsReached,
  /// When adding a transition, the number of tapes doesn't coincide.
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

#[cfg(test)]
mod test {
  use std::collections::HashSet;

  use crate::turing_machine::{
    TuringMachine, TuringMachineError,
    transition::{Direction, Transition},
  };

  #[test]
  fn test_add_transition() {
    let tr1 =
      Transition::new(&vec!['a', 'a'], &vec![Direction::Right, Direction::Left], 1).unwrap();
    let tr2 = Transition::new(
      &vec!['a', '\0', '\0'],
      &vec![Direction::Stop, Direction::Right, Direction::Left],
      3,
    )
    .unwrap();
    let mut tm = TuringMachine::new(0, 2, &HashSet::from([0, 1, 2]));
    assert_eq!(tm.insert_transition(0, &vec!['a', 'a'], &tr1), Ok(()));
    assert_eq!(tm.insert_transition(10, &vec!['a', 'a'], &tr1), Ok(()));
    assert_eq!(tm.insert_transition(0, &vec!['b', 'a'], &tr1), Ok(()));
    assert_eq!(
      tm.insert_transition(0, &vec!['b', 'a'], &tr1),
      Err(TuringMachineError::Indeterminancy)
    );
    assert_eq!(
      tm.insert_transition(0, &vec!['a', 'b'], &tr2),
      Err(TuringMachineError::UnmatchingSizes)
    );
  }
}
