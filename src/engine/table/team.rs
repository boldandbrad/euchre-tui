use crate::engine::table::player::Player;

#[derive(Clone, Default)]
pub enum TeamType {
    Home,
    #[default]
    Away,
}

#[derive(Clone, Default)]
pub struct Team {
    pub name: String,
    pub team_type: TeamType,
    pub players: [Player; 2],
    pub game_score: u8,
    pub hand_score: u8,
}

impl Team {
    pub fn new(name: String, team_type: TeamType, players: [Player; 2]) -> Self {
        Team {
            name,
            team_type,
            players,
            ..Default::default()
        }
    }
}
