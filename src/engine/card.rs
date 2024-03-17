use rand::seq::SliceRandom;

// card color repr
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SuitColor {
    Red,
    Black,
}

// card suit repr
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
    _None,
}

impl Suit {
    pub fn get_symbol(self) -> char {
        match self {
            Suit::Hearts => '♥',
            Suit::Diamonds => '♦',
            Suit::Clubs => '♣',
            Suit::Spades => '♠',
            _ => '_',
        }
    }

    pub fn get_name(self) -> String {
        match self {
            Suit::Hearts => "Hearts".to_string(),
            Suit::Diamonds => "Diamonds".to_string(),
            Suit::Clubs => "Clubs".to_string(),
            Suit::Spades => "Spades".to_string(),
            _ => "None".to_string(),
        }
    }

    pub fn get_color(self) -> SuitColor {
        match self {
            Suit::Hearts | Suit::Diamonds => SuitColor::Red,
            _ => SuitColor::Black,
        }
    }
}

// card face repr
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Face {
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Face {
    pub fn get_symbol(self) -> char {
        match self {
            Face::Nine => '9',
            Face::Ten => 'T',
            Face::Jack => 'J',
            Face::Queen => 'Q',
            Face::King => 'K',
            Face::Ace => 'A',
        }
    }

    pub fn get_name(self) -> String {
        match self {
            Face::Nine => "Nine".to_string(),
            Face::Ten => "Ten".to_string(),
            Face::Jack => "Jack".to_string(),
            Face::Queen => "Queen".to_string(),
            Face::King => "King".to_string(),
            Face::Ace => "Ace".to_string(),
        }
    }

    pub fn get_rank(self) -> u8 {
        match self {
            Face::Nine => 9,
            Face::Ten => 10,
            Face::Jack => 11,
            Face::Queen => 12,
            Face::King => 13,
            Face::Ace => 14,
        }
    }
}

// card repr
#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub face: Face,
    pub suit: Suit,
}

impl Card {
    fn new(face: Face, suit: Suit) -> Self {
        Card { face, suit }
    }

    pub fn get_name(self) -> String {
        let suit_symbol = self.suit.get_symbol();
        let face_symbol = self.face.get_symbol();
        face_symbol.to_string() + " of " + &suit_symbol.to_string()
    }

    pub fn get_color(self) -> SuitColor {
        self.suit.get_color()
    }

    /// Return the rank of the card relative to the given high and lead suits
    ///
    /// # Arguments
    ///
    /// * `high_suit` - The high suit of the hand
    /// * `lead_suit` - The suit of the leading card
    ///
    /// # Returns
    ///
    /// The relative rank of the card
    pub fn get_rank(self, high_suit: Suit, lead_suit: Suit) -> u8 {
        let rank = self.face.get_rank();

        // card is left bower
        if self.is_left_bower(high_suit) {
            return rank + 15; // 26
        }
        match self.suit {
            s if s == high_suit => {
                match self.face {
                    Face::Nine | Face::Ten => {
                        rank + 12 // 21, 22
                    }
                    Face::Queen | Face::King | Face::Ace => {
                        rank + 11 // 23, 24, 25
                    }
                    Face::Jack => rank + 16, // 27 (right bower)
                }
            }
            s if s == lead_suit => rank + 6, // 15, 16, 17, 18, 19, 20
            _ => rank,                       // 9, 10, 11, 12, 13, 14
        }
    }

    pub fn is_left_bower(self, high_suit: Suit) -> bool {
        high_suit != Suit::_None
            && self.face == Face::Jack
            && self.get_color() == high_suit.get_color()
            && self.suit != high_suit
    }
}

