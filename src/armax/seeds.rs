use crate::armax::table;
use crate::magic;

// Generate ActionReplay MAX seeds
pub fn generate() -> [u32; 32] {
    // Output seeds
    let mut output: [u32; 32] = [0u32; 32];
                                                // Here's what it looks like:
    let mut iv : [u8; 56] = create_iv();        // <- Static IV
    let mut rk  : [u8; 56] = [0u8; 56];         // <- Round key based on i
    let mut seed: [u8; 8] = [0u8; 8];           // <- Obfuscated seed data

    for i in 0..16 {
        // Update round key table
        rk = round_key(i, rk, &iv);

        // Pick seed table
        seed = pick_seeds(&rk);

        // Construct output u32 values from bytes
        output[i << 1] = (
            read_big_endian(seed[0], seed[2], seed[4], seed[6])
        );
        output[(i << 1) + 1] = (
            read_big_endian(seed[1], seed[3], seed[5], seed[7])
        );
    }

    // Swap u32 pairs around
    output = reverse_pairs(output);

    output
}

// Read big endian bytes into a u32
fn read_big_endian(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    ((b0 as u32) << 24) |
        ((b1 as u32) << 16) |
        ((b2 as u32) << 8) |
        (b3 as u32)
}

// Initialization vector
fn create_iv() -> [u8; 56] {
    let mut output = [0u8; 56];
    let mut offset: u8;
    for i in 0..56 {
        // Get generator data
        offset = table::G0[i] - 1;
        let gen: u32 = (table::GS[(offset >> 3) as usize] & table::G1[(offset & 7) as usize]) as u32;
        // Emulate C 32-bit overflow behavior with 64-bit
        let magic: u32 = magic::subtract_from_zero(gen);

        output[i] = (magic >> 31) as u8;
    }
    output
}

// Key for each round, picked from su
fn round_key(generator_index: usize, input: [u8; 56], sub_table: &[u8; 56]) -> [u8; 56] {
    // Pick next generator byte
    let gen = table::G2[generator_index];
    // Clone input to update and return
    let mut output = input.clone();

    // Pick values from substitution table
    let mut tmp: u8 = 0;
    for i in 0..56 {
        tmp = gen+ i;

        if i > 0x1B {
            if tmp > 0x37 {
                tmp -= 0x1C;
            }
        }
        else if tmp > 0x1B {
            tmp -= 0x1C;
        }

        output[i as usize] = sub_table[tmp as usize];
    }

    output
}

// Pick seeds from input substitution table & generator table data
fn pick_seeds(sub_table: &[u8; 56]) -> [u8; 8] {
    // Create zeroed buffer
    let mut output = [0u8; 8];

    // OR input table with generator table and substitution table
    let mut index: u8;
    for i in 0..48 {

        if sub_table[(table::G3[i]-1) as usize] == 0 {
            continue;
        }

        index = (((i * 0x2AAB) >> 16) - (i >> 0x1F)) as u8;

        output[index as usize] |= (table::G1[(i - (index * 6) as usize)] as usize >> 2) as u8;
    }
    output
}

// Reverse u32 pairs
fn reverse_pairs(input: [u32; 32]) -> [u32; 32] {
    // Clone input to modify and return
    let mut output: [u32; 32] = input.clone();

    // Reverse the DWORD pairs
    let mut end = 31;
    let mut range = (0..16).into_iter();
    while let (Some(x), Some(y)) = (range.next(), range.next()) {
        output.swap(x, end-1);
        output.swap(y, end);
        end -= 2;
    }

    output
}