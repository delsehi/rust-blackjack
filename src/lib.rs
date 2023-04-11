mod card;
mod deck;
mod game;
mod player;
mod view;
use card::*;
use deck::*;
use game::Score;
use player::*;

pub fn run() {
  loop {
    let mut deck = Deck::new(); // Create a new deck
    deck.shuffle(); // Shuffle it.

    // Create some players.
    let mut player = Player::new("Player 1");
    let mut dealer = Dealer::new("Dealer");

    game::deal_players(&mut deck, &mut dealer, &mut player); // Give the players their initial cards.

    // Display inital hand
    view::display_playerhand(&dealer.name, dealer.get_hand(), game::get_score(&dealer));
    view::display_playerhand(&player.name, player.get_hand(), game::get_score(&player));

    // The player chooses if they want more cards.
    while game::get_score(&player) < game::Score::Points(22) && view::player_wants_to_hit() {
      let card = deck.get_card();

      view::announce_dealing(&card, &player.name);
      player.deal_card(card);
      if game::get_score(&player) == game::Score::Busted {
        view::display_playerhand(&dealer.name, dealer.get_hand(), game::get_score(&dealer));
        view::display_playerhand(&player.name, player.get_hand(), game::get_score(&player));
        break;
      } else if game::get_score(&player) == game::Score::Blackjack {
        view::display_playerhand(&dealer.name, dealer.get_hand(), game::get_score(&dealer));
        view::display_playerhand(&player.name, player.get_hand(), game::get_score(&player));
        break;
      } else {
        view::display_playerhand(&dealer.name, dealer.get_hand(), game::get_score(&dealer));
        view::display_playerhand(&player.name, player.get_hand(), game::get_score(&player));
      }
    }
    // Dealer's turn.
    if game::get_score(&player) != game::Score::Busted
      && game::get_score(&player) != game::Score::Blackjack
    {
      let mut dealer_score = game::get_score(&dealer);
      while dealer_score < Score::Points(17) && dealer_score != Score::Busted {
        let card = deck.get_card();
        view::announce_dealing(&card, &dealer.name);
        dealer.deal_card(card);
        dealer_score = game::get_score(&dealer);
      }
      view::display_playerhand(&dealer.name, dealer.get_hand(), game::get_score(&dealer));
      view::display_playerhand(&player.name, player.get_hand(), game::get_score(&player));
    }

    let winner = game::get_winner(&dealer, &player);
    match winner {
      Some(winner) => view::announce_winner(&winner.get_name(), game::get_score(winner)),
      None => println!("It's a tie!"),
    }

    if !view::play_again() {
      break;
    }
  }
}
