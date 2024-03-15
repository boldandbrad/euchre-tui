pub mod player;
pub mod team;

use crate::engine::table::player::Player;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub enum Seat {
    #[default]
    Bottom,
    Left,
    Top,
    Right,
}

pub type PlayerMap = HashMap<Seat, Player>;

pub struct Table {
    pub players: PlayerMap,
}
