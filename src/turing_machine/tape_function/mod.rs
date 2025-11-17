//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Transition function module. Representing each Tape function.

pub mod transition;

use self::transition::Transition;
use std::collections::HashMap;

/// Struct representing the transition function for a single tape.
/// The each element in the Vector represents the current state,
/// and the Key of the Hashmap the symbol readed.
#[derive(Clone, Debug, Default)]
pub struct TapeFunction {
  func: Vec<HashMap<char, Transition>>,
}

/// Public implementation.
impl TapeFunction {
  /// Returns a new empty instance of TapeFunction.
  pub fn new() -> Self {
    TapeFunction::default()
  }
  /// Add a new transition.
  /// If already exists a transition for the couple (Current state, readed symbol), returns it.
  pub fn add(&mut self, tr: Transition) -> Option<Transition> {
    self.increase_size(tr.get().0);
    self
      .func
      .get_mut(tr.get().0)
      .unwrap()
      .insert(tr.get().1, tr)
  }

  /// Returns an Option if the transition exists.
  pub fn get(&self, state: usize, sym: char) -> Option<&Transition> {
    let x = self.func.get(state)?;
    x.get(&sym)
  }
}

/// Private implementation.
impl TapeFunction {
  /// Increase the size of the vector for holding more states.
  /// If the vector already can hold the state, no changes are made.
  fn increase_size(&mut self, state: usize) {
    if state >= self.func.len() {
      self.func.resize_with(state + 1, || HashMap::new());
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::turing_machine::tape_function::{
    TapeFunction,
    transition::{Direction, Transition},
  };

  #[test]
  fn test_add() {
    let mut f = TapeFunction::new();
    let tr1 = Transition::new((0, 'a', 'b', 1, Direction::Right));
    let tr2 = Transition::new((0, 'b', 'c', 2, Direction::Left));
    let tr3 = Transition::new((4, 'a', '2', 0, Direction::Stop));
    assert_eq!(f.add(tr1), None);
    assert_eq!(f.add(tr2), None);
    assert_eq!(f.add(tr3), None);
    assert_eq!(f.add(tr1), Some(tr1));
  }

  #[test]
  fn test_get() {
    let mut f = TapeFunction::new();
    let tr1 = Transition::new((0, 'a', 'b', 1, Direction::Right));
    let tr2 = Transition::new((0, 'b', 'c', 2, Direction::Left));
    assert_eq!(f.add(tr1), None);
    assert_eq!(f.get(tr1.get().0, tr1.get().1), Some(&tr1));
    assert_eq!(f.get(tr2.get().0, tr2.get().1), None);
  }
}
