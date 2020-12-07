use std::fmt;

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

#[derive(Debug)]
#[allow(unused_variables, dead_code)]
pub enum Suit {
    Hearts, 
    Tiles, 
    Clovers, 
    Pikes
}
#[derive(Debug)]
#[allow(unused_variables, dead_code)]
pub enum Rank {
    Ace, Two, Three, Four, Five, Six, Seven, 
    Eight, Nine, Ten, Knight, Queen, King
}

