use crate::cheat::{Cheat, UnknownCheat};
use crate::Region;

#[derive(Clone)]
pub struct Game {
    pub id:     u32,
    pub name:   String,
    pub cheats: Vec<UnknownCheat>,
    pub region: Region,
}

impl Game {
    pub fn new() -> Self {
        Game {
            id: 0x1234&0x1FF,       // TODO: Figure out the game id mask thing
            name: "New Game".to_string(),
            cheats: vec![],
            region: Region::Unknown,
        }
    }
}