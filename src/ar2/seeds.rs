use crate::ar2::swap_bytes;

const DEFAULT_SEED_KEY: u32 = 0x04030209;

// Generate default, beginning AR2 seeds
pub fn generate() -> [u8; 4] {
    regenerate(DEFAULT_SEED_KEY)
}

// Regenerate AR2 seeds with a new key
pub fn regenerate(key: u32) -> [u8; 4] {
    let seed = swap_bytes(key);
    let output =
    [
         (seed & 0x000000FF) as u8,
        ((seed & 0x0000FF00) >> 8) as u8,
        ((seed & 0x00FF0000) >> 16) as u8,
        ((seed & 0xFF000000) >> 24) as u8,
    ];
    output
}