use crate::armax::table;
use crate::armax::{rotate_left, rotate_right, swap_bytes};
use crate::ar2;
use crate::cheat::Cheat;
use crate::magic;

// TODO: Translate alpha_to_octets() from alphatobin() less literally
// Decode ARMAX lines into pairs of address/value octets
pub fn alpha_to_octets(input: Vec<&str>) -> Option<Vec<(u32, u32)>> {
    // Output octets
    let mut output: Vec<(u32, u32)> = vec!();

    // Replaces provided size parameter in original source
    let mut lines_left = input.len();

    // Built octet count
    let mut octet_count = 0;

    let alphabet = vec!(
        '0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        'G', 'H', 'J', 'K', 'M', 'N', 'P', 'Q',
        'R', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    );

    // Read input lines
    while lines_left > 0 {

        // TODO: Swap panics for printlns
        // TODO: De-duplicate octet building

        // Build 1st octet (code address)
        let mut octet1: u32 = 0;
        for index in 0..7 {
            // Get input char at this index
            match input[octet_count>>1].chars().nth(index) {
                None => {
                    // TODO: Handle parsing errors gracefully
                    panic!("[!] Unable to get character at {} from input of length {}", index, input[octet_count>>1].len());
                }
                Some(char_in) => {
                    // Get index of input char in cipher alphabet
                    match alphabet.iter().position(|&char_alpha| char_alpha == char_in) {
                        None => {
                            // TODO: Handle parsing error gracefully
                            panic!("[!] Received non-alphanumeric character \"{}\" in ARMAX code at index {}", char_in, index);
                        }
                        Some(match_index) => {
                            // OR octet w/ index of input char in alphabet string,
                            // shifted by permutation of input char index
                            if index < 6 {
                                // Indexes 0-5
                                octet1 |= (match_index as u32) << (((5-index)*5)+2);
                            }
                            else {
                                // Index 6
                                octet1 |= (match_index as u32) >> 3;
                            }
                        }
                    }
                }
            }
        }
        // Increment built octet count
        octet_count+=1;

        // Build 2nd octet (code value)
        let mut octet2: u32 = 0;
        for index in 0..7 {
            // Get input char at this index
            match input[octet_count>>1].chars().nth(index+6) {
                None => {
                    // TODO: Handle parsing errors gracefully
                    panic!("[!] Unable to get character at {} from input of length {}", index+6, input[octet_count>>1].len());
                }
                Some(char_in) => {
                    // Get index of input char in cipher alphabet
                    match alphabet.iter().position(|&char_alpha| char_alpha == char_in) {
                        None => {
                            // TODO: Handle parsing error gracefully
                            panic!("[!] Received non-alphanumeric character \"{}\" in ARMAX code at index {}", char_in, index+6);
                        }
                        Some(match_index) => {
                            // OR octet w/ index of input char in alphabet string,
                            // shifted by permutation of input char index
                            if index < 6 {
                                // Indexes 6-11
                                octet2 |= (match_index as u32) << (((5-index)*5)+4);
                            }
                            else {
                                // Index 12
                                octet2 |= (match_index as u32) >> 1;
                            }
                        }
                    }
                }
            }
        }
        // Increment built octet count
        octet_count+=1;

        // Calculate parity
        let mut parity: u8 = 0;
        for i in 0..64 {
            if i < 32 {
                parity ^= (octet1 >> (i-(0<<5))) as u8;
            }
            else {
                parity ^= (octet2 >> (i-(1<<5))) as u8;
            }
        }

        // Verify parity bit and potentially add to output list
        match input[(octet_count-2)>>1].chars().nth(12) {
            None => {
                // TODO: Handle parsing errors gracefully
                panic!("[!] Unable to get character at 12 from input of length {}", input[octet_count>>1].len());
            }
            Some(char_in) => {
                // Get index of input char in cipher alphabet
                match alphabet.iter().position(|&char_alpha| char_alpha == char_in) {
                    None => {
                        // TODO: Handle parsing error gracefully
                        panic!("[!] Received non-alphanumeric character \"{}\" in ARMAX code at index {}", char_in, 12);
                    }
                    Some(match_index) => {
                        if parity&1 != ((match_index as u8)&1) {
                            panic!("[!] Parity bit validation failed! Octets: {:08X} / {:08X}", octet1, octet2);
                        }
                        else {
                            // Parity check passed! Add octets to output list
                            output.push((octet1, octet2));
                        }
                    }
                }
            }
        }

        // Decrement input line counter
        lines_left-=1;
    }

    Some(output)
}

// Decrypt a pair of ARMAX octets
pub fn decrypt_pair(input: (u32, u32), seeds: &[u32; 32]) -> (u32, u32) {
    // Byte swap 1/2    |   armax.c:getcode()
    let mut addr = swap_bytes(input.0);
    let mut val = swap_bytes(input.1);

    // Unscramble 1/2   |   armax.c:unscramble1()
    let unscrambled = unscramble_1(addr, val);
    addr = unscrambled.0;
    val = unscrambled.1;

    // Apply seeds
    for i in (0..32).step_by(4) {
        let mut tmp = rotate_right(val, 4) ^ seeds[i];
        let mut tmp2 = val ^ seeds[i+1];
        addr ^= octet_mask(tmp, tmp2);

        tmp = rotate_right(addr,4) ^ seeds[i+2];
        tmp2 = addr ^ seeds[i+3];
        val ^= octet_mask(tmp, tmp2);
    }

    // Unscramble 2/2   |   armax.c:unscramble2()
    let unscrambled = unscramble_2(addr, val);
    addr = unscrambled.0;
    val = unscrambled.1;

    // Byte swap 2/2    |   armax.c:setcode()
    addr = swap_bytes(addr);
    val = swap_bytes(val);

    // Swap address and value
    let tmp = addr;
    addr = val;
    val = tmp;

    (addr, val)
}

