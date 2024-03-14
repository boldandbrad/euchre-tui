use crate::engine::table::player::Player;

#[derive(Clone)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
}

impl Team {
    pub fn new(name: String, players: Vec<Player>) -> Self {
        Team { name, players }
    }
}
