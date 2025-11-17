//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Turing machine struct module.

mod tape;
mod tape_function;
mod transition;

use crate::turing_machine::tape::Tape;
use crate::turing_machine::tape_function::TapeFunction;
use crate::turing_machine::transition::Transition;
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
  /// Creates a new TuringMachine instance.
  pub fn new_load(
    ntapes: usize, init: usize, accept: &HashSet<usize>, tr_func: &[&[Transition]],
  ) -> Result<Self, TuringMachineError> {
    let mut tmachine = TuringMachine {
      initial: init,
      ft: Vec::with_capacity(ntapes),
      acceptance: accept.clone(),
    };
    tmachine.ft.resize_with(ntapes, || TapeFunction::new());
    for t in tr_func.iter() {
      tmachine.add_transition(t)?;
    }
    Ok(tmachine)
  }

  /// Test is a string is accepted by the Turing machine
  pub fn test(&self, s: &str) -> Result<bool, TuringMachineError> {
    todo!()
  }
}

/// Private implementation.
impl TuringMachine {
  /// Add a transition to the Turing machine.
  /// Due the ammount of Tapes is known compile-time, it will take as parameters an array of
  fn add_transition(&mut self, trs: &[Transition]) -> Result<(), TuringMachineError> {
    if trs.len() != self.ft.len() {
      return Err(TuringMachineError::NotEnoughTransition(
        trs.len(),
        self.ft.len(),
      ));
    }
    for t in trs.iter().enumerate() {
      if self
        .ft
        .get_mut(t.0)
        .expect("Unexpected error getting vector")
        .add(*t.1)
        .is_some()
      {
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
  use std::collections::HashSet;

  use crate::turing_machine::{
    TuringMachine, TuringMachineError,
    transition::{Direction, Transition},
  };

  fn new_load() -> Result<TuringMachine, TuringMachineError> {
    let tr1 = Transition::new((0, 'a', 'b', 1, Direction::Left));
    let tr2 = Transition::new((0, 'b', 'b', 1, Direction::Right));
    let tr3 = Transition::new((1, 'a', 'b', 2, Direction::Stop));
    let tr4 = Transition::new((1, 'b', 'b', 2, Direction::Right));
    let tr5 = Transition::new((2, 'a', 'a', 3, Direction::Stop));
    let tr6 = Transition::new((2, 'b', 'a', 3, Direction::Left));
    let tr_vec1 = vec![tr1, tr2];
    let tr_vec2 = vec![tr3, tr4];
    let tr_vec3 = vec![tr5, tr6];
    let tr_func = vec![tr_vec1.as_slice(), tr_vec2.as_slice(), tr_vec3.as_slice()];
    let mut acceptance = HashSet::new();
    acceptance.insert(3);
    TuringMachine::new_load(2, 0, &acceptance, tr_func.as_slice())
  }

  #[test]
  fn test_new_load() {
    new_load().unwrap();
  }
}
