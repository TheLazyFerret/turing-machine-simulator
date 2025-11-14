//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Module representing a turing machine.

use core::fmt;

/// Represents a blank in a tape's cell.
const BLANK: char = '\0';

/// Struct representing a Single tape.
#[derive(Clone, Debug)]
pub struct Tape {
  n_half: Vec<char>,
  p_half: Vec<char>,
  head: isize,
}

/// Public implementation for Tape struct.
impl Tape {
  /// Returns a new Empty tape.
  pub fn new() -> Self {
    Tape::default()
  }

  /// Moves the head one cell to right. If the cell is not defined, create a blank one.
  pub fn move_right(&mut self) {}

  /// Moves the head one cell to left. If the cell is not defined, create a blank one.
  pub fn move_left(&mut self) {}

  /// Loads a string into the Tape.
  pub fn load_string(&mut self, f: &str) {
    self.clean();
    self.p_half = f.chars().collect();
  }

  /// Clean the current state of the object.
  fn clean(&mut self) {
    self.n_half.clear();
    self.p_half.clear();
    self.head = 0;
  }
}

impl Default for Tape {
  fn default() -> Self {
    Tape {
      n_half: Vec::new(),
      p_half: Vec::new(),
      head: 0,
    }
  }
}

impl fmt::Display for Tape {
  #[allow(unused_variables)]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use crate::tape::Tape;

  #[test]
  fn test_load_string() {
    let mut x = Tape::new();
    x.load_string("perro");
    assert_eq!(*x.p_half.as_slice().first().unwrap(), 'p');
    assert_eq!(*x.p_half.as_slice().last().unwrap(), 'o');
  }
}
