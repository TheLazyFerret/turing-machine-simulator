//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
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
  write: char,
  next_state: usize,
  direction: Direction,
}

impl Transition {}

impl Display for Transition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
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
