/*
    C language quirk emulation
 */

// Emulate C u32 overflow behavior when subtracting from 0
pub fn subtract_from_zero(input: u32) -> u32 {
    (u32::MAX as u64 + 1u64 - input as u64) as u32
}

// Emulate C raw pointer increment logic to carve up input
pub fn emulate_pointer_increment(input: &Vec<u32>, offset: u32) -> u32 {

    let index = (offset/4) as usize;
    match offset % 4 {
        0 => {
            // Offset points to the beginning of a u32
            input[index]
        }
        r => {
            // Offset points to the middle of a u32.
            // This will require adding (4-r) bytes of u32 #1 to r bytes of u32 #2
            // All the (r*2) operations are to account for the hex notation

            // Mask for 1st u32
            let a = (input[index] & (0xFFFFFFFF >> (r*2))) << (r*2);
            // Mask for 2nd u32
            let b = (input[index+1] & (0xFFFFFFFF << (r*2))) >> (r*2);

            a + b
        }
    }
}