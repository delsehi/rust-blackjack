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

    let mut deck = Deck::new();
    deck.shuffle();
    let mut player = Player::new("Player 1");
    let mut dealer = Player::new("Dealer");

    game::deal_players(&mut deck, &mut dealer, &mut player);

    view::display_playerhand(&player.name, player.get_hand(), game::get_score(&player));

    while game::get_score(&mut player) < 17 {
        let card = deck.get_card();
        println!("Dealing card {}", card);
        player.deal_card(card)
    }

    println!("Score: {}", game::get_score(&mut player))
}
