use regex::Regex;
use crate::formats::{CodeFormat, CodeType, FORMATS};
use crate::cheat::{Cheat, CheatState, UnknownCheat};
use crate::token::{Token, TokenType};
use crate::{armax, Region};
use crate::ar2;
use crate::armax::cheat::ARMAXCheat;

// Which mode is represented by a given operation/options
pub enum CryptMode {
    Input,
    Output,
}

// Parsing method
enum ParserType {
    Simple,
    Strict,
    Reformat,
}

// Encryption/decryption options
pub struct CryptOpt {
    pub mode:   CryptMode,
    pub code:   CodeType,
}

pub struct Seeds {
    // ARMAX Seeds
    pub armax: [u32; 32],
    // AR2 seeds
    pub ar2: [u8; 4],
}

impl Seeds {
    // Initialize the default environment
    pub fn new() -> Self {
        // Return default State object
        Seeds {
            armax: armax::seeds::generate(),
            ar2: ar2::seeds::generate(),
        }
    }
}

pub fn read_input(input: &str) -> Vec<UnknownCheat> {
    // RegEx patterns for comments and general octets.
    // Cheats of supported types will have recognize() functions.
    let PATTERN_COMMENT: Regex    = Regex::new(r#"^\s*(#|//)\s*(.+)"#).unwrap();
    let PATTERN_OCTET: Regex      = Regex::new(r#"^\s*([ABCDEF\d]{8})\s+([ABCDEF\d]{8})\s*"#).unwrap();

    // Output cheat list
    let mut output: Vec<UnknownCheat> = vec![];

    // Properties of cheat object currently being built
    let mut codes: Vec<u32> = vec![];
    let mut name: String = String::new();
    let mut comment: String = String::new();

    // Whether we are expecting a name
    let mut read_name: bool = true;

    // Read each input line
    for line in input.lines() {
        // Empty lines
        if line.is_empty() {
            // Create cheat
            let cheat = UnknownCheat {
                id: None,
                parent: None,
                state: CheatState::Parsed,
                name: name.clone(),
                comment: Some(comment.clone()),
                region: Region::Unknown,
                enable: false,
                codes: Some(codes.clone()),
            };

            // Push working cheat to output
            output.push(cheat.clone());

            // Reset input
            name = String::new();
            comment = String::new();
            codes = vec![];
            read_name = true;

            // Continue reading lines
            continue;
        }
        // Comments
        if PATTERN_COMMENT.is_match(line) {
            // Captures = [0]: Whole line  [1]: Comment signifier  [2]: Comment text
            if let Some(captures) = PATTERN_COMMENT.captures(line) {
                if comment.is_empty() {
                    comment = String::from(captures[2].trim());
                }
                else {
                    comment = comment + " " + captures[2].trim();
                }
            }

            // Continue reading lines
            continue;
        }
        // Raw octets
        if PATTERN_OCTET.is_match(line) {
            if let Some(captures) = PATTERN_OCTET.captures(line) {
                // Captures = [0]: Whole line  [1]: Address  [2]: Value
                if captures.len() == 3 {
                    // Parse hex octets
                    if let (Ok(addr), Ok(val)) =
                        (hex::decode(&captures[1]), hex::decode(&captures[2]))
                    {
                        // Add parsed octets, combining u8s to form a u32.
                        // Address octet
                        codes.push(
                            ((addr[0] as u32) << 3) +
                                ((addr[1] as u32) << 2) +
                                ((addr[2] as u32) << 1) +
                                (addr[3] as u32));
                        // Value octet
                        codes.push(
                            ((val[0] as u32) << 3) +
                                ((val[1] as u32) << 2) +
                                ((val[2] as u32) << 1) +
                                (val[3] as u32));

                        // Continue reading lines
                        continue;
                    }
                }
            }
        }
        // ActionReplay MAX
        if armax::recognize(line) {
            // Remove the dashes
            let raw_chars = line.replace("-", "");

            // Attempt to decode the ARMAX string to an address/value pair of octets
            if let Some(octets) = armax::decrypt::alpha_to_octets(vec!(&raw_chars)) {
                // Add the octets to our code list
                for octet in octets {
                    codes.push(octet.0);
                    codes.push(octet.1);
                }

                // Continues reading lines
                continue;
            }
            else {
                // TODO: Handle parsing errors gracefully
                println!("[!] Unable to parse ARMAX code to octets: {:?}", line);
            }
        }
        // Strings
        if read_name {
            // Set the cheat name if we were expecting to read a name.
            name = String::from(line.trim());
            read_name = false;

            // Continue reading lines
            continue;
        }
        else {
            // Unexpected string. Discard.
            println!("[-] Unexpected line in input: {}", line);
        }
    }
    if !codes.is_empty() {
        // No newline at end of file. Create & push last cheat.

        // Create cheat
        let cheat = UnknownCheat {
            id: None,
            parent: None,
            state: CheatState::Parsed,
            name: name.clone(),
            comment: Some(comment.clone()),
            region: Region::Unknown,
            enable: false,
            codes: Some(codes.clone()),
        };

        // Push working cheat to output
        output.push(cheat.clone());
    }

    output
}