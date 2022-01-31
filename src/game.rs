use crate::cheat::Cheat;

#[derive(Clone)]
pub struct Game {
    pub id:     u32,
    pub name:   String,
    pub cheats: Vec<Cheat>,
}