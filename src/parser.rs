//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main module for the parser.


use std::fmt::Display;

use crate::turing_machine::{TuringMachine, TuringMachineError};
use serde::Deserialize;

/// Struct representing an raw, not checked turing machine.
#[derive(Debug, Default, Clone, Deserialize)]
struct RawTuringMachine {
  ntapes: usize,
  accept: Vec<usize>,
  transition: Vec<RawTransition>
}

/// Struct representing a raw, not checked transition for the turing machine.
#[derive(Debug, Default, Clone, Deserialize)]
struct RawTransition {
  from: usize,
  next: usize,
  read: Vec<char>,
  write: Vec<char>,
  direction: Vec<String>
}

/// Tries to parse a toml to a RawTuringMachine.
pub fn parse_toml(raw: &str) -> Result<RawTuringMachine, Box<dyn std::error::Error>> {
  let rtm: RawTuringMachine = toml::from_str(raw)?;
  Ok(rtm)
}