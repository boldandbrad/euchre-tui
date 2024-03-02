use crate::structs::Card;
use crate::table::team::Team;

// Define the game state
#[derive(Clone)]
pub struct GameState {
    // TODO: find a better way to identify left/right, top/bottom players from teams
    pub user_team: Team,
    pub opposing_team: Team,
    pub deck: Vec<Card>,
}
