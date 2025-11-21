//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Crate representing the Runtime errors.

use core::fmt;

/// Enum representing the possible rutime errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
  /// Multiple transitions with same (state, read).
  Indeterminancy,
  /// Reached the maximum number of allowed steps.
  MaxStepsReached,
  /// When adding a transition, the number of tapes doesn't coincide.
  UnmatchingSizes(usize, usize),
  /// The number of tapes must be 1 or more.
  TapeErrorCount,
  /// The vector inside the transitions doesn´t match
  TransitionSizeUnmatch(usize, usize),
  /// Unkown direction.
  UnkownDirection(String),
  /// Error parsing the toml file.
  ErrorParsing(String),
  /// Error opening the toml file.
  ErrorOpenFile(String),
  /// Fail the test (used with shell flag)
  Fail(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      | Error::Indeterminancy => {
        write!(f, "Multiple transitions for the same pair state-readed.")
      },
      | Error::MaxStepsReached => {
        write!(f, "Run stopped, reached the maximum ammount of steps.")
      },
      | Error::UnmatchingSizes(x, y) => {
        write!(f, "The number of tapes doesn't coincide with the transition ({x}, {y}).")
      },
      | Error::TapeErrorCount => {
        write!(f, "The number of tapes must be atleast one.")
      },
      | Error::TransitionSizeUnmatch(x, y) => {
        write!(f, "The direction and write vectors size doesn't match ({x}, {y}).")
      },
      | Error::UnkownDirection(x) => {
        write!(f, "Found an unkown direction while parsing: {x}.")
      },
      | Error::ErrorParsing(x) => {
        write!(f, "Found an error parsing the toml file: {x}")
      },
      | Error::ErrorOpenFile(x) => {
        write!(f, "Couldn't open the toml file: {x}")
      },
      | Error::Fail(x) => {
        write!(f, "Failed the test: {x}")
      },
    }
  }
}
