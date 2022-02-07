use crate::Region;
use crate::token::Token;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CheatState {
    New,
    Parsed,
    Encrypted,
    Decrypted,
    Translated,
}

pub trait Cheat {
    // Initializer
    fn new() -> Self;

    // Tokenizer
    fn from_tokens(input: &Vec<Token>) -> Self;

    // State in the encryption/decryption/translation process
    fn state(&self) -> CheatState;

    // Game region cheat is meant for
    fn region(&self) -> Region;

    // Cheat ID
    fn id(&self) -> Option<u32>;

    // Parent Game ID, if applicable
    fn parent_id(&self) -> Option<u32>;

    // Whether or not this code is required
    // aka "Master Code", "Enable Code", etc.
    fn enable_code(&self) -> bool;

    // Cheat name
    fn name(&self) -> String;

    // Comments, e.g. "Press X+Y+Z to trigger"
    fn comment(&self) -> Option<String>;

    // Code octets
    fn codes(&self) -> Option<Vec<u32>>;
}

// Represents parsed but not-yet-identified cheats
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnknownCheat {
    pub id:     Option<u32>,
    pub parent: Option<u32>,
    pub state:  CheatState,
    pub name:   String,
    pub comment: Option<String>,
    pub region: Region,
    pub enable: bool,
    pub codes:  Option<Vec<u32>>
}

impl Cheat for UnknownCheat {
    fn new() -> Self {
        UnknownCheat {
            id: None,
            parent: None,
            state: CheatState::New,
            name: "".to_string(),
            comment: Some(String::new()),
            region: Region::Unknown,
            enable: false,
            codes: None
        }
    }

    fn from_tokens(input: &Vec<Token>) -> Self {
        todo!()
    }

    fn state(&self) -> CheatState { self.state.clone() }

    fn region(&self) -> Region { self.region.clone() }

    fn id(&self) -> Option<u32> { self.id }

    fn parent_id(&self) -> Option<u32> { self.parent }

    fn enable_code(&self) -> bool { self.enable }

    fn name(&self) -> String { self.name.clone() }

    fn comment(&self) -> Option<String> { self.comment.clone() }

    fn codes(&self) -> Option<Vec<u32>> { self.codes.clone() }
}