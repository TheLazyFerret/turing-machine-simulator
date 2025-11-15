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

/// How the blanks will be printed.
const BLANK_REP: char = 'β';

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

  /// Returns a new Tape with a string loaded.
  pub fn from_string(f: &str) -> Self {
    Tape {
      n_half: Vec::new(),
      p_half: f.chars().collect(),
      head: 0,
    }
  }

  /// Moves the head one cell to right. If the cell is not defined, create a blank one.
  /// It is only defined to create a new cell if the head is in the positive side.
  pub fn move_right(&mut self) {
    self.head += 1;
    let norm = self.normalized_head();
    if self.head >= 0 && self.p_half.get(norm).is_none() {
      self.p_half.push(BLANK);
    }
  }

  /// Moves the head one cell to left. If the cell is not defined, create a blank one.
  /// It is only defined to create a new cell if the head is in the negative half.
  pub fn move_left(&mut self) {
    self.head -= 1;
    let norm = self.normalized_head();
    if self.head < 0 && self.n_half.get(norm).is_none() {
      self.n_half.push(BLANK);
    }
  }

  /// Returns the value in the head position.
  pub fn read(&self) -> char {
    if self.head >= 0 {
      *self
        .p_half
        .get(self.normalized_head())
        .expect("weird error accesing for read")
    } else {
      *self
        .n_half
        .get(self.normalized_head())
        .expect("weird error accesing for read")
    }
  }

  /// Write a char in the head position.
  pub fn write(&mut self, f: char) {
    let pos = self.normalized_head();
    if self.head >= 0 {
      *self
        .p_half
        .get_mut(pos)
        .expect("weird error accesing for write") = f;
    } else {
      *self
        .p_half
        .get_mut(pos)
        .expect("weird error accesing for write") = f;
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
    if self.head >= 0 {
      self.head.cast_unsigned()
    } else {
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

/// Auxiliar function for Display: returns a visual representation of a symbol.
/// Basically prints β if the symbol is a blank, itself otherwise.
fn print_sym(x: char) -> char {
  if x == BLANK { BLANK_REP } else { x }
}

impl fmt::Display for Tape {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for x in self.n_half.iter().enumerate() {
      let relative_pos = (self.n_half.len().cast_signed() - x.0.cast_signed()).neg();
      if relative_pos == self.head {
        write!(f, "|[{}]", print_sym(*x.1))?;
      } else {
        write!(f, "|{}", print_sym(*x.1))?;
      }
    }
    for x in self.p_half.iter().enumerate() {
      let relative_pos = x.0.cast_signed();
      if relative_pos == self.head {
        write!(f, "|[{}]", print_sym(*x.1))?;
      } else {
        write!(f, "|{}", print_sym(*x.1))?;
      }
    }
    write!(f, "|")?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::tape::{BLANK, Tape};

  #[test]
  fn test_from_string() {
    let x = Tape::from_string("perro");
    assert_eq!(*x.p_half.as_slice().first().unwrap(), 'p');
    assert_eq!(*x.p_half.as_slice().last().unwrap(), 'o');
  }

  #[test]
  fn test_write_read_head() {
    let mut x = Tape::from_string("perro");
    x.head = 1;
    x.write('@');
    assert_eq!(x.read(), '@');
  }

  #[test]
  fn test_normalization() {
    let mut x = Tape::new();
    x.head = 4;
    assert_eq!(x.normalized_head(), 4);
    x.head = 0;
    assert_eq!(x.normalized_head(), 0);
    x.head = -2;
    assert_eq!(x.normalized_head(), 1);
  }

  #[test]
  fn test_move() {
    let mut x = Tape::from_string("p");
    x.move_left();
    assert_eq!(x.read(), BLANK);
    x.move_right();
    assert_eq!(x.read(), 'p');
    x.move_right();
    assert_eq!(x.read(), BLANK);
  }

  #[test]
  fn test_display() {
    let mut x = Tape::from_string("p");
    x.move_left();
    x.move_right();
    x.move_right();
    x.move_right();
    x.move_left();
    let rep = x.to_string();
    assert_eq!(rep, "|β|p|[β]|β|");
  }
}
