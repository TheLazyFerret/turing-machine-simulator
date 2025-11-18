//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Single transition struct module.

/// Each transition of the turing machine.
#[derive(Clone, Debug)]
pub struct Transition {
  // Vector tuple of the write-direction of each Tape.
  oper: Vec<(char, Direction)>,
  // The next state.
  next: usize,
}

/// Public implementation.
impl Transition {
  /// Returns a new Transition.
  pub fn new(op: &[(char, Direction)], n: usize) -> Self {
    Transition {
      next: n,
      oper: op.to_vec(),
    }
  }

  /// Returns the next state.
  pub fn next(&self) -> usize {
    self.next
  }

  /// Returns the oper Vec as a slice.
  pub fn oper(&self) -> &[(char, Direction)] {
    self.oper.as_slice()
  }
}

/// Simple enum representing the possible movements in each transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
  Left,
  Right,
  Stop,
}

#[cfg(test)]
mod tests {
  use crate::turing_machine::transition::{Direction, Transition};

  #[test]
  fn test_transition() {
    let opers = vec![('a', Direction::Left), ('b', Direction::Right)];
    let tr = Transition::new(opers.as_slice(), 2);
    assert_eq!(tr.next(), 2);
    assert_eq!(tr.oper(), opers.as_slice());
  }
}
