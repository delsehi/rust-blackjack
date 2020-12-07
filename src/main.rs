mod card;
mod deck;
use deck::*;
use card::*;


fn main() {
    let a = Card::new(Suit::Hearts, Rank::Queen);
    let mut deck = Deck::new();
    for num in 0..51 {
        println!("{}", deck.get_card_at(num));
    }
}