// Mask XOR'd to address/value octets
pub fn octet_mask(i1: u32, i2: u32) -> u32 {
    table::T6[(i1&63) as usize]             ^  table::T4[((i1>>8)&63) as usize]  ^
        table::T2[((i1>>16)&63) as usize]   ^  table::T0[((i1>>24)&63) as usize] ^
        table::T7[(i2&63) as usize]         ^  table::T5[((i2>>8)&63) as usize]  ^
        table::T3[((i2>>16)&63) as usize]   ^  table::T1[((i2>>24)&63) as usize]
}

// Unscramble operation 1 of 2
pub fn unscramble_1(mut addr: u32, mut val: u32) -> (u32, u32) {
    val = rotate_left(val, 4);

    let mut tmp: u32 = (addr ^ val) & 0xF0F0F0F0;
    addr ^= tmp;
    val = rotate_right(val ^ tmp, 20);

    tmp = (addr ^ val) & 0xFFFF0000;
    addr ^= tmp;
    val = rotate_right(val ^ tmp,18);

    tmp = (addr ^ val) & 0x33333333;
    addr ^= tmp;
    val = rotate_right(val ^ tmp,6);

    tmp = (addr ^ val) & 0x00FF00FF;
    addr ^= tmp;
    val = rotate_left(val ^ tmp,9);

    tmp = (addr ^ val) & 0xAAAAAAAA;
    addr = rotate_left(addr ^ tmp,1);
    val ^= tmp;

    (addr, val)
}
// Unscramble operation 2 of 2
pub fn unscramble_2(mut addr: u32, mut val: u32) -> (u32, u32) {
    val = rotate_right(val, 1);

    let mut tmp: u32 = (addr ^ val) & 0xAAAAAAAA;
    val ^= tmp;
    addr = rotate_right(addr ^ tmp,9);

    tmp = (addr ^ val) & 0x00FF00FF;
    val ^= tmp;
    addr = rotate_left(addr ^ tmp,6);

    tmp = (addr ^ val) & 0x33333333;
    val ^= tmp;
    addr = rotate_left(addr ^ tmp,18);

    tmp = (addr ^ val) & 0xFFFF0000;
    val ^= tmp;
    addr = rotate_left(addr ^ tmp,20);

    tmp = (addr ^ val) & 0xF0F0F0F0;
    val ^= tmp;
    addr = rotate_right(addr ^ tmp,4);

    (addr, val)
}

// Original source: armax.c:armReadVerifier()
// Read verifier bit string from a decrypted cheat and return the number of code lines it occupies
pub fn read_verifier_length(input: &Vec<u32>) -> i16 {
    // TODO: [oddity] Is lines=1 an off-by-one in the original code? It's definitely required.
    // Output line count
    let mut lines: i16 = 1;

    // Bit counter
    let mut bits_read = 0;

    // Expansion sizes
    let exp_sizes: [u8; 8] = [
        6,      // ?
        10,     // ?
        12,     // ?
        19,     // Folder content
        19,     // Folder content
        8,      // Folder
        7,      // ?
        32,     // Disc hashes, other?
    ];

    // Key array for bit string operations
    let mut key: [u32; 3] = [
        1u32,   // Skip reading first WORD
        8u32,   // Skip reading first 8 bytes
        input.len() as u32,
    ];

    // Get initial verifier terminator
    let mut terminator = read_bit_string(input, &mut key, 1);   // TODO: Return an error if this errors
    bits_read += 1;

    while terminator < 1 {
        // Get index into expansion size array
        let exp_index = read_bit_string(input, &mut key, 3) as usize;
        bits_read += 3;

        // Get expansion data (unused)
        let _ = read_bit_string(input, &mut key, exp_sizes[exp_index]);
        bits_read += exp_sizes[exp_index];

        // Get next verifier terminator
        terminator = read_bit_string(input, &mut key, 1);
        bits_read += 1;
    }

    // There's only 24 bits on the first line for [ terminator | exp_index | exp_data ]
    if bits_read >= 24 {
        // Count first line
        bits_read -= 24;
        lines += 1;

        // Calculate the number of additional lines occupied
        if bits_read >= 64 {
            lines += (bits_read / 64) as i16;
        }
    }

    lines
}

// Read bits from arbitrary indexes within a Vec<u32> to form a u32
pub fn read_bit_string(input: &Vec<u32>, ctrl: &mut [u32; 3], length: u8) -> u32 {

    let mut output: u32 = 0;
    let mut tmp: u32 = magic::u32_pointer_increment(input, ctrl[0] << 2);

    for _ in 0..length {
        if ctrl[1] > 31 {
            ctrl[1] = 0;
            ctrl[0] += 1;
            tmp = magic::u32_pointer_increment(input, ctrl[0] << 2);
        }
        if ctrl[0] >= ctrl[2] {
            // TODO: Allow indicating error here instead of just panicking
            panic!("Error getting bitstring of length {}", length);
        }
        output = (output << 1) | ((tmp >> (31 - ctrl[1])) & 1);
        ctrl[1] += 1;
    }

    output
}