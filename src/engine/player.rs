use crate::engine::card::Card;

#[derive(Default)]
pub enum PlayerType {
    User,
    #[default]
    Bot,
    // Network,
}

#[derive(Default)]
pub struct Player {
    pub name: String,
    pub player_type: PlayerType,
    pub hand: Vec<Card>,
}

impl Player {
    // create a new player
    pub fn new(name: String, player_type: PlayerType) -> Self {
        Player {
            name,
            player_type,
            hand: vec![],
        }
    }

    pub fn call_pickup() {}

    pub fn pickup_card() {}

    pub fn call_suit() {}

    pub fn play_card() {}
}
