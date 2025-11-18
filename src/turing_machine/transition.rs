//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Single transition struct module.

use crate::turing_machine::print_sym;

/// Each transition of the turing machine.
#[derive(Clone, Debug)]
pub struct Transition {
  // Vector tuple of the written-next in each Tape.
  f: Vec<(char, usize)>,
  // The next state.
  next: usize,
}

/// Public implementation.
impl Transition {}

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
  fn test_new() {}

  #[test]
  fn test_getters() {}
}
