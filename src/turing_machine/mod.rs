//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Turing machine struct module.

mod tape;
pub mod transition;

use crate::error::Error;
use crate::turing_machine::tape::Tape;
use crate::turing_machine::transition::Transition;
use std::collections::{HashMap, HashSet};

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
  pub fn new(initial: usize, ntapes: usize, accept: &HashSet<usize>) -> Result<Self, Error> {
    if ntapes == 0 {
      Err(Error::TapeErrorCount)
    } else {
      Ok(TuringMachine {
        initial: initial,
        ntapes: ntapes,
        function: Vec::new(),
        acceptance: accept.clone(),
      })
    }
  }

  /// Run the simulator, loading the string.
  pub fn run(&self, s: &str) -> Result<bool, Error> {
    let mut tapes = vec![Tape::new(); self.ntapes];
    let mut current: usize = self.initial;
    let mut counter = 0;
    tapes.get_mut(0).unwrap().load_string(s);
    while self.step(&mut current, tapes.as_mut_slice()) {
      if counter >= MAX_STEP {
        return Err(Error::MaxStepsReached);
      }
      counter += 1;
    }
    Ok(self.acceptance.get(&current).is_some())
  }

  /// Auxiliar function, representing each one of the steps of test().
  /// Returns false if finished, true otherwise.
  fn step(&self, current: &mut usize, tapes: &mut [Tape]) -> bool {
    assert!(self.ntapes == tapes.len());
    let readed = Self::read_tapes(tapes);
    if let Some(x) = self.function.get(*current) {
      if let Some(x) = x.get(&readed) {
        *current = x.next();
        Self::update_tapes(tapes, x);
        true
      } else {
        false
      }
    } else {
      false
    }
  }

  /// Read the current symbol of each tape, and return them.
  fn read_tapes(tapes: &[Tape]) -> Vec<char> {
    let mut x = Vec::new();
    tapes.iter().for_each(|tape| x.push(tape.read()));
    x
  }

  /// Update the tapes
  fn update_tapes(tapes: &mut [Tape], tr: &Transition) {
    assert!(tapes.len() == tr.len());
    for tape in tapes.iter_mut().enumerate() {
      let char_to_write = tr.write_slice().get(tape.0).unwrap().clone();
      let direction_to_move = tr.move_slice().get(tape.0).unwrap().clone();
      tape.1.write(char_to_write);
      tape.1.mov(direction_to_move);
    }
  }

  /// Add a transition to the Turing machine.
  /// Due the ammount of Tapes is known compile-time, it will take as parameters an array of
  pub fn insert_transition(
    &mut self, state: usize, read: &[char], tr: &Transition,
  ) -> Result<(), Error> {
    if tr.len() != self.ntapes {
      return Err(Error::UnmatchingSizes(self.ntapes, tr.len()));
    } else if read.len() != self.ntapes {
      return Err(Error::UnmatchingSizes(self.ntapes, read.len()));
    }
    self.resize_func_vec(state);
    if let Some(_) = self.function.get_mut(state).unwrap().insert(read.to_owned(), tr.clone()) {
      return Err(Error::Indeterminancy);
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

#[cfg(test)]
mod test {
  use std::collections::HashSet;

  use crate::turing_machine::{
    Error, TuringMachine,
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
    let mut tm = TuringMachine::new(0, 2, &HashSet::from([0, 1, 2])).unwrap();
    assert_eq!(tm.insert_transition(0, &vec!['a', 'a'], &tr1), Ok(()));
    assert_eq!(tm.insert_transition(10, &vec!['a', 'a'], &tr1), Ok(()));
    assert_eq!(tm.insert_transition(0, &vec!['b', 'a'], &tr1), Ok(()));
    assert_eq!(tm.insert_transition(0, &vec!['b', 'a'], &tr1), Err(Error::Indeterminancy));
    assert_eq!(tm.insert_transition(0, &vec!['a', 'b'], &tr2), Err(Error::UnmatchingSizes(2, 3)));
  }

  #[test]
  fn test_run_singletape() {
    let mut tm = TuringMachine::new(0, 1, &HashSet::from([0])).unwrap();
    let tr0 = Transition::new(&vec!['M'], &vec![Direction::Right], 1).unwrap();
    let tr1 = Transition::new(&vec!['M'], &vec![Direction::Right], 0).unwrap();
    tm.insert_transition(0, &vec!['a'], &tr0).expect("Unexpected error found adding transition");
    tm.insert_transition(1, &vec!['a'], &tr1).expect("Unexpected error found adding transition");
    assert_eq!(tm.run(""), Ok(true));
  }
}
