pub mod card;
pub mod game;
pub mod player;
pub mod table;
pub mod team;

use crate::engine::{player::Player, table::Seat};
use std::collections::HashMap;

pub type PlayerMap = HashMap<Seat, Player>;
