mod table;
pub mod decrypt;
pub mod seeds;

// Original source: ar2.c:nibble_flip()
pub fn flip_nibble(input: u8) -> u8 {
    (input << 4) | (input >> 4)
}

// TODO: De-duplicate common operations
// Original source: common.c:swapbytes()
pub fn swap_bytes(input: u32) -> u32 {
    (input << 24) | ((input << 8) & 0xFF0000) | ((input >> 8) & 0xFF00) | (input >> 24)
}