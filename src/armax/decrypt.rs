use crate::cheat::Cheat;

// TODO: Translate alpha_to_octets() from alphatobin() less literally
// Decode ARMAX lines into pairs of address/value octets
pub fn alpha_to_octets(input: Vec<&str>) -> Option<Vec<(u32, u32)>> {
    // Output octets
    let mut output: Vec<(u32, u32)> = vec!();

    // Replaces provided size parameter in original source
    let mut lines_left = input.len();

    // Built octet count
    let mut octet_count = 0;

    // Parity flag
    let mut parity: u8 = 0;

    let alphabet = vec!(
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
        'M', 'N', 'P', 'Q', 'R', 'T', 'U', 'V', 'W', 'X',
        'Y', 'Z');

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
                    return None
                }
                Some(char_in) => {
                    // Get index of input char in cipher alphabet
                    match alphabet.iter().position(|&char_alpha| char_alpha == char_in) {
                        None => {
                            // TODO: Handle parsing error gracefully
                            panic!("[!] Received non-alphanumeric character \"{}\" in ARMAX code at index {}", char_in, index);
                            return None
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
                    return None
                }
                Some(char_in) => {
                    // Get index of input char in cipher alphabet
                    match alphabet.iter().position(|&char_alpha| char_alpha == char_in) {
                        None => {
                            // TODO: Handle parsing error gracefully
                            panic!("[!] Received non-alphanumeric character \"{}\" in ARMAX code at index {}", char_in, index+6);
                            return None
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

        // Calculate parity bit
        parity = 0;
        for i in 0..64 {
            if i < 32 {
                parity ^= (octet1 >> (i-(0<<5))) as u8;
            }
            else {
                parity ^= (octet1 >> (i-(1<<5))) as u8;
            }
        }

        // Verify parity bit and potentially add to output list
        match input[(octet_count-2)>>1].chars().nth(12) {
            None => {
                // TODO: Handle parsing errors gracefully
                panic!("[!] Unable to get character at 12 from input of length {}", input[octet_count>>1].len());
                return None
            }
            Some(char_in) => {
                // Get index of input char in cipher alphabet
                match alphabet.iter().position(|&char_alpha| char_alpha == char_in) {
                    None => {
                        // TODO: Handle parsing error gracefully
                        panic!("[!] Received non-alphanumeric character \"{}\" in ARMAX code at index {}", char_in, 12);
                        return None
                    }
                    Some(match_index) => {
                        if parity&1 != ((match_index as u8)&1) {
                            // Parity check passed! Add octets to output list
                            output.push((octet1, octet2));
                        }
                        else {
                            panic!("[!] Parity bit validation failed! Octets: {} / {}", octet1, octet2);
                            return None
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

pub fn batch(input: &mut Vec<u32>, seeds: Vec<u32>) {
    let mut output: Vec<u32> = vec!();
    // Default ActionReplay MAX seed
    // TODO: Investigate ARMAX seeds.
    // This is declared in omniconvert.c as `armseeds` for use in batch ARMAX encryption/decryption functions.
    // In armax.c, `armBatchDecryptFull()` receives it as the `ar2key` parameter.
    let default_seed: u32 = 0x04030209;

    // Decrypt address/value pairs from pairs of codes
    let mut codes = input.iter();
    while let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
        let (out_addr, out_val) = decrypt_pair((*in_addr, *in_val), &seeds);
        output.push(out_addr);
        output.push(out_val);
    }
    // TODO: Check for trailing address/value?

    if output.len() > 0 {
        let mut tmp: [u32; 4] = [0u32; 4];
        tmp[0] = output[0];
        tmp[1] = 0u32;
        tmp[2] = 4u32;  // TODO: [oddity] Original source comment just says "skip crc"
        tmp[3] = input.len() as u32;

        // TODO: You left off here.
        // Start with the getbitstring() function, which is applied to tmp[] and another slice.
        // Don't forget to define the tablex[] slices mentioned in decrypt_pair()
    }
    else {
        // TODO: Handle errors more elegantly
        panic!("[!] No ARMAX cheats decrypted");
    }

}

// TODO: Define the tables referenced in decrypt_pair (e.g. table6, table4, ...)
// Decrypt a pair of ARMAX octets
fn decrypt_pair(input: (u32, u32), seeds: &Vec<u32>) -> (u32, u32) {
    // Byte swap 1/2
    // armax.c:getcode()
    let mut addr = swap_bytes(input.0);
    let mut val = swap_bytes(input.1);

    // Unscramble 1/2
    // armax.c:unscramble1()
    (addr, val) = unscramble_1(addr, val);

    // Apply seeds
    let mut range = (0..32).into_iter();
    while let (
        // Seed indexes
        Some(seed_a), Some(seed_b), Some(seed_c), Some(seed_d)
    ) = (range.next(), range.next(), range.next(), range.next()) {

        let mut tmp = rotate_right(val, 4) ^ seeds[seed_a];
        let mut tmp2 = val ^ seeds[seed_b];
        addr ^= (
            table6[tmp&63]       ^  table4[(tmp>>8)&63]  ^
            table2[(tmp>>16)&63] ^  table0[(tmp>>24)&63] ^
            table7[tmp2&63]      ^  table5[(tmp2>>8)&63] ^
            table3[(tmp2>>16)&63]^  table1[(tmp2>>24)&63]
        );

        tmp = rotate_right(addr,4) ^ seeds[seed_c];
        tmp2 = addr ^ seeds[seed_d];
        val ^= (
            table6[tmp&63]       ^  table4[(tmp>>8)&63]  ^
            table2[(tmp>>16)&63] ^  table0[(tmp>>24)&63] ^
            table7[tmp2&63]      ^  table5[(tmp2>>8)&63] ^
            table3[(tmp2>>16)&63]^  table1[(tmp2>>24)&63]
        );
    }

    // Unscramble 2/2
    // armax.c:unscramble2()
    (addr, val) = unscramble_2(addr, value);

    // Byte swap 2/2
    addr = swap_bytes(val);
    val = swap_bytes(addr);

    (addr, val)
}

fn unscramble_1(mut addr: u32, mut val: u32) -> (u32, u32) {
    val = rotate_left(val, 4);

    let mut tmp: u32 = ((addr ^ val) & 0xF0F0F0F0);
    addr ^= tmp;
    val = rotate_right((val ^ tmp), 20);

    tmp = ((addr ^ val) & 0xFFFF0000);
    addr ^= tmp;
    val = rotate_right((val ^ tmp),18);

    tmp = ((addr ^ val) & 0x33333333);
    addr ^= tmp;
    val = rotate_right((val ^ tmp),6);

    tmp = ((addr ^ val) & 0x00FF00FF);
    addr ^= tmp;
    val = rotate_left((val ^ tmp),9);

    tmp = ((addr ^ val) & 0xAAAAAAAA);
    addr = rotate_left((addr ^ tmp),1);
    val ^= tmp;

    (addr, val)
}

fn unscramble_2(mut addr: u32, mut val: u32) -> (u32, u32) {
    val = rotate_right(val, 1);

    let mut tmp: u32 = (addr ^ val) & 0xAAAAAAAA;
    val ^= tmp;
    addr = rotate_right((addr ^ tmp),9);

    tmp = (addr ^ val) & 0x00FF00FF;
    val ^= tmp;
    addr = rotate_left((addr ^ tmp),6);

    tmp = (addr ^ val) & 0x33333333;
    val ^= tmp;
    addr = rotate_left((addr ^ tmp),18);

    tmp = (addr ^ val) & 0xFFFF0000;
    val ^= tmp;
    addr = rotate_left((addr ^ tmp),20);

    tmp = (addr ^ val) & 0xF0F0F0F0;
    val ^= tmp;
    addr = rotate_right((addr ^ tmp),4);

    (addr, val)
}

fn rotate_left(input: u32, rot: u8) -> u32 { (input << rot) | (input >> (32 - rot)) }

fn rotate_right(input: u32, rot: u8) -> u32 { (input >> rot) | (input << (32 - rot)) }

// Original source: armax.c:byteswap()
// Shuffle bytes around
fn swap_bytes(input: u32) -> u32 {
    (input << 24) | ((input << 8) & 0x00FF0000) | ((input >> 8) & 0x0000FF00) | (input >> 24)
}