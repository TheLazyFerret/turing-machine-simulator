//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main module for the parser.

/// Represents a blank in a tape's cell.
pub const BLANK: char = '\0';
/// How the blanks will be printed.
pub const BLANK_REP: char = 'β';

use crate::error::Error;
use crate::turing_machine::TuringMachine;
use crate::turing_machine::transition::{Direction, Transition};
use clap::Parser;
use serde::Deserialize;
use std::collections::HashSet;

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
    // Characters readed.
    let read: Vec<char> = Vec::from_iter(tr.read.chars())
      .iter()
      .map(|x| if *x == BLANK_REP { BLANK } else { *x })
      .collect();
    // Characters writen.
    let write: Vec<char> = Vec::from_iter(tr.write.chars())
      .iter()
      .map(|x| if *x == BLANK_REP { BLANK } else { *x })
      .collect();
    // Direction of each tape.
    let direc = map_direction_vec(&tr.direction)?;
    // Create and insert the new transition.
    let transition = Transition::new(&write, &direc, tr.next)?;
    tm.insert_transition(tr.from, &read, &transition)?;
  }
  Ok(tm) // Return the TuringMachine.
}

/// From a String, convert into a Direction.
fn convert_direction(d: &str) -> Result<Direction, Error> {
  match d {
    | "Left" => Ok(Direction::Left),
    | "L" => Ok(Direction::Left),
    | "Right" => Ok(Direction::Right),
    | "R" => Ok(Direction::Right),
    | "Stop" => Ok(Direction::Stop),
    | "S" => Ok(Direction::Stop),
    | _ => Err(Error::UnkownDirection(d.to_owned())),
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

#[derive(Parser, Debug, Default)]
#[command(version, about)]
pub struct Args {
  /// String to be tested on the Turing machine.
  pub string: String,
  /// Path where the Turing machine configuration is located.
  #[arg(short, long)]
  pub turing_path: String,
  // /// Path where the tape steps will be saved.
  //#[arg(short, long)]
  //pub dump: Option<String>,
  // /// Shell mode (no output, only returns 0 if accept or 1 otherwise)
  #[arg(short, long, action = clap::ArgAction::Count)]
  pub shell: u8,
}
