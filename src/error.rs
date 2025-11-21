//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Crate representing the Runtime errors.

use core::fmt;

/// Enum representing the possible rutime errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
  /// Multiple transitions with same (state, read).
  Indeterminancy,
  /// Reached the maximum number of allowed steps.
  MaxStepsReached,
  /// When adding a transition, the number of tapes doesn't coincide.
  UnmatchingSizes,
  /// The number of tapes must be 1 or more.
  TapeErrorCount,
  /// The vector inside the transitions doesn´t match
  TransitionSizeUnmatch,
  /// Unkown direction.
  UnkownDirection,
  /// Error parsing the toml file.
  ErrorParsing,
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
      | Error::UnmatchingSizes => {
        write!(f, "The number of tapes doesn't coincide with the transition.")
      },
      | Error::TapeErrorCount => {
        write!(f, "The number of tapes must be altelast one.")
      },
      | Error::TransitionSizeUnmatch => {
        write!(f, "The vec direction and write size doesn't match.")
      },
      | Error::UnkownDirection => {
        write!(f, "Found an unkown direction while parsing.")
      },
      | Error::ErrorParsing => {
        write!(f, "Found an error parsing the toml file.")
      },
    }
  }
}
