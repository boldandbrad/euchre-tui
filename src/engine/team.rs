use crate::engine::Seat;
use std::collections::HashSet;

#[derive(Default)]
pub struct Team {
    pub name: String,
    pub seats: HashSet<Seat>,
    pub game_score: u8,
    pub hand_score: u8,
}

impl Team {
    pub fn new(name: String, seats: HashSet<Seat>) -> Self {
        Team {
            name,
            seats,
            game_score: 0,
            hand_score: 0,
        }
    }
}
