use crate::magic;
use crate::ar2::flip_nibble;
use crate::ar2::seeds;
use crate::ar2::table;

// Decrypt a list of AR2 codes stored as address/value octet pairs
pub fn decrypt_cheat(input: Vec<u32>, seeds: &[u8; 4]) -> Vec<u32> {
    // Clone given AR2 seeds to manipulate
    let mut seeds = seeds.clone();

    // Clone given input to manipulate and return
    let mut output = input.clone();

    // Get length of input for use in iterations
    let mut size = output.len();

    // TODO: Replace custom pair iterators with (x..y).step_by(z)
    // Decrypt AR2 codes
    for mut i in (0..size).step_by(2) {
        // Decrypt address and value octets
        output[i] = decrypt_code(output[i], seeds[0], seeds[1]);
        output[i+1] = decrypt_code(output[i+1], seeds[2], seeds[3]);

        // TODO: What does the 0xDEADFACE string represent for AR2?
        // Check if address is special address
        if output[i] == 0xDEADFACE {
            // Generate new seeds based off the value
            seeds = seeds::regenerate(output[i+1]);

            if (i+2) < size {
                for j in (i+2)..size {
                    output[j-2] = output[j];
                }
            }
            i -= 2;
            size -= 2;
        }
    }

    output
}

// Decrypt a single AR2 octet stored as u32
pub fn decrypt_code(input: u32, in_ctrl: u8, seed: u8) -> u32 {

    // Handle control value (original source: type)
    let mut ctrl = in_ctrl;
    if ctrl == 7 {
        if seed & 1 > 0 {
            ctrl = 1;
        }
        else {
            return magic::invert(input);
        }
    }

    // Break input up into byte array
    let mut output: [u8; 4] = [
        (input & 0x000000FF) as u8,
        ((input & 0x0000FF00) >> 8) as u8,
        ((input & 0x00FF0000) >> 16) as u8,
        ((input & 0xFF000000) >> 24) as u8,

    ];

    println!("{:?}", output);

    match ctrl {
        0 => {
            output[3] ^= table::T0[seed as usize];
            output[2] ^= table::T1[seed as usize];
            output[1] ^= table::T2[seed as usize];
            output[0] ^= table::T3[seed as usize];
        },
        1 => {
            output[3] = flip_nibble(output[3]) ^ table::T0[seed as usize];
            output[2] = flip_nibble(output[2]) ^ table::T2[seed as usize];
            output[1] = flip_nibble(output[1]) ^ table::T3[seed as usize];
            output[0] = flip_nibble(output[0]) ^ table::T1[seed as usize];
        },
        2 => {
            output[3] = magic::add_u8_overflow(output[3], table::T0[seed as usize]);
            output[2] = magic::add_u8_overflow(output[2], table::T1[seed as usize]);
            output[1] = magic::add_u8_overflow(output[1], table::T2[seed as usize]);
            output[0] = magic::add_u8_overflow(output[0], table::T3[seed as usize]);
        },
        3 => {
            output[3] -= table::T3[seed as usize];
            output[2] -= table::T2[seed as usize];
            output[1] -= table::T1[seed as usize];
            output[0] -= table::T0[seed as usize];
        },
        4 => {
            output[3] = magic::add_u8_overflow(output[3] ^ table::T0[seed as usize], table::T0[seed as usize]);
            output[2] = magic::add_u8_overflow(output[2] ^ table::T3[seed as usize], table::T3[seed as usize]);
            output[1] = magic::add_u8_overflow(output[1] ^ table::T1[seed as usize], table::T1[seed as usize]);
            output[0] = magic::add_u8_overflow(output[0] ^ table::T2[seed as usize], table::T2[seed as usize]);
        },
        5 => {
            output[3] = (output[3] - table::T1[seed as usize]) ^ table::T0[seed as usize];
            output[2] = (output[2] - table::T2[seed as usize]) ^ table::T1[seed as usize];
            output[1] = (output[1] - table::T3[seed as usize]) ^ table::T2[seed as usize];
            output[0] = (output[0] - table::T0[seed as usize]) ^ table::T3[seed as usize];
        },
        6 => {
            output[3] += table::T0[seed as usize];
            output[2] -= table::T1[((seed + 1) & 31) as usize];
            output[1] += table::T2[((seed + 2) & 31) as usize];
            output[0] -= table::T3[((seed + 3) & 31) as usize];
        }
        c => {
            // TODO: Handle AR2 parsing errors more gracefully
            panic!("[!] Received unrecognized control value in ar2/decrypt: {}", c);
        }
    }

    // Reconstruct output into u32
    (((output[3] as u32) << 24) as u32 +
        ((output[2] as u32) << 16) as u32 +
        ((output[1] as u32) << 8) as u32 +
        output[0] as u32
    )
}