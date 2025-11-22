//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Crate representing the Runtime errors.

/// Enum representing the possible rutime errors.

use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
  #[error("Multiple transitions for the same pair state-readed.")]
  Indeterminancy,
  #[error("Run stopped, reached the maximum ammount of steps.")]
  MaxStepsReached,
  #[error("The number of tapes doesn't coincide with the transition ({0}, {1}).)")]
  UnmatchingSizes(usize, usize),
  #[error("The number of tapes must be atleast one.")]
  TapeErrorCount,
  #[error("The direction and write vectors size doesn't match ({0}, {1}).")]
  TransitionSizeUnmatch(usize, usize),
  #[error("Found an unkown direction while parsing: {0}.")]
  UnkownDirection(String),
  #[error("Found an error parsing the toml file: {0}.")]
  ErrorParsing(String),
  #[error("Couldn't open the toml file: {0}.")]
  ErrorOpenFile(String),
  #[error("Failed the test: {0}.")]
  Fail(String),
}