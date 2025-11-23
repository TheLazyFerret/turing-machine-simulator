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
use clap::Parser;
use std::fs::File;
use std::io::Read;

/// Print the result of the simulator run.
fn print_result(string: &str, result: Result<bool, Error>) {
  if result.is_err() {
    println!("\"{string}\" -> {}", result.err().unwrap().to_string());
  } else if result.unwrap() == true {
    println!("\"{string}\" -> true");
  } else {
    println!("\"{string}\" -> false");
  }
}

/// If using the flag -s|--shell, return to the shell an error if not accepted.
fn return_shell(string: &str, result: Result<bool, Error>) -> Result<(), Error> {
  if result.is_err() {
    return Err(Error::Fail(string.to_string()));
  } else if result.is_ok() && result.unwrap() == false {
    return Err(Error::Fail(string.to_string()));
  } else {
    return Ok(());
  }
}

/// Auxiliar function to get the dump file (if exists).
fn get_dump_file(option: Option<String>) -> Result<Option<File>, Error> {
  if option.is_some() {
    let path = option.unwrap();
    let result = File::create(path);
    if result.is_err() {
      Err(Error::ErrorOpenFile(result.err().unwrap().to_string()))
    } else {
      Ok(Some(result.unwrap()))
    }
  } else {
    Ok(None)
  }
}

fn main() -> Result<(), Error> {
  let args: Args = Args::parse();
  // Open the Turing machine configuration file.
  let turing_file = File::open(args.turing_path);
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
  // Get the string to test.
  let test_string = args.string.clone();
  // Get the dump file.
  let dump_file = get_dump_file(args.dump)?;
  // Run.
  let result = turing_machine.run(&test_string, dump_file);
  if args.shell > 0 {
    return return_shell(&test_string, result);
  } else {
    print_result(&test_string, result);
    Ok(())
  }
}
