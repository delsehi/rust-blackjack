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

    for card in player.get_hand().iter() {
        println!("Player has: {}", card)
    }

    while game::get_score(&mut player) < 21 {
        let card = deck.get_card();
        println!("Dealing card {}", card);
        player.deal_card(card)
    }

    println!("Score: {}", game::get_score(&mut player))
}
