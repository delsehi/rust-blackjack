mod card;
mod deck;
mod player;
use deck::*;
use card::*;
use player::*;


fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let mut player = Player::new();
    
    for num in 0..5 {
        player.deal_card(deck.get_card())
    }
    for card in player.get_hand().iter() {
        println!("Card: {}", card)
    }
}

