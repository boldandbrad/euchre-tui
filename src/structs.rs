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
pub struct Hand {
    pub cards: Vec<Card>,
}

// Define the game state
pub struct GameState {
    pub current_screen: CurrentScreen,
    pub player_hand: Hand,
    pub cpu_hands: Vec<Hand>,
}

pub enum CurrentScreen {
    Title,
    GameTable,
    Settings,
}
