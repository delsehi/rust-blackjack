use super::*;

pub fn deal_players(deck: &mut Deck, dealer: &mut Player, player: &mut Player) {
    player.deal_card(deck.get_card());
    dealer.deal_card(deck.get_card());
    player.deal_card(deck.get_card());
}

pub fn get_score(player: &mut Player) -> u8 {
    let hand = &player.get_hand();
    let mut result: u8 = 0;
    for card in hand.iter() {
        match &card.rank {
            card::Rank::Two => result += 2,
            card::Rank::Three => result += 3,
            card::Rank::Four => result += 4,
            card::Rank::Five => result += 5,
            card::Rank::Six => result += 6,
            card::Rank::Seven => result += 7,
            card::Rank::Eight => result += 8,
            card::Rank::Nine => result += 9,
            card::Rank::Ten => result += 10,
            card::Rank::Knight => result += 10,
            card::Rank::Queen => result += 10,
            card::Rank::King => result += 10,
            _ => result += 0,
        }
    }
    result
}