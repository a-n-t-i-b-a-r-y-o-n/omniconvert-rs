use crate::armax::table;

// Generate ActionReplay MAX seeds
pub fn generate() -> [u32; 32] {
    // Output seeds
    let mut output: [u32; 32] = [0u32; 32];
                                                // Here's what it looks like:
    let mut arr0: [u8; 56] = fill_array0();     // <- Static key
    let mut arr1: [u8; 56] = [0u8; 56];         // <- Round key based on i
    let mut arr2: [u8; 8] = [0u8; 8];           // <- Obfuscated seed data

    for i in 0..16 {
        // Fill array 1
        arr1 = update_array1(i, arr1, &arr0);

        // Fill array 2
        arr2 = fill_array2(&arr1);

        // Construct output u32 values from bytes
        output[i << 1] = (
            read_big_endian(arr2[0], arr2[2], arr2[4], arr2[6])
        );
        output[(i << 1) + 1] = (
            read_big_endian(arr2[1], arr2[3], arr2[5], arr2[7])
        );
    }

    // Swap blocks
    output = swap_blocks(output);

    output
}

// Read big endian bytes into a u32
fn read_big_endian(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    ((b0 as u32) << 24) |
        ((b1 as u32) << 16) |
        ((b2 as u32) << 8) |
        (b3 as u32)
}

fn fill_array0() -> [u8; 56] {
    let mut output = [0u8; 56];
    let mut tmp: u8 = 0;
    for i in 0..56 {
        // Get generator data
        tmp = table::G0[i] - 1;
        let seed: u32 = (table::GS[(tmp >> 3) as usize] & table::G1[(tmp & 7) as usize]) as u32;
        // Emulate C 32-bit overflow behavior with 64-bit
        let magic: u32 = (u32::MAX as u64 + 1u64 - seed as u64) as u32;

        output[i] = (magic >> 31) as u8;
    }
    output
}

fn update_array1(generator_index: usize, input: [u8; 56], sub_table: &[u8; 56]) -> [u8; 56] {
    // Pick next generator byte
    let gen = table::G2[generator_index];
    // Clone input to update and return
    let mut output = input.clone();

    // Pick values from substitution table
    let mut tmp: u8 = 0;
    for j in 0..56 {
        tmp = gen+j;

        if j > 0x1B {
            if tmp > 0x37 {
                tmp -= 0x1C;
            }
        }
        else if tmp > 0x1B {
            tmp -= 0x1C;
        }

        output[j as usize] = sub_table[tmp as usize];
    }

    output
}

fn fill_array2(sub_table: &[u8; 56]) -> [u8; 8] {
    // Create zeroed buffer
    let mut output = [0u8; 8];

    // OR input with generator table mask and substitution table
    let mut tmp: u8 = 0;
    for j in 0..48 {

        if sub_table[(table::G3[j]-1) as usize] == 0 {
            continue;
        }

        tmp = (((j * 0x2AAB) >> 16) - (j >> 0x1F)) as u8;

        output[tmp as usize] |= (table::G1[(j - (tmp * 6) as usize)] as usize >> 2) as u8;
    }
    output
}

fn swap_blocks(input: [u32; 32]) -> [u32; 32] {
    let mut output: [u32; 32] = input.clone();

    let mut end = 31;

    let mut range = (0..16).into_iter();
    while let (Some(x), Some(y)) = (range.next(), range.next()) {
        output.swap(x, end-1);
        output.swap(y, end);
        end -= 2;
    }

    output
}