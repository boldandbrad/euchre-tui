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

// TODO: finish this impl
// impl GameState {
//     pub fn new(
//         user_name: String,
//         partner_name: String,
//         opp1_name: String,
//         opp2name: String,
//         user_team_name: String,
//         opp_team_name: String,
//     ) -> Self {
//         GameState {
//             user_team: Team::new(),
//             opposing_team: Team::new(),
//             deck: Vec::new(),
//         }
//     }
// }
