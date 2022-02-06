#[derive(Clone)]
pub struct Cheat {
    pub game_id:        u32,            //  Parent Game ID
    pub region:         u8,             //  Game region
    pub id:             u32,            //  Cheat ID
    pub name:           String,         //  Cheat name
    pub comment:        String,         //  Cheat comment(s)
    pub flags:          [u8; 3],        //  TODO: Remove Cheat flags?
    pub enable_code:    bool,           //  Whether this code is the 'Master Code'
    pub codes:          Vec<u32>,       //  Codes composing this cheat
    pub state:          CheatStates,    //  Decryption/translation state
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
            game_id: 0,
            region: 0,
            id: 0,
            name: "New Cheat".to_string(),
            comment: "".to_string(),
            flags: [0u8; 3],
            enable_code: false,
            codes: vec![],
            state: CheatStates::Unverified
        }
    }
}