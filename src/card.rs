use std::fmt;
use strum_macros::EnumIter;

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            suit,
            rank,
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", &self.rank, &self.suit)
    }
}

#[derive(EnumIter, Debug, PartialEq, Clone)]
#[allow(unused_variables, dead_code)]
pub enum Suit {
    Hearts, 
    Tiles, 
    Clovers, 
    Pikes
}
#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
#[allow(unused_variables, dead_code)]
pub enum Rank { // Todo: Refactor so Two = 2, etc and in game use 'Two as u8' instead of that match statement. 
    Ace, Two = 2, Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7, 
    Eight = 8, Nine = 9, Ten = 10, Knight, Queen, King
}

