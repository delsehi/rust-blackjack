use super::*;

pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Self {
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
