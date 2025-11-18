//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Single transition struct module.

/// Each transition of the turing machine.
#[derive(Clone, Debug)]
pub struct Transition {
  // Vector of the char to write in each tape.
  to_write: Vec<char>,
  // Vector of the Direction to move in each tape.
  to_move: Vec<Direction>,
  // The next state.
  next: usize,
}

impl Transition {
  /// Returns a new Transition.
  pub fn new(tw: &[char], tm:&[Direction], n: usize) -> Option<Self> {
    if tw.len() != tm.len() {
      return None
    }
    Some(Transition { to_write: tw.to_owned(), to_move: tm.to_owned(), next: n })
  }

  /// Returns the next state.
  pub fn next(&self) -> usize {
    self.next
  }

  /// Returns a slice of the characters to write.
  pub fn write_slice(&self) -> &[char] {
    &self.to_write
  }

  /// Returns a slice of the movements.
  pub fn move_slice(&self) -> &[Direction] {
    &self.to_move
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
    assert!(Transition::new(&vec!['a', 'b'], &vec![Direction::Left, Direction::Right], 2).is_some());
    assert!(Transition::new(&vec!['a', 'b', 'c'], &vec![Direction::Left, Direction::Right], 2).is_none()); 
  }
}
