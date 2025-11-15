//! Author: TheLazyFerret (<https://github.com/TheLazyFerret>)
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Transition struct module.

use std::fmt::Display;

/// Simple enum representing the possible movements in each transition.
#[derive(Clone, Copy, Debug)]
pub enum Direction {
  Left,
  Right,
  Stop,
}

/// Struct representing each one of the transitions in the Turing machine.
#[derive(Clone, Copy, Debug)]
pub struct Transition {
  /// The character readed. Only for printing purposes.
  read: char,
  /// The character that will be written.
  write: char,
  /// The next state.
  state: usize,
  /// The direction the head will take.
  direction: Direction,
}

impl Transition {
  /// Builds a new transition from parameters.
  pub fn new(rd: char, wr: char, ns: usize, dir: Direction) -> Self {
    Transition {
      read: rd,
      write: wr,
      state: ns,
      direction: dir,
    }
  }

  /// Returns the internal value, Without the read.
  pub fn get(&self) -> (char, usize, Direction) {
    (self.write, self.state, self.direction)
  }
}

impl Display for Transition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}, {}, {}, {}]",
      self.read, self.write, self.state, self.direction
    )
  }
}

impl Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      | Direction::Left => write!(f, "L")?,
      | Direction::Right => write!(f, "R")?,
      | Direction::Stop => write!(f, "S")?,
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::turing_machine::transition::Transition;

  #[test]
  fn test_display() {
    let x = Transition::new('a', 'b', 2, super::Direction::Left);
    let str = x.to_string();
    assert_eq!(str, "[a, b, 2, L]");
  }
}
