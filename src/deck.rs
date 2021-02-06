use rand::thread_rng;
use super::*;
use strum::IntoEnumIterator;
use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<Card>, 
}

impl Deck {
    pub fn add_card(&mut self, c: Card) {
        self.cards.push(c);
    }
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit.clone(), rank.clone()))
            }
        }
        cards.push(Card::new(Suit::Clovers, Rank::Ace));
        Self {
            cards
        }
    }
    pub fn get_clone_from(&self, index: usize) -> Option<Card> {
        if self.cards.get(index).is_none() {
            None
        } else {
            Some(self.cards[index].clone())
        }
        
    }
    pub fn get_card(&mut self) -> Card {
        let card = self.cards.pop();
        match card {
            None => panic!("Not enough cards in deck."),
            Some(card) => card
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        &self.cards.shuffle(&mut rng);
    }
}