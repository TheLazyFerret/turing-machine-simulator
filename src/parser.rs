//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main module for the parser.

use std::collections::HashSet;

use crate::turing_machine::{
  TuringMachine, TuringMachineError,
  transition::{Direction, Transition},
};
use serde::Deserialize;

/// Struct representing an raw, not checked turing machine.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RawTuringMachine {
  ntapes: usize,
  accept: Vec<usize>,
  initial: usize,
  transition: Vec<RawTransition>,
}

/// Struct representing a raw, not checked transition for the turing machine.
#[derive(Debug, Default, Clone, Deserialize)]
struct RawTransition {
  from: usize,
  next: usize,
  read: Vec<char>,
  write: Vec<char>,
  direction: Vec<String>,
}

/// Tries to parse a toml to a RawTuringMachine.
pub fn parse_toml(raw: &str) -> Result<RawTuringMachine, Box<dyn std::error::Error>> {
  let rtm: RawTuringMachine = toml::from_str(raw)?;
  Ok(rtm)
}

/// Parse a TuringMachine from a RawTuringMachine
pub fn parse(rtm: &RawTuringMachine) -> Result<TuringMachine, TuringMachineError> {
  let accept_set = HashSet::from_iter(rtm.accept.iter().cloned());
  let mut tm = TuringMachine::new(rtm.initial, rtm.ntapes, &accept_set)?;
  // For each transition.
  for tr in &rtm.transition {
    let read = Vec::from_iter(tr.read.iter().cloned()); // Characters readed.
    let write = Vec::from_iter(tr.write.iter().cloned()); // Characters writen.
    let direc = map_direction_vec(&tr.direction)?; // Direction of each tape.
    if let Ok(x) = Transition::new(&write, &direc, tr.next) { // Creates a new transition.
      tm.insert_transition(tr.from, &read, &x)?;
    } else {
      return Err(TuringMachineError::TransitionSizeUnmatch);
    }
  }
  return Ok(tm);
}

/// From a String, convert into a Direction.
fn convert_direction(d: &str) -> Result<Direction, TuringMachineError> {
  match d {
    | "Left" => Ok(Direction::Left),
    | "Right" => Ok(Direction::Right),
    | "Stop" => Ok(Direction::Stop),
    | _ => Err(TuringMachineError::UnkownDirection),
  }
}

/// From a Vector of string, convert into a Vector of Direction.
fn map_direction_vec(dir: &[String]) -> Result<Vec<Direction>, TuringMachineError> {
  let mut vec = Vec::new();
  for d in dir {
    let direction = convert_direction(d)?;
    vec.push(direction);
  }
  Ok(vec)
}
