//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main file of the crate.

mod error;
mod parser;
mod turing_machine;

use crate::error::Error;
use crate::parser::Args;
use crate::turing_machine::TuringMachine;
use clap::Parser;
use std::fs;
use std::io::Read;

/// test the string with shell flag, only return true or false to the shell.
fn shell_run(string: &str, tm: &TuringMachine) -> Result<(), Error> {
  if let Ok(x) = tm.run(string) {
    if x == true {
      Ok(())
    }
    else {
      Err(Error::Fail(string.to_string()))
    }
  }
  else {
    Err(Error::Fail(string.to_string()))
  }
}

/// Test the string and print in the screen the result.
fn normal_run(string: &str, tm: &TuringMachine) {
  let result = tm.run(string);
  if result.is_err() {
    println!("\"{string}\" -> {}", result.err().unwrap().to_string());
  }
  else if result.unwrap() == true {
    println!("\"{string}\" -> true");
  }
  else {
    println!("\"{string}\" -> false");
  }
}

fn main() -> Result<(), Error> {
  let args: Args = Args::parse();
  // Open the Turing machine configuration file.
  let turing_file = fs::File::open(args.turing_path);
  if turing_file.is_err() {
    return Err(Error::ErrorOpenFile(turing_file.err().unwrap().to_string()));
  }
  // Read the Turing machine configuration.
  let mut turing_file_str = String::new();
  if let Err(x) = turing_file.unwrap().read_to_string(&mut turing_file_str) {
    return Err(Error::ErrorOpenFile(x.to_string()));
  }
  // Parse the toml file.
  let rtm = parser::parse_toml(&turing_file_str);
  if rtm.is_err() {
    return Err(Error::ErrorParsing(rtm.err().unwrap().to_string()));
  }
  // Parse the Turing configuration into a turing machine.
  let turing_machine = parser::parse(&rtm.unwrap())?;
  // Preparing to run.
  let string_to_test = args.string.clone();
  // Run.
  if args.shell > 0 {
    return shell_run(&string_to_test, &turing_machine);
  }
  normal_run(&string_to_test, &turing_machine);
  Ok(())
}
