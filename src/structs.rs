use crate::game::team::Team;

// Define card suits
#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

// Define card ranks
#[derive(Debug, Copy, Clone)]
pub enum Rank {
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

// Define a card
#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    // fn new(rank: Rank, suit: Suit) -> Self {
    //     Card { rank, suit }
    // }
}

// Define a player's hand
#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

// Define the game state
#[derive(Clone)]
pub struct GameState {
    pub current_screen: CurrentScreen,
    pub user_team: Team,
    pub opposing_team: Team,
    pub deck: Vec<Card>,
}

#[derive(Clone)]
pub enum CurrentScreen {
    Title,
    GameTable,
    Settings,
}

#[derive(Clone)]
pub enum Seat {
    Bottom,
    Left,
    Top,
    Right,
}
