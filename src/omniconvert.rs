use crate::formats::{CodeDevice, CodeFormat, CodeType, get_formats};
use crate::game::Game;
use crate::cheat::{Cheat, CheatStates};
use crate::token::{Token, TokenType};
use crate::armax;

// Which mode is represented by a given operation/options
enum CryptMode {
    Input,
    Output,
}

enum ARMAXVerifierMode {
    Manual,
    Auto,
}

// Game regions
enum Region {
    USA,
    PAL,
    Japan,
}

// Parsing method
enum ParserType {
    Simple,
    Strict,
    Reformat,
}

// Encryption/decryption options
struct CryptOpt {
    mode:   CryptMode,
    code:   CodeType,
}

struct State {
    // Input & output formats
    incrypt:    CryptOpt,
    outcrypt:   CryptOpt,
    // Parser options
    parser:     ParserType,
    // ARMAX Verifier mode
    verifier:   ARMAXVerifierMode,
    // Game region
    region:     Region,
}

impl State {
    // Initialize the default environment
    fn new() -> Self {
        // Get list of supported formats
        let mut formats = get_formats();
        // Return default State object
        State {
            // Default to ARMAX input
            incrypt: CryptOpt {
                mode: CryptMode::Input,
                code: formats[8].clone(),
            },
            // Default to RAW output
            outcrypt: CryptOpt {
                mode: CryptMode::Output,
                code: formats[0].clone(),
            },
            parser: ParserType::Simple,
            verifier: ARMAXVerifierMode::Auto,
            region: Region::USA
        }
    }
}

// Tokenize input based on a given format
// Remarks: Most formats are handled similarly, with the exception of ARMAX
fn read_input(input: &str, format: CodeFormat) -> Vec<Token> {

    // Output tokens
    let mut output: Vec<Token> = vec![];

    // Iterate each line
    println!("[-] Iterating input lines...");
    for line in input.lines() {

        println!("[?] Line: \"{}\"", line);

        if !line.is_empty() {
            // Line has some tokens in it.

            // Flag to indicate if we read a code address last iteration and are expecting a value
            let mut expecting_value = false;
            // Iterate tokens on this line
            println!("[-] Iterating tokens...");
            for (i, t) in line.split_whitespace().enumerate() {
                // Ignore comment lines starting with '#'
                if i == 0 && t.chars().nth(0) == Some('#') {
                    break;
                }

                // Identify token type
                let mut token_type: TokenType;
                if !expecting_value {
                    // Identify as normal, handling ARMAX codes as necessary
                    token_type = Token::identify_type(t, format == CodeFormat::ARMAX);
                    // If we just read a code, set it to be an address & set the next iteration to expect a code value
                    if token_type == TokenType::Code {
                        token_type = TokenType::CodeAddress;
                        expecting_value = true;
                    }
                }
                else {
                    // TODO: Avoid blindly expecting code values
                    // We're expecting a code address, since we read a code value last iteration
                    token_type = TokenType::CodeValue;
                    // Reset the 'expecting' flag
                    expecting_value = false;
                }

                // Add Token object to output
                output.push(Token {
                    string:     String::from(t),
                    is_multi:   false,
                    types:      vec!(token_type),
                });
            }
            // Set the last token to also be an end-of-line token
            if let Some(last) = output.last_mut() {
                last.is_multi = true;
                last.types.push(TokenType::EndOfLine);
            }
        }
        else {
            // Line is empty.

            // Set the previous token to also be an end-of-block token
            if let Some(last) = output.last_mut() {
                last.is_multi = true;
                last.types.push(TokenType::EndOfBlock);
            }

            // Add a newline token to use when serializing output
            output.push(Token {
                string:     String::from("\n"),
                is_multi:   true,
                types:      vec!(TokenType::String, TokenType::NewLine, TokenType::EndOfLine),
            })
        }

        // TODO: Go back and set pairs of hex octets to be code address/code values
    }

    println!("[-] Done iterating input lines.");

    // Clean up input and delineate individual cheats
    println!("[-] Cleaning up input");
    for (i, t) in output.iter_mut().enumerate() {
        // Consider all remaining raw hex octets to actually be strings
        if t.types.first() == Some(&TokenType::HexOctet) {
            t.types[0] = TokenType::String;
        }
        // TODO: Should we identify 'EndCode' tokens the same was as the original code?
        // It's currently done as part of reading an empty line.

    }

    // Ensure that the last token is an end-of-block token
    if let Some(last) = output.last_mut() {
        if !last.types.contains(&TokenType::EndOfBlock) {
            last.is_multi = true;
            last.types.push(TokenType::EndOfBlock);
        }
    }

    output
}

