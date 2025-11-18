//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Single transition struct module.

use std::fmt::Display;

/// Each transition of the turing machine.
#[derive(Clone, Debug)]
pub struct Transition {
  /// The node it comes from.
  from: usize,
  /// Transition info for each of the tapes.
  info: Vec<(char, char, Direction)>,
  /// The node it go.
  next: usize,
}

/// Public implementation.
impl Transition {
  /// Creates a new Transition.
  pub fn new(fr: usize, nx: usize) -> Self {
    Transition {
      from: fr,
      info: Vec::new(),
      next: nx,
    }
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
mod tests {}
