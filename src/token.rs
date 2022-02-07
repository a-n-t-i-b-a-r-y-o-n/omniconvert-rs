use crate::armax;

// Types of tokens we may encounter
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    Wildcard,
    String,
    HexOctet,
    Code,
    CodeAddress,    // "CODEADDR" in original source
    CodeValue,      // "CODEVAL" in original source
    ARMAXCode,      // "ARMCODE" in original source
    EndOfBlock,     // "ENDCODE" in original source
    EndOfLine,      // "ENDLINE" in original source
    NewLine,
}

// Token parsed from input
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub string:     String,         // String of characters forming the token
    pub is_multi:   bool,           // Whether this token represents more than one type
    pub types:      Vec<TokenType>, // Type(s) of token represented
}

impl Token {
    // Original sources: omniconvert.c:GetTokenType() and omniconvert.c:ProcessText()
    // Identify the "type" of a token read from input. Must know if we're expecting ARMAX codes or not.
    pub fn identify_type(input: &str, expect_armax: bool) -> TokenType {
        // Get initial guess
        match input.len() {
            8 => {
                if expect_armax {
                    // Must be a string if we're reading ARMAX codes
                    TokenType::String
                }
                else {
                    // If it's all hex chars, consider it an octet. Otherwise, a string.
                    match input.chars().all(|c| c.is_ascii_hexdigit()) {
                        true => TokenType::HexOctet,
                        false => TokenType::String,
                    }
                }
            },
            16 => {
                if expect_armax {
                    // Must be a string if we're reading ARMAX codes
                    TokenType::String
                }
                else {
                    // If it's all hex chars, consider it a code. Otherwise, a string.
                    match input.chars().all(|c| c.is_ascii_hexdigit()) {
                        true => TokenType::Code,
                        false => TokenType::String,
                    }
                }
            }
            15 => {
                if !expect_armax {
                    // Must be a string if we're NOT reading ARMAX codes
                    TokenType::String
                }
                else {
                    // If it looks like an ARMAX code, consider it one. Otherwise, a string.
                    if armax::recognize(input) {
                        TokenType::ARMAXCode
                    }
                    else {
                        TokenType::String
                    }
                }
            }
            _ => {
                TokenType::String
            }
        }
    }
}