use crate::engine::{
    card::{Card, Deck},
    player::{Player, PlayerType},
    table::Seat,
    team::Team,
    PlayerMap,
};
use std::collections::HashSet;

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
    pub state: GameState,
    pub user_team: Team,
    pub opp_team: Team,
    pub players: PlayerMap,
    pub deck: Deck,
    pub current_player_seat: Seat,
    pub dealer_seat: Seat,
    pub leader_seat: Seat,
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
        // TODO: deal cards to players after game creation and choosing dealer
        let user_hand = deck.deal(5);

        let mut cpu_hands: Vec<Vec<Card>> = Vec::new();
        for _ in 0..NUM_CPUS {
            cpu_hands.push(deck.deal(5));
        }

        let user = Player::new(user_name, PlayerType::User, user_hand);
        let user_partner = Player::new(partner_name, PlayerType::Bot, cpu_hands.remove(0));
        let user_team = Team::new(user_team_name, HashSet::from([Seat::Bottom, Seat::Top]));

        let opponent_1 = Player::new(opp1_name, PlayerType::Bot, cpu_hands.remove(0));
        let opponent_2 = Player::new(opp2name, PlayerType::Bot, cpu_hands.remove(0));
        let opp_team = Team::new(opp_team_name, HashSet::from([Seat::Left, Seat::Right]));

        Game {
            state: GameState::default(),
            user_team,
            opp_team,
            players: PlayerMap::from([
                (Seat::Bottom, user),
                (Seat::Top, user_partner),
                (Seat::Left, opponent_1),
                (Seat::Right, opponent_2),
            ]),
            deck,
            current_player_seat: Seat::Bottom,
            dealer_seat: Seat::Bottom,
            leader_seat: Seat::Bottom,
        }
    }

    pub fn get_current_player(&self) -> &Player {
        self.players
            .get(&self.current_player_seat)
            .expect("player not found")
    }

    pub fn get_player_by_seat(&self, seat: Seat) -> &Player {
        self.players.get(&seat).expect("player not found")
    }

    pub fn next_turn(&mut self) {
        self.current_player_seat = self.current_player_seat.next();
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new(
            "User".to_string(),
            "Partner".to_string(),
            "Opponent 1".to_string(),
            "Opponent 2".to_string(),
            "Good Guys".to_string(),
            "Bad Guys".to_string(),
        )
    }
}
