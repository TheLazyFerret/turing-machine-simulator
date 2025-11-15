//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Module representing a turing machine.

use core::fmt;
use std::ops::{Neg, Sub};

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

  /// Returns the value in the head position.
  pub fn read(&self) -> char {
    if self.head.is_positive() {
      *self.p_half.get(self.normalized_head()).expect("weird error accesing for read")
    }
    else {
      *self.n_half.get(self.normalized_head()).expect("weird error accesing for read")
    }
  }

  /// Write a char in the head position.
  pub fn write(&mut self, f: char) {
    let pos = self.normalized_head();
    if self.head.is_positive() {
      *self.p_half.get_mut(pos).expect("weird error accesing for write") = f;
    }
    else {
      *self.p_half.get_mut(pos).expect("weird error accesing for write") = f;
    }
  }

  /// Loads a string into the Tape.
  pub fn load_string(&mut self, f: &str) {
    self.clean();
    self.p_half = f.chars().collect();
  }

}

/// Private implementation for Tape struct.
impl Tape {
  /// Clean the current state of the object.
  fn clean(&mut self) {
    self.n_half.clear();
    self.p_half.clear();
    self.head = 0;
  }

  /// Normalize the value of the head to peak in one of the vectors.
  ///  - head >= 0 returns head
  ///  - head < 0 returns head - 1
  fn normalized_head(&self) -> usize {
    if self.head.is_positive() {
      self.head.cast_unsigned()
    }
    else {
      self.head.neg().cast_unsigned().sub(1)
    }
  }
}

impl Default for Tape {
  /// Default construction for Tape.
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
