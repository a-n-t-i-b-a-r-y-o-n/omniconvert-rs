use crate::armax::table;
use crate::cheat::Cheat;
use crate::game::Game;
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
                            // Parity check passed! Add octets to output list
                            output.push((octet1, octet2));
                        }
                        else {
                            println!("[!] Parity bit validation failed! Octets: {:08X} / {:08X}", octet1, octet2);
                            // TODO: Fix parity calculations
                            // Push anyway
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

/*
pub fn whole_game(game: Game, input: &Vec<u32>, seeds: &[u32; 32]) -> Game {

    let mut output: Game = game.clone();

    for mut cheat in game.cheats {

        let mut output_codes: Vec<u32> = vec!();

        // Decrypt address/value pairs from pairs of codes
        let mut codes = cheat.codes.iter();
        while let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            let (out_addr, out_val) = decrypt_pair((*in_addr, *in_val), seeds);
            output_codes.push(out_addr);
            output_codes.push(out_val);
        }
        // TODO: Check for trailing address/value?

        // Read game metadata
        if output_codes.len() > 0 {
            let mut tmp: [u32; 3] = [0u32; 3];
            tmp[0] = 0u32;
            tmp[1] = 4u32;  // TODO: [oddity] Original source comment just says "skip crc"
            tmp[2] = input.len() as u32;

            // TODO: Apply game metadata properties to Game object
            let game_id = read_bit_string(&output_codes, &mut tmp, 13);
            let code_id = read_bit_string(&output_codes, &mut tmp, 19);
            let enable_code = read_bit_string(&output_codes, &mut tmp, 1) == 1;
            let unknown = read_bit_string(&output_codes, &mut tmp, 1) == 1;
            let region = read_bit_string(&output_codes, &mut tmp, 2) as Region;

            println!("[+] Properties:");
            println!("\tGame ID: {:04X}", game_id);
            println!("\tCode ID: {:08X}", code_id);
            println!("\tEnable Code: {}", enable_code);
            println!("\tUnknown: {}", unknown);
            println!("\tRegion: {:?}", region);

            // TODO: Verify the code with CRC16
            let check = output_codes[0];
            output_codes[0] &= 0x0FFFFFFF;

            // Update game metadata
            output.id = game_id;
            output.region = region;

            // Add to output cheat list
            let mut output_cheat = Cheat {
                id: code_id,
                name: if enable_code { String::from("Enable Code") } else { String::from("???") },
                comment: "".to_string(),
                flags: [0u8; 3],
                codes: output_codes,
                state: CheatStates::Decrypted
            };

            output.cheats.push(output_cheat);

        }
        else {
            // TODO: Handle errors more elegantly
            panic!("[!] No ARMAX cheats decrypted");
        }



    }

    output

}*/

// TODO: Update the game info through this function
// Decrypt a whole game's cheats
pub fn whole_game(game: Game, seeds: &[u32; 32]) -> Game {
    // Clone input to modify and return. Empty cheat list.
    let mut output: Game = game.clone();
    output.cheats = vec!();
    // Iterate through input Game's cheats
    for mut in_cheat in game.cheats {
        // Clone input cheat properties and empty code list
        let mut out_cheat = in_cheat.clone();
        out_cheat.codes = vec!();
        // Decrypt each code for this input cheat
        out_cheat.codes = decrypt_codes(&in_cheat.codes, seeds);
        // Push output cheat to output game's cheat list
        output.cheats.push(out_cheat);
    }

    output
}

pub fn decrypt_codes(input: &Vec<u32>, seeds: &[u32; 32]) -> Vec<u32> {
    let mut output: Vec<u32> = vec!();

    // Decrypt address/value pairs from pairs of codes
    let mut codes = input.iter();
    while let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
        let (out_addr, out_val) = decrypt_pair((*in_addr, *in_val), seeds);
        output.push(out_addr);
        output.push(out_val);
    }
    // TODO: Check for trailing address/value?

    // Read game metadata
    if output.len() > 0 {
        // Key for bit string operations
        let mut key: [u32; 3] = [
            0u32,
            4u32,   // TODO: [oddity] Original source comment just says "skip crc"
            input.len() as u32,
        ];

        // TODO: Apply game metadata properties to Game object
        // WARNING: READING PERMUTES THE KEY ARRAY  -   ORDER MATTERS!
        let game_id = read_bit_string(&output, &mut key, 13);
        let code_id = read_bit_string(&output, &mut key, 19);
        let master_code = read_bit_string(&output, &mut key, 1);
        let unknown = read_bit_string(&output, &mut key, 1);
        let region = read_bit_string(&output, &mut key, 2);

        println!("[+] Properties:");
        println!("\tGame ID: {:04X}", game_id);
        println!("\tCode ID: {:08X}", code_id);
        println!("\tEnable Code: {}", master_code);
        println!("\tUnknown: {}", unknown);
        println!("\tRegion: {}", region);

        // TODO: Verify the code with CRC16
        let check = output[0];
        output[0] &= 0x0FFFFFFF;

    }
    else {
        // TODO: Handle errors more elegantly
        panic!("[!] No ARMAX cheats decrypted");
    }

    output
}

