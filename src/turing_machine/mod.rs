//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Turing machine struct module.

mod tape;
mod tape_function;

use crate::turing_machine::tape_function::TapeFunction;
use crate::turing_machine::tape_function::transition::Transition;
use std::collections::HashSet;
use std::fmt;

/// Maximum ammount of steps a single run can do before being cancelled.
const MAX_STEP: usize = 500;

/// Struct representing a deterministic Turing machine.
/// Inside the struct only will be present the definition of the TuringMachine.
/// The tapes will be independent of each run.
#[derive(Clone, Debug)]
pub struct TuringMachine {
  /// Initial transition.
  initial: usize,
  /// Transition function. Each element represents a tape function.
  /// Also saves the number of Tapes.
  ft: Vec<TapeFunction>,
  /// Set of the final acceptance states.
  acceptance: HashSet<usize>,
}

/// Public implementation.
impl TuringMachine {
  /// Returns a new, empty TuringMachine Instance.
  pub fn new(ntapes: usize) -> Self {
    let mut f: Vec<TapeFunction> = Vec::with_capacity(ntapes);
    f.resize_with(ntapes, || TapeFunction::new());
    TuringMachine {
      initial: 0,
      ft: f,
      acceptance: HashSet::new(),
    }
  }

  pub fn run(&self, s: &str) -> Result<bool, TuringMachineError> {
    todo!()
  }
}

/// Private implementation.
impl TuringMachine {
  /// Add a transition to the Turing machine.
  /// Due the ammount of Tapes is known compile-time, it will take as parameters an array of
  fn add_transition(&mut self, trs: &[Transition]) -> Result<(), TuringMachineError> {
    if trs.len() != self.ft.len() {
      return Err(TuringMachineError::NotEnoughTransition(trs.len(), self.ft.len()));
    }
    for t in trs.iter().enumerate() {
      if self.ft.get_mut(t.0).expect("Unexpected error getting vector").add(*t.1).is_some() {
        return Err(TuringMachineError::Indeterminancy);
      }
    }
    Ok(())
  }

  /// Auxiliar function, representing each one of the steps of run().
  /// Returns true if it finished, false otherwise.
  fn run_step(&self, state: &mut usize) -> Result<bool, TuringMachineError> {
    todo!()
  }
}

/// Enum for represting the multiple errors with TuringMachine.
#[derive(Clone, Copy, Debug)]
pub enum TuringMachineError {
  /// Multiple transitions with same (state, read).
  Indeterminancy,
  MaxStepsReached,
  NotEnoughTransition(usize, usize),
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
      | TuringMachineError::NotEnoughTransition(x, y) => {
        write!(f, "Adding a new transition, but not satisfied {x}  {y}")
      },
    }
  }
}

#[cfg(test)]
mod test {
    use crate::turing_machine::{TuringMachine, tape_function::transition::{Direction, Transition}};


  #[test]
  fn test_add_transition() {
    let mut x = TuringMachine::new(2);
    let tr1 = Transition::new((0, 'a', 'b', 2, Direction::Left));
    let tr2 = Transition::new((0, 'b', 'c', 2, Direction::Stop));
    let tr3 = Transition::new((0, 'a', 'c', 4, Direction::Stop));
    let tr4 = Transition::new((1, 'a', 'a', 3, Direction::Right));
    let tr_vec1 = vec![tr1, tr2];
    let tr_vec2 = vec![tr3, tr4];
    assert_eq!(x.add_transition(&tr_vec1).is_ok(), true); // All fine
    assert_eq!(x.add_transition(&tr_vec2).is_err(), true); // Forced indeterminancy in the first tape.
  }
}
