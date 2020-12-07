mod card;
mod deck;
use deck::*;
use card::*;


fn main() {
    let a = Card::new(Suit::Hearts, Rank::Queen);
    let mut deck = Deck::new();
    deck.add_card(a);
    println!("{}", deck.get_card());
}

