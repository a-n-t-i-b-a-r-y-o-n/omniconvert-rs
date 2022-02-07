use regex::{Regex};

pub mod decrypt;
pub mod seeds;
mod table;
pub mod cheat;

#[derive(Clone, PartialEq)]
pub enum VerifierMode {
    Manual,
    Auto,
}

const ARMAX_FORMAT: &str = r#"[\w\d]{4}-[\w\d]{4}-[\w\d]{5}"#;

// Attempt to recognize if this string is an ARMAX code or not
pub fn recognize(input: &str) -> bool {
    Regex::new(ARMAX_FORMAT)
        .unwrap()
        .is_match(input.trim())
}

// TODO: De-duplicate common operations
// Original sources: armax.c:rotate_left() & armax.c:rotate_right()
// Rotate bytes left
pub fn rotate_left(input: u32, rot: u8) -> u32 { (input << rot) | (input >> (32 - rot)) }
// Rotate bytes right
pub fn rotate_right(input: u32, rot: u8) -> u32 { (input >> rot) | (input << (32 - rot)) }

// TODO: De-duplicate common operations
// Original source: armax.c:byteswap()
// Shuffle bytes around
pub fn swap_bytes(input: u32) -> u32 { (input << 24) | ((input << 8) & 0x00FF0000) | ((input >> 8) & 0x0000FF00) | (input >> 24) }