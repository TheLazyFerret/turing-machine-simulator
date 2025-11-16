//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Single transition struct module.

use std::fmt::Display;

/// Struct representing a single transition in a single tape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transition {
  /// The current state.
  current_state: usize,
  /// The character readed. Only for printing purposes.
  read: char,
  /// The character that will be written.
  write: char,
  /// The next state.
  next_state: usize,
  /// The direction the head will take.
  direction: Direction,
}

impl Transition {
  /// Builds a new transition from parameters.
  pub fn new(t: (usize, char, char, usize, Direction)) -> Self {
    Transition {
      current_state: t.0,
      read: t.1,
      write: t.2,
      next_state: t.3,
      direction: t.4,
    }
  }

  /// Returns the internal value.
  pub fn get(&self) -> (usize, char, char, usize, Direction) {
    (
      self.current_state,
      self.read,
      self.write,
      self.next_state,
      self.direction,
    )
  }
}

impl Display for Transition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}, {}, {}, {}, {}]",
      self.current_state, self.read, self.write, self.next_state, self.direction
    )
  }
}

/// Simple enum representing the possible movements in each transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
  Left,
  Right,
  Stop,
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
  use super::Transition;

  #[test]
  fn test_display() {
    let x = Transition::new((1, 'a', 'b', 2, super::Direction::Left));
    let str = x.to_string();
    assert_eq!(str, "[1, a, b, 2, L]");
  }
}
