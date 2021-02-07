#![allow(dead_code)]
mod card;
mod deck;
mod player;
mod game;
mod view;
use card::*;
use deck::*;
use player::*;


fn main() {
    let mut deck = Deck::new(); // Create a new deck
    deck.shuffle(); // Shuffle it. 

    // Create some players. 
    let mut player = Player::new("Player 1");
    let mut dealer = Player::new("Dealer");

    game::deal_players(&mut deck, &mut dealer, &mut player); // Give the players their initial cards.

    view::display_playerhand(&dealer.name, dealer.get_hand(), game::get_score(&dealer)); 
    view::display_playerhand(&player.name, player.get_hand(), game::get_score(&player)); 

    while view::player_wants_to_hit() {
        let card = deck.get_card();
        println!("Dealing card {} to {}.", card, dealer.name);
        dealer.deal_card(card)
    }

    while game::get_score(&dealer) < 17 {
        let card = deck.get_card();
        println!("Dealing card {} to {}.", card, dealer.name);
        dealer.deal_card(card)
    }

    println!("Score: {}", game::get_score(&dealer))
}