// Decrypt a pair of ARMAX octets
pub fn decrypt_pair(input: (u32, u32), seeds: &[u32; 32]) -> (u32, u32) {
    // Byte swap 1/2
    // armax.c:getcode()
    let mut addr = swap_bytes(input.0);
    let mut val = swap_bytes(input.1);

    // Unscramble 1/2
    // armax.c:unscramble1()
    let unscrambled = unscramble_1(addr, val);
    addr = unscrambled.0;
    val = unscrambled.1;

    // Apply seeds
    let mut range = (0..32).into_iter();
    while let (
        // Seed indexes
        Some(seed_a), Some(seed_b), Some(seed_c), Some(seed_d)
    ) = (range.next(), range.next(), range.next(), range.next()) {

        let mut tmp = rotate_right(val, 4) ^ seeds[seed_a];
        let mut tmp2 = val ^ seeds[seed_b];
        addr ^= octet_mask(tmp, tmp2);

        tmp = rotate_right(addr,4) ^ seeds[seed_c];
        tmp2 = addr ^ seeds[seed_d];
        val ^= octet_mask(tmp, tmp2);
    }

    // Unscramble 2/2
    // armax.c:unscramble2()
    let unscrambled = unscramble_2(addr, val);
    addr = unscrambled.0;
    val = unscrambled.1;

    // Byte swap 2/2
    // Note that this also swaps the address and value
    let tmp_addr = addr;
    let tmp_val = val;
    addr = swap_bytes(tmp_val);
    val = swap_bytes(tmp_addr);

    (addr, val)
}

// Mask XOR'd to address/value octets
pub fn octet_mask(i1: u32, i2: u32) -> u32 {
    table::T6[(i1&63) as usize]             ^  table::T4[((i1>>8)&63) as usize]  ^
        table::T2[((i1>>16)&63) as usize]   ^  table::T0[((i1>>24)&63) as usize] ^
        table::T7[(i2&63) as usize]         ^  table::T5[((i2>>8)&63) as usize]  ^
        table::T3[((i2>>16)&63) as usize]   ^  table::T1[((i2>>24)&63) as usize]
}

pub fn unscramble_1(mut addr: u32, mut val: u32) -> (u32, u32) {
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

pub fn unscramble_2(mut addr: u32, mut val: u32) -> (u32, u32) {
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

fn read_bit_string(input: &Vec<u32>, ctrl: &mut [u32; 3], length: u8) -> u32 {
    // Emulate C raw pointer increment logic
    let mut tmp: u32 = magic::emulate_pointer_increment(input, ctrl[0] << 2);

    let mut output: u32 = 0;

    for _ in 0..length {
        if ctrl[1] > 31 {
            ctrl[1] = 0;
            ctrl[0] += 1;
            tmp = magic::emulate_pointer_increment(input, ctrl[0] << 2);
        }
        if ctrl[0] >= ctrl[2] {
            panic!("Error getting bitstring of length {}", length);
        }
        output = ((output << 1) | ((tmp >> (31 - ctrl[1])) & 1));
        ctrl[1] += 1;
    }

    output
}

// Original sources: armax.c:rotate_left() & armax.c:rotate_right()
pub fn rotate_left(input: u32, rot: u8) -> u32 { (input << rot) | (input >> (32 - rot)) }
pub fn rotate_right(input: u32, rot: u8) -> u32 { (input >> rot) | (input << (32 - rot)) }

// Original source: armax.c:byteswap()
// Shuffle bytes around
pub fn swap_bytes(input: u32) -> u32 { (input << 24) | ((input << 8) & 0x00FF0000) | ((input >> 8) & 0x0000FF00) | (input >> 24) }