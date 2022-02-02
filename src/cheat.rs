#[derive(Clone)]
pub struct Cheat {
    pub id:         u32,
    pub name:       String,
    pub comment:    String,
    pub flags:      [u8; 3],
    pub codes:      Vec<u32>,
    pub state:      CheatStates,
}

#[derive(Clone, PartialEq)]
pub enum CheatStates {
    Unverified,
    Parsed,
    Decrypted,
    Translated,
}

impl Cheat {
    // Original source: cheat.c:cheatInit()
    pub fn new() -> Self {
        Self {
            id:         0,
            name:       "New Cheat".to_string(),
            comment:    String::new(),
            flags:      [0, 0, 0],
            codes:      vec![],
            state:      CheatStates::Unverified
        }
    }
}