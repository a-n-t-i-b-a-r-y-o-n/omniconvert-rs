pub mod decrypt;

use std::iter::{Enumerate, FilterMap};
use std::slice::Iter;

#[derive(Clone, PartialEq)]
pub enum VerifierMode {
    Manual,
    Auto,
}

// Attempt to recognize if this string is an ARMAX code or not
pub fn is_armax_code(input: &str) -> bool {
    input.chars().all(|c| { c.is_alphanumeric() || c == '-' }) &&
        input.chars().nth(4) == Some('-') && input.chars().nth(9) == Some('-')
}