/*
    HC SVNT DRACONES
*/

// Emulate C u32 overflow behavior when subtracting from 0
pub fn subtract_from_zero(input: u32) -> u32 {
    (u32::MAX as u64 + 1u64 - input as u64) as u32
}

// Emulate C u32 inversion with ~ symbol
pub fn invert(input: u32) -> u32 {
    (u32::MAX as u64 - input as u64) as u32
}

// Emulate C u8 add with overflow
pub fn add_u8_overflow(a: u8, b: u8) -> u8 {
    (a as u16 + b as u16) as u8
}

// Emulate C raw pointer increment logic to carve up input at byte offsets
pub fn u32_pointer_increment(input: &Vec<u32>, offset_bytes: u32) -> u32 {
    // Start index to read at
    let base = (offset_bytes/4) as usize;
    match offset_bytes % 4 {
        0 => {
            // Offset points to the beginning of a u32
            // Return the whole u32
            input[base]
        }
        r => {
            // Offset points to the middle of a u32.
            // This will require adding (4-r) bytes of u32 #1 to r bytes of u32 #2

            // TODO: Research how raw u32 pointer increment even works!
            // Mask for 1st u32
            let a = (input[base] & (0xFFFFFFFF >> (r*8))) << (r*8);
            // Mask for 2nd u32
            let b = (input[base+1] & (0xFFFFFFFF << (r*8))) >> (r*8);

            a + b
        }
    }
}