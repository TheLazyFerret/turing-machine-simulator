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
use std::{fs, io::Read};

fn main() -> Result<(), Error> {
  // Parsing phase.
  let mut file = fs::File::open("example.toml").unwrap();
  let mut file_str = String::new();
  file.read_to_string(&mut file_str).unwrap();
  let rtm = parser::parse_toml(&file_str);
  if rtm.is_err() {
    return Err(Error::ErrorParsing);
  }
  let tm = parser::parse(&rtm.unwrap())?;
  // Run phase.
  let test_string = "aa";
  if let Ok(x) = tm.run(test_string) {
    println!("\"{test_string}\" -> {x}");
  } else {
    println!("\"{test_string}\" -> error");
  }
  Ok(())
}
