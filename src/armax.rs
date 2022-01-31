use std::iter::{Enumerate, FilterMap};
use std::slice::Iter;

// Attempt to recognize if this string is an ARMAX code or not
pub fn is_armax_code(input: &str) -> bool {
    input.chars().all(|c| { c.is_alphanumeric() || c == '-' }) &&
        input.chars().nth(4) == Some('-') && input.chars().nth(9) == Some('-')
}

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