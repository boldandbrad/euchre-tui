use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Seat {
    Bottom,
    Left,
    Top,
    Right,
}

impl Seat {
    pub fn next(&self) -> Self {
        match self {
            Seat::Bottom => Seat::Left,
            Seat::Left => Seat::Top,
            Seat::Top => Seat::Right,
            Seat::Right => Seat::Bottom,
        }
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Seat::Bottom => write!(f, "Bottom"),
            Seat::Left => write!(f, "Left"),
            Seat::Top => write!(f, "Top"),
            Seat::Right => write!(f, "Right"),
        }
    }
}

// TODO: implement table struct that contains played cards, deck, kitty, etc
pub struct Table {}
