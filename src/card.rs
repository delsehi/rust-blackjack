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
    pub fn print(&self) {
        println!("I am {:?} of {:?}", &self.rank, &self.suit)
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
    Ace(u8), Two(u8), Three(u8), Four(u8), Five(u8), Six(u8), Seven(u8), 
    Eight(u8), Nine(u8), Ten(u8), Knight(u8), Queen(u8), King(u8)
}

