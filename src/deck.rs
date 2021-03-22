use crate::Card;
use crate::Rank;
use crate::Suit;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

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
        Self { cards }
    }
    pub fn get_card(&mut self) -> Card {
        let card = self.cards.pop();
        match card {
            None => panic!("Not enough cards in deck."),
            Some(card) => card,
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        &self.cards.shuffle(&mut rng);
    }
}
