//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Single transition struct module.

use std::fmt::Display;

use crate::turing_machine::print_sym;

/// Each transition of the turing machine.
#[derive(Clone, Debug)]
pub struct Transition {
  /// The state it comes from.
  from: usize,
  /// Transition info for each of the tapes.
  info: Vec<(char, char, Direction)>,
  /// The state it goes.
  next: usize,
}

/// Public implementation.
impl Transition {
  /// Creates a new Transition.
  pub fn new(fr: usize, nx: usize, infv: &[(char, char, Direction)]) -> Self {
    Transition {
      from: fr,
      info: infv.to_vec(),
      next: nx,
    }
  }

  /// Return a tuple of the from and next nodes.
  pub fn state(&self) -> (usize, usize) {
    (self.from, self.next)
  }

  /// Returns the info of each Tape.
  pub fn info(&self) -> &[(char, char, Direction)] {
    self.info.as_slice()
  }
}

impl Display for Transition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{} ", self.state().0)?;
    for tape in self.info() {
      write!(
        f,
        "({} {} {}) ",
        print_sym(tape.0),
        print_sym(tape.1),
        tape.2
      )?;
    }
    write!(f, "{}]", self.state().1)?;
    Ok(())
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
  use crate::turing_machine::transition::{Direction, Transition};

  #[test]
  fn test_new() {
    let vinf = vec![('a', 'a', Direction::Left), ('\0', '\0', Direction::Right)];
    Transition::new(0, 1, &vinf);
  }

  #[test]
  fn test_getters() {
    let vinf = vec![('a', 'a', Direction::Left), ('\0', '\0', Direction::Right)];
    let tr = Transition::new(0, 1, &vinf);
    assert_eq!(tr.state(), (0, 1));
    assert_eq!(tr.info(), vinf.as_slice());
  }

  #[test]
  fn test_display() {
    let vinf = vec![('a', 'a', Direction::Left), ('\0', '\0', Direction::Right)];
    let tr = Transition::new(0, 1, &vinf);
    assert_eq!(tr.to_string(), "[0 (a a L) (β β R) 1]");
  }
    
}
