use crate::engine::cards::{Card, Deck};
use crate::engine::table::{
    player::{Player, PlayerType},
    team::{Team, TeamType},
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
    pub teams: [Team; 2],
    pub deck: Deck,
    pub current_player_index: usize,
    pub dealer_index: usize,
    pub leader_index: usize,
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
        let user_team = Team::new(user_team_name, TeamType::Home, [user, user_partner]);

        let opponent_1 = Player::new(opp1_name, PlayerType::Bot, cpu_hands.remove(0));
        let opponent_2 = Player::new(opp2name, PlayerType::Bot, cpu_hands.remove(0));
        let opposing_team = Team::new(opp_team_name, TeamType::Away, [opponent_1, opponent_2]);

        Game {
            state: GameState::default(),
            teams: [user_team, opposing_team],
            deck,
            current_player_index: 0,
            dealer_index: 0,
            leader_index: 0,
        }
    }

    pub fn current_player(&self) -> &Player {
        &self.teams[self.current_player_index / 2 as usize].players
            [self.current_player_index % 2 as usize]
    }

    pub fn next_player_index(&self) -> usize {
        (self.current_player_index + 1) % 4
        // should this be % 2?
    }

    pub fn next_turn(&mut self) {
        self.current_player_index = self.next_player_index();
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
