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

    pub fn get_name(self) -> String {
        let suit_symbol = match self.suit {
            Suit::Hearts => '♥',
            Suit::Diamonds => '♦',
            Suit::Clubs => '♣',
            Suit::Spades => '♠',
        };
        let rank_symbol = match self.rank {
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        };
        return rank_symbol.to_string() + " of " + &suit_symbol.to_string();
    }
}