// Build a vec of Cheat objects from a vec of Token objects
fn build_cheat_list(token_list: Vec<Token>) -> Vec<Cheat> {
    // Output cheat list
    let mut output: Vec<Cheat> = vec![];

    // Flag used to indicate we're reading the cheat name
    let mut reading_name = true;

    // Cheat object currently being built
    let mut cheat = Cheat::new();

    // String currently being built
    let mut s = String::new();

    // Iterate through tokens to build a list of cheats
    let mut tokens = token_list.iter();
    loop {
        // Get the next token or None if we're at the end
        let mut next = tokens.next();
        if next == None {
            // We've reached the end. Stop looping.
            break;
        }
        // Unwrap our 'next' value to be the current token
        let token = next.unwrap();

        if token.types.contains(&TokenType::String) {
            // Handle strings - name or comments

            // TODO: Clean up cheat name/comment reading process (ported pretty directly from C source)

            if !token.types.contains(&TokenType::NewLine) {
                // Token isn't a newline

                s += &token.string;

                if token.types.contains(&TokenType::EndOfLine) {

                    if !reading_name {
                        s += "\n";
                    }
                    else {
                        // Expect further strings to be comments
                        cheat.name = String::from(s);
                        s = String::new();

                        reading_name = false;
                    }
                }
                else {
                    s += " ";
                }
            }
            else if !reading_name {

                // Found a newline after reading the name, indicating the end of one cheat and start of another.
                // Add current cheat to output list, then start a new one.

                // Set built string to cheat's comment
                cheat.comment = String::from(s);
                s = String::new();

                // Set cheat as parsed
                cheat.state = CheatStates::Parsed;

                output.push(cheat.clone());

                cheat = Cheat::new();

                reading_name = true;
            }
        }
        else if token.types.contains(&TokenType::CodeAddress) {
            // Handle code address token

            // If current token is address, next token must be value.
            // Take next token as value for current address token.
            if let Some(subsequent) = tokens.next() {
                // Attempt to parse the address/value octets
                if let (Ok(address), Ok(value)) =
                (hex::decode(&token.string), hex::decode(&subsequent.string))
                {
                    // Double-check our length
                    if address.len() < 4 || value.len() < 4 {
                        // TODO: Handle parsing errors gracefully
                        println!("[!] Received address/value of lengths {}/{}", address.len(), value.len());
                    }
                    else {
                        // Add parsed octets, combining u8s to form a u32.
                        // Address octet
                        cheat.codes.push(
                            ((address[0] as u32) << 3) +
                            ((address[1] as u32) << 2) +
                            ((address[2] as u32) << 1) +
                            (address[3] as u32));
                        // Value octet
                        cheat.codes.push(
                            ((value[0] as u32) << 3) +
                            ((value[1] as u32) << 2) +
                            ((value[2] as u32) << 1) +
                            (value[3] as u32));
                    }
                }
                else {
                    // TODO: Handle parsing errors gracefully
                    //println!("[!] Unable to parse address/value pair: ({}/{})", &token.string, &value.string);
                }

                // If we hit the end of a text/token block, start a new cheat.
                if subsequent.types.contains(&TokenType::EndOfBlock) {

                    // Set cheat as parsed
                    cheat.state = CheatStates::Parsed;

                    // Add to output list
                    output.push(cheat.clone());

                    // Start a new cheat
                    cheat = Cheat::new();
                    reading_name = true;
                }
            }
            else {
                // TODO: Handle parsing errors gracefully
                println!("[!] Expected value for final address token: {:?}", token);
            }
        }
        else if token.types.contains(&TokenType::ARMAXCode) {
            // Handle ARMAX code token

            // Remove the dashes
            let raw_chars = token.string.replace("-", "");

            // Attempt to decode the ARMAX string to an address/value pair of octets
            if let Some(octets) = armax::alpha_to_octets(vec!(&raw_chars)) {
                // Add the octets to our code list
                for octet in octets {
                    cheat.codes.push(octet.0);
                    cheat.codes.push(octet.1);
                }
            }
            else {
                // TODO: Handle parsing errors gracefully
                println!("[!] Unable to parse ARMAX code to octets: {:?}", &token.string);
            }

            // If we hit the end of a text/token block, start a new cheat.
            if token.types.contains(&TokenType::EndOfBlock) {
                // Set cheat as parsed
                cheat.state = CheatStates::Parsed;

                // Add to output list
                output.push(cheat.clone());

                // Start a new cheat
                cheat = Cheat::new();
                reading_name = true;
            }
        }
        else {
            // Unhandled token
            println!("[!] Unhandled token of type(s) {:?} - {:?}", token.types, token);
        }

    }

    output
}

pub fn decrypt_and_translate(state: &State, game: &Game) {
    // TODO: Make an ARMAX disc hash if we're using ARMAX output w/ auto verifier
    if state.outcrypt.code.device == CodeDevice::ARMAX && state.verifier == ARMAXVerifierMode::Auto {
        panic!("[!] ARMAX disc hashes not implemented yet");
    }

    // TODO: Reset CB devices for input mode

    for cheat in game.cheats {
        // Decrypt the code
        // TODO: Left off here
    }

    // TODO: Reset CB devices for output mode
}

// Minimum requirements to perform code conversion
pub fn minimal_conversion() {

    // DEBUG: Test input - the "Master Code" for Kingdom Hearts
    let test_input = "UQRN-ER36-M3RD5\nWC60-T93N-MGJBW\n7QTG-QEQB-YXP60\nVFE7-FK9B-M32EA\nKQEK-5ZFB-F8UP9";

    // Set up default environment
    println!("--> Begin conversion...");
    let mut state: State = State::new();

    // TODO: Build ARMAX seeds

    // TODO: Build GS3 seeds

    // Initialize game object
    println!("[-] Initializing game object");
    // TODO: Get Game ID from user
    let mut game: Game = Game {
        id: 0x1234&0x1FF,       // TODO: Figure out the game id mask thing
        name: "New Game".to_string(),
        cheats: vec![]
    };

    // Read test input
    println!("[-] Reading input");
    // TODO: Read input into tokens
    let tokens = read_input(test_input, state.incrypt.code.format);

    // Parse tokens into cheats
    println!("[-] Building cheat list");
    // TODO: Parse tokens into cheats
    game.cheats = build_cheat_list(tokens);

    // Decrypt and translate cheats
    decrypt_and_translate(&state, &game);
}