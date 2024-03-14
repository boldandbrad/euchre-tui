use crate::engine::table::player::Player;

#[derive(Clone, Default)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
    pub game_score: u8,
    pub hand_score: u8,
}

impl Team {
    pub fn new(name: String, players: Vec<Player>) -> Self {
        Team {
            name,
            players,
            ..Default::default()
        }
    }
}
