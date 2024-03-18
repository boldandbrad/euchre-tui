use crate::engine::{
    card::Deck,
    player::{Player, PlayerType},
    table::Seat,
    team::Team,
    PlayerMap,
};
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

// const WINNING_SCORE: u8 = 10;

#[derive(Default)]
pub enum GameState {
    #[default]
    PickingDealer,
    DealingHand,
    CallingPickup,
    CallingHighSuit,
    PlayingHand,
}

impl GameState {
    pub fn next(&self) -> Self {
        match self {
            GameState::PickingDealer => GameState::DealingHand,
            GameState::DealingHand => GameState::CallingPickup,
            GameState::CallingPickup => GameState::CallingHighSuit,
            GameState::CallingHighSuit => GameState::PlayingHand,
            GameState::PlayingHand => GameState::PickingDealer,
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            GameState::PickingDealer => write!(f, "Picking Dealer"),
            GameState::DealingHand => write!(f, "Dealing Hand"),
            GameState::CallingPickup => write!(f, "Calling Pickup"),
            GameState::CallingHighSuit => write!(f, "Calling High Suit"),
            GameState::PlayingHand => write!(f, "Playing Hand"),
        }
    }
}

// Define the game state
pub struct Game {
    pub state: GameState,
    // TODO: should teams be in an array or set to make for easy iteration?
    pub user_team: Team,
    pub opp_team: Team,
    pub players: PlayerMap,
    pub deck: Deck,
    pub current_player_seat: Seat,
    pub dealer_seat: Seat,
    pub leader_seat: Seat,
    pub hand_num: u8,
}

impl Game {
    // Initialize the game state with a specified number of CPU players
    pub fn new(
        user_name: String,
        partner_name: String,
        opp1_name: String,
        opp2_name: String,
        user_team_name: String,
        opp_team_name: String,
    ) -> Self {
        // create and shuffle the deck
        let deck = Deck::new();

        // create teams
        let user_team = Team::new(user_team_name, HashSet::from([Seat::Bottom, Seat::Top]));
        let opp_team = Team::new(opp_team_name, HashSet::from([Seat::Left, Seat::Right]));

        // create player/seat map
        let players = PlayerMap::from([
            (Seat::Bottom, Player::new(user_name, PlayerType::User)),
            (Seat::Top, Player::new(partner_name, PlayerType::Bot)),
            (Seat::Left, Player::new(opp1_name, PlayerType::Bot)),
            (Seat::Right, Player::new(opp2_name, PlayerType::Bot)),
        ]);

        // using ..Default::default() here will not work. Causes stack overflow. Idk why
        Game {
            state: GameState::default(),
            user_team,
            opp_team,
            players,
            deck,
            current_player_seat: Seat::default(),
            dealer_seat: Seat::default(),
            leader_seat: Seat::default(),
            hand_num: 0,
        }
    }

    pub fn current_player(&self) -> &Player {
        self.players
            .get(&self.current_player_seat)
            .expect("player not found")
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        self.players
            .get_mut(&self.current_player_seat)
            .expect("player not found")
    }

    pub fn get_player_in_seat(&self, seat: Seat) -> &Player {
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
