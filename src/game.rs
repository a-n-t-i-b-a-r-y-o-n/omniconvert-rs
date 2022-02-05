use crate::cheat::Cheat;

#[derive(Clone)]
pub struct Game {
    pub id:     u32,
    pub name:   String,
    pub cheats: Vec<Cheat>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            id: 0x1234&0x1FF,       // TODO: Figure out the game id mask thing
            name: "New Game".to_string(),
            cheats: vec![]
        }
    }
}