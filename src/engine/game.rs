use crate::engine::cards::{Card, Deck};
use crate::engine::table::{
    player::{Player, PlayerType},
    team::Team,
};

const NUM_CPUS: usize = 3;
// const WINNING_SCORE: u8 = 10;

#[derive(Clone, Default)]
pub enum GameState {
    #[default]
    PickingDealer,
    CallingPickup,
    CallingHighSuit,
    PlayingHand,
}

// Define the game state
#[derive(Clone)]
pub struct Game {
    // TODO: find a better way to identify left/right, top/bottom players from teams
    pub state: GameState,
    pub user_team: Team,
    pub opposing_team: Team,
    pub deck: Deck,
}

impl Game {
    // Initialize the game state with a specified number of CPU players
    pub fn new(
        user_name: String,
        partner_name: String,
        opp1_name: String,
        opp2name: String,
        user_team_name: String,
        opp_team_name: String,
    ) -> Self {
        // Create and shuffle a deck of cards
        let mut deck = Deck::new();

        // Deal hands to player and CPU players
        let user_hand = deck.deal(5);

        let mut cpu_hands: Vec<Vec<Card>> = Vec::new();
        for _ in 0..NUM_CPUS {
            cpu_hands.push(deck.deal(5));
        }

        let user = Player::new(user_name, PlayerType::User, user_hand);
        let user_partner = Player::new(partner_name, PlayerType::Bot, cpu_hands.remove(0));
        let user_team = Team::new(user_team_name, vec![user, user_partner]);

        let opponent_1 = Player::new(opp1_name, PlayerType::Bot, cpu_hands.remove(0));
        let opponent_2 = Player::new(opp2name, PlayerType::Bot, cpu_hands.remove(0));
        let opposing_team = Team::new(opp_team_name, vec![opponent_1, opponent_2]);

        Game {
            state: GameState::default(),
            user_team,
            opposing_team,
            deck,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new(
            "Brad".to_string(),
            "Partner".to_string(),
            "Opponent 1".to_string(),
            "Opponent 2".to_string(),
            "Good Guys".to_string(),
            "Bad Guys".to_string(),
        )
    }
}
