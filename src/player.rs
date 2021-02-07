use super::*;

pub struct Player {
    pub name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            hand: Vec::<Card>::new(),
        }
    }
    pub fn deal_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }
}
