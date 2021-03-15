use super::*;

pub struct Player {
    pub name: String,
    hand: Vec<Card>,
}

pub struct Dealer {
    pub name: String,
    hand: Vec<Card>,
}

pub trait Person {
    //  fn new(name: &str) -> Self;
    fn deal_card(&mut self, card: Card);
    fn get_hand(&self) -> &Vec<Card>;
    fn next_move(&self) -> Reply;
    fn get_name(&self) -> &str;
}
impl Dealer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            hand: Vec::<Card>::new(),
        }
    }
}

impl Person for Dealer {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn deal_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    fn next_move(&self) -> Reply {
        Reply::AskUI
    }
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            hand: Vec::<Card>::new(),
        }
    }
}

impl Person for Player {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn deal_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    fn next_move(&self) -> Reply {
        Reply::AskUI
    }
}

pub enum Reply {
    Hit,
    Stand,
    Split,
    AskUI,
}
