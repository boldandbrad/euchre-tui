use crate::structs::{Hand, Seat};

#[derive(Clone)]
pub enum Player {
    Human(Human),
    Bot(Bot),
}

pub trait Playable {
    fn call_pickup() -> bool;
    fn pick_up_card();
    fn call_suit();
    fn play_card();
}

#[derive(Clone)]
pub struct Human {
    pub name: String,
    pub seat: Seat,
    hand: Hand,
}

impl Human {
    pub fn new(name: String, seat: Seat, hand: Hand) -> Self {
        Human { name, seat, hand }
    }
}

#[derive(Clone)]
pub struct Bot {
    pub name: String,
    pub seat: Seat,
    hand: Hand,
}

impl Bot {
    pub fn new(name: String, seat: Seat, hand: Hand) -> Self {
        Bot { name, seat, hand }
    }
}