#[derive(Debug, Clone, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &face in &[
                Face::Nine,
                Face::Ten,
                Face::Jack,
                Face::Queen,
                Face::King,
                Face::Ace,
            ] {
                cards.push(Card::new(face, suit));
            }
        }
        cards.shuffle(&mut rand::thread_rng());
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn deal(&mut self, num_cards: usize) -> Vec<Card> {
        self.cards.drain(0..num_cards).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_rank() {
        let high_suit = Suit::Spades;
        let lead_suit = Suit::Diamonds;

        assert!(Card::new(Face::Nine, Suit::Spades).get_rank(high_suit, lead_suit) == 21);
        assert!(Card::new(Face::Ten, Suit::Spades).get_rank(high_suit, lead_suit) == 22);
        assert!(Card::new(Face::Jack, Suit::Spades).get_rank(high_suit, lead_suit) == 27);
        assert!(Card::new(Face::Queen, Suit::Spades).get_rank(high_suit, lead_suit) == 23);
        assert!(Card::new(Face::King, Suit::Spades).get_rank(high_suit, lead_suit) == 24);
        assert!(Card::new(Face::Ace, Suit::Spades).get_rank(high_suit, lead_suit) == 25);

        assert!(Card::new(Face::Nine, Suit::Clubs).get_rank(high_suit, lead_suit) == 9);
        assert!(Card::new(Face::Ten, Suit::Clubs).get_rank(high_suit, lead_suit) == 10);
        assert!(Card::new(Face::Jack, Suit::Clubs).get_rank(high_suit, lead_suit) == 26);
        assert!(Card::new(Face::Queen, Suit::Clubs).get_rank(high_suit, lead_suit) == 12);
        assert!(Card::new(Face::King, Suit::Clubs).get_rank(high_suit, lead_suit) == 13);
        assert!(Card::new(Face::Ace, Suit::Clubs).get_rank(high_suit, lead_suit) == 14);

        assert!(Card::new(Face::Nine, Suit::Diamonds).get_rank(high_suit, lead_suit) == 15);
        assert!(Card::new(Face::Ten, Suit::Diamonds).get_rank(high_suit, lead_suit) == 16);
        assert!(Card::new(Face::Jack, Suit::Diamonds).get_rank(high_suit, lead_suit) == 17);
        assert!(Card::new(Face::Queen, Suit::Diamonds).get_rank(high_suit, lead_suit) == 18);
        assert!(Card::new(Face::King, Suit::Diamonds).get_rank(high_suit, lead_suit) == 19);
        assert!(Card::new(Face::Ace, Suit::Diamonds).get_rank(high_suit, lead_suit) == 20);

        assert!(Card::new(Face::Nine, Suit::Hearts).get_rank(high_suit, lead_suit) == 9);
        assert!(Card::new(Face::Ten, Suit::Hearts).get_rank(high_suit, lead_suit) == 10);
        assert!(Card::new(Face::Jack, Suit::Hearts).get_rank(high_suit, lead_suit) == 11);
        assert!(Card::new(Face::Queen, Suit::Hearts).get_rank(high_suit, lead_suit) == 12);
        assert!(Card::new(Face::King, Suit::Hearts).get_rank(high_suit, lead_suit) == 13);
        assert!(Card::new(Face::Ace, Suit::Hearts).get_rank(high_suit, lead_suit) == 14);
    }

    #[test]
    fn test_card_rank_nones() {
        let high_suit = Suit::_None;
        let lead_suit = Suit::_None;

        assert!(Card::new(Face::Nine, Suit::Spades).get_rank(high_suit, lead_suit) == 9);
        assert!(Card::new(Face::Ten, Suit::Spades).get_rank(high_suit, lead_suit) == 10);
        assert!(Card::new(Face::Jack, Suit::Spades).get_rank(high_suit, lead_suit) == 11);
        assert!(Card::new(Face::Queen, Suit::Spades).get_rank(high_suit, lead_suit) == 12);
        assert!(Card::new(Face::King, Suit::Spades).get_rank(high_suit, lead_suit) == 13);
        assert!(Card::new(Face::Ace, Suit::Spades).get_rank(high_suit, lead_suit) == 14);

        assert!(Card::new(Face::Nine, Suit::Clubs).get_rank(high_suit, lead_suit) == 9);
        assert!(Card::new(Face::Ten, Suit::Clubs).get_rank(high_suit, lead_suit) == 10);
        assert!(Card::new(Face::Jack, Suit::Clubs).get_rank(high_suit, lead_suit) == 11);
        assert!(Card::new(Face::Queen, Suit::Clubs).get_rank(high_suit, lead_suit) == 12);
        assert!(Card::new(Face::King, Suit::Clubs).get_rank(high_suit, lead_suit) == 13);
        assert!(Card::new(Face::Ace, Suit::Clubs).get_rank(high_suit, lead_suit) == 14);

        assert!(Card::new(Face::Nine, Suit::Diamonds).get_rank(high_suit, lead_suit) == 9);
        assert!(Card::new(Face::Ten, Suit::Diamonds).get_rank(high_suit, lead_suit) == 10);
        assert!(Card::new(Face::Jack, Suit::Diamonds).get_rank(high_suit, lead_suit) == 11);
        assert!(Card::new(Face::Queen, Suit::Diamonds).get_rank(high_suit, lead_suit) == 12);
        assert!(Card::new(Face::King, Suit::Diamonds).get_rank(high_suit, lead_suit) == 13);
        assert!(Card::new(Face::Ace, Suit::Diamonds).get_rank(high_suit, lead_suit) == 14);

        assert!(Card::new(Face::Nine, Suit::Hearts).get_rank(high_suit, lead_suit) == 9);
        assert!(Card::new(Face::Ten, Suit::Hearts).get_rank(high_suit, lead_suit) == 10);
        assert!(Card::new(Face::Jack, Suit::Hearts).get_rank(high_suit, lead_suit) == 11);
        assert!(Card::new(Face::Queen, Suit::Hearts).get_rank(high_suit, lead_suit) == 12);
        assert!(Card::new(Face::King, Suit::Hearts).get_rank(high_suit, lead_suit) == 13);
        assert!(Card::new(Face::Ace, Suit::Hearts).get_rank(high_suit, lead_suit) == 14);
    }
}
