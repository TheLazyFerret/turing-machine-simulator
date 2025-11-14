//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Module representing a turing machine.

use std::collections::LinkedList;

pub struct Tape {
  n_half: LinkedList<char>,
  p_half: LinkedList<char>,
  head: isize,
}
