//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main module for the parser.

use crate::turing_machine::{
  BLANK, BLANK_REP, TuringMachine,
  transition::{Direction, Transition},
};
use serde::Deserialize;
use std::collections::HashSet;
use crate::error::Error;

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
  read: String,
  write: String,
  direction: Vec<String>,
}

/// Tries to parse a toml to a RawTuringMachine.
pub fn parse_toml(raw: &str) -> Result<RawTuringMachine, Box<dyn std::error::Error>> {
  let rtm: RawTuringMachine = toml::from_str(raw)?;
  Ok(rtm)
}

/// Parse a TuringMachine from a RawTuringMachine
pub fn parse(rtm: &RawTuringMachine) -> Result<TuringMachine, Error> {
  let accept_set = HashSet::from_iter(rtm.accept.iter().cloned());
  let mut tm = TuringMachine::new(rtm.initial, rtm.ntapes, &accept_set)?;
  // For each transition.
  for tr in &rtm.transition {
    let mut read = Vec::from_iter(tr.read.chars()); // Characters readed.
    read = read.iter().map(|x| if *x == BLANK_REP { BLANK } else { *x }).collect(); // change the blank representation for blanks
    let mut write = Vec::from_iter(tr.write.chars()); // Characters writen.
    write = write.iter().map(|x| if *x == BLANK_REP { BLANK } else { *x }).collect(); // change the blank representation for blanks
    let direc = map_direction_vec(&tr.direction)?; // Direction of each tape.
    if let Ok(x) = Transition::new(&write, &direc, tr.next) {
      // Create and insert a new transition.
      tm.insert_transition(tr.from, &read, &x)?;
    } else {
      return Err(Error::TransitionSizeUnmatch);
    }
  }
  return Ok(tm);
}

/// From a String, convert into a Direction.
fn convert_direction(d: &str) -> Result<Direction, Error> {
  match d {
    | "Left" => Ok(Direction::Left),
    | "Right" => Ok(Direction::Right),
    | "Stop" => Ok(Direction::Stop),
    | _ => Err(Error::UnkownDirection),
  }
}

/// From a Vector of string, convert into a Vector of Direction.
fn map_direction_vec(dir: &[String]) -> Result<Vec<Direction>, Error> {
  let mut vec = Vec::new();
  for d in dir {
    let direction = convert_direction(d)?;
    vec.push(direction);
  }
  Ok(vec)
}
