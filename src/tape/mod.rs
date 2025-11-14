//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Module representing a turing machine.



use core::fmt;
use std::collections::LinkedList;

/// Represents a blank in a tape's cell.
const BLANK: char = '\0'; 

/// Struct representing a Single tape.
#[derive(Clone, Debug)]

pub struct Tape {
  n_half: LinkedList<char>,
  p_half: LinkedList<char>,
  head: isize,
}

/// Public implementation for Tape struct.
impl Tape {
  /// Returns a new Empty tape.
  pub fn new() -> Self {
    Tape::default()
  }
}

impl Default for Tape {
  fn default() -> Self {
    let mut x = Tape {
      n_half: LinkedList::new(),
      p_half: LinkedList::new(),
      head: 0
    };
    // The turing machine must have atleast one defined cell.
    x.p_half.push_back(BLANK);
    return x;
  }
}

impl fmt::Display for Tape {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}