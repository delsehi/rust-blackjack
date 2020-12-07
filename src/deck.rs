use super::*;

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn add_card(&mut self, c: Card) {
        self.cards.push(c);
    }
    pub fn new() -> Self {
        Self {
            cards: Vec::new()
        }
    }
    pub fn get_card(&self) -> &Card {
        &self.cards[0]
    }
}