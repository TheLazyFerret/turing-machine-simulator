//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Turing machine struct module.

#[allow(dead_code)]
mod tape;
#[allow(dead_code)]
mod transition;

use crate::turing_machine::tape::Tape;
use crate::turing_machine::transition::Transition;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct TuringMachine {
  tapes: Vec<Tape>,
  initial: usize,
  transition: Vec<HashMap<char, Transition>>,
  acceptance: HashSet<usize>,
}
