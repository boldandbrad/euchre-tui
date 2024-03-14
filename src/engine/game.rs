use crate::structs::{Card, Hand, Rank, Seat, Suit};
use crate::table::{
    player::{Bot, Human, Player},
    team::Team,
};
use rand::seq::SliceRandom;

const NUM_CPUS: usize = 3;

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
    pub deck: Vec<Card>,
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
        let mut deck = Vec::new();
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ] {
                deck.push(Card { rank, suit });
            }
        }
        // Shuffle the deck
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);

        // Deal hands to player and CPU players
        let player_hand = Hand {
            cards: deck.drain(0..5).collect(),
        };
        let mut cpu_hands = Vec::new();
        for _ in 0..NUM_CPUS {
            cpu_hands.push(Hand {
                cards: deck.drain(0..5).collect(),
            });
        }

        let user = Human::new(user_name, Seat::Bottom, player_hand);
        let user_partner = Bot::new(partner_name, Seat::Top, cpu_hands.remove(0));
        let user_team = Team::new(
            user_team_name,
            vec![Player::Human(user), Player::Bot(user_partner)],
        );

        let opponent_1 = Bot::new(opp1_name, Seat::Left, cpu_hands.remove(0));
        let opponent_2 = Bot::new(opp2name, Seat::Right, cpu_hands.remove(0));
        let opposing_team = Team::new(
            opp_team_name,
            vec![Player::Bot(opponent_1), Player::Bot(opponent_2)],
        );

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
