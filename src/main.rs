#![allow(dead_code)]
mod card;
mod deck;
mod player;
mod game;
use card::*;
use deck::*;
use player::*;


fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let mut player = Player::new();
    let mut dealer = Player::new();

    game::deal_players(&mut deck, &mut dealer, &mut player);

    for _num in 0..5 {
     //   player.deal_card(deck.get_card())
    }
    for card in player.get_hand().iter() {
        println!("Card: {}", card)
    }

    println!("Score: {}", game::get_score(&mut player))
}
