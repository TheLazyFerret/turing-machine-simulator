//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Module representing a turing machine.

use core::fmt;
use std::ops::{Neg, Sub};

use crate::parser::{BLANK, BLANK_REP};
use crate::turing_machine::transition::Direction;

/// Struct representing a Single tape.
#[derive(Clone, Debug)]
pub struct Tape {
  /// When the head (relative position) is < 0, the Tape will work in this vector.
  n_half: Vec<char>,
  /// When the head (relative position) is >= 0, the Tape will work in this vector.
  p_half: Vec<char>,
  /// Relative position of the tape.
  head: isize,
}

impl Tape {
  /// Returns a new Empty tape.
  pub fn new() -> Self {
    Tape { n_half: Vec::new(), p_half: Vec::from(&['\0']), head: 0 }
  }

  /// Loads a string to the tape and reset the tape.
  pub fn load_string(&mut self, f: &str) {
    self.n_half.clear();
    // The tape must have atleast one cell defined to work.
    if f.is_empty() {
      self.p_half = "\0".chars().collect();
    } else {
      self.p_half = f.chars().collect();
    }
    self.head = 0;
  }

  /// Move the head to the specified direction.
  pub fn mov(&mut self, dir: Direction) {
    match dir {
      | Direction::Left => {
        self.move_left();
      },
      | Direction::Right => {
        self.move_right();
      },
      | Direction::Stop => {},
    }
  }

  /// Returns the value in the head position.
  pub fn read(&self) -> char {
    if self.head >= 0 {
      *self.p_half.get(self.absolute_pos()).expect("weird error accesing for read")
    } else {
      *self.n_half.get(self.absolute_pos()).expect("weird error accesing for read")
    }
  }

  /// Write a char in the head position.
  pub fn write(&mut self, f: char) {
    let pos = self.absolute_pos();
    if self.head >= 0 {
      *self.p_half.get_mut(pos).expect("weird error accesing for write") = f;
    } else {
      *self.n_half.get_mut(pos).expect("weird error accesing for write") = f;
    }
  }

  /// Returns the size of the Tape, being the the sum of both halfs.
  #[allow(unused)]
  pub fn size(&self) -> usize {
    self.n_half.len() + self.p_half.len()
  }

  /// Clean the current state of the object.
  #[allow(unused)]
  fn clean(&mut self) {
    self.n_half.clear();
    self.p_half.clear();
    self.head = 0;
  }

  /// Returns the absolute position of the head in the vectors.
  ///  - head >= 0 returns head
  ///  - head < 0 returns head - 1
  fn absolute_pos(&self) -> usize {
    if self.head >= 0 { self.head.cast_unsigned() } else { self.head.neg().cast_unsigned().sub(1) }
  }

  /// Moves the head one cell to right. If the cell is not defined, create a blank one.
  /// It is only defined to create a new cell if the head is in the positive side.
  fn move_right(&mut self) {
    self.head += 1;
    let norm = self.absolute_pos();
    if self.head >= 0 && self.p_half.get(norm).is_none() {
      self.p_half.push(BLANK);
    }
  }

  /// Moves the head one cell to left. If the cell is not defined, create a blank one.
  /// It is only defined to create a new cell if the head is in the negative half.
  fn move_left(&mut self) {
    self.head -= 1;
    let norm = self.absolute_pos();
    if self.head < 0 && self.n_half.get(norm).is_none() {
      self.n_half.push(BLANK);
    }
  }
}

impl fmt::Display for Tape {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for x in self.n_half.iter().enumerate() {
      // From the absolute position in the n_half, get the relative position.
      // absolute position = 2 relative position = -3
      //   relative pos = - (n_half.len() - absolute_pos)
      let relative_pos = (self.n_half.len().cast_signed() - x.0.cast_signed()).neg();
      if relative_pos == self.head {
        write!(f, "|[{}]", print_sym(*x.1))?;
      } else {
        write!(f, "|{}", print_sym(*x.1))?;
      }
    }
    for x in self.p_half.iter().enumerate() {
      // Absolute position coincide with relative position.
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

/// Auxiliar function for Display: returns a visual representation of a symbol.
/// Basically prints β if the symbol is a blank, itself otherwise.
fn print_sym(x: char) -> char {
  if x == BLANK { BLANK_REP } else { x }
}

#[cfg(test)]
mod tests {
  use crate::turing_machine::tape::{BLANK, Tape};

  #[test]
  fn test_from_string() {
    let mut x = Tape::new();
    x.load_string("perro");
    assert_eq!(*x.p_half.as_slice().first().unwrap(), 'p');
    assert_eq!(*x.p_half.as_slice().last().unwrap(), 'o');
  }

  #[test]
  fn test_write_read_head() {
    let mut x = Tape::new();
    x.load_string("perro");
    x.head = 1;
    x.write('@');
    assert_eq!(x.read(), '@');
  }

  #[test]
  fn test_normalization() {
    let mut x = Tape::new();
    x.head = 4;
    assert_eq!(x.absolute_pos(), 4);
    x.head = 0;
    assert_eq!(x.absolute_pos(), 0);
    x.head = -2;
    assert_eq!(x.absolute_pos(), 1);
  }

  #[test]
  fn test_move() {
    let mut x = Tape::new();
    x.load_string("p");
    x.move_left();
    assert_eq!(x.read(), BLANK);
    x.move_right();
    assert_eq!(x.read(), 'p');
    x.move_right();
    assert_eq!(x.read(), BLANK);
  }

  #[test]
  fn test_display() {
    let mut x = Tape::new();
    x.load_string("p");
    x.move_left();
    x.move_right();
    x.move_right();
    x.move_right();
    x.move_left();
    let rep = x.to_string();
    assert_eq!(rep, "|β|p|[β]|β|");
  }
}
