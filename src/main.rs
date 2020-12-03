mod card;
use card::*;

fn main() {
    let a = Card::new(Suit::Hearts, Rank::King(13));

    a.print();

    
}

