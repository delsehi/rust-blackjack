use super::*;

pub fn deal_players(deck: &mut Deck, dealer: &mut Player, player: &mut Player) {
    player.deal_card(deck.get_card());
    dealer.deal_card(deck.get_card());
    player.deal_card(deck.get_card());
}

pub fn get_score(player: &Player) -> u8 {
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
            _ => {
            }
        }
    }
    for card in hand.iter() {
        match &card.rank {
            card::Rank::Ace => {
                if result + 11 > 21 {
                    result += 1;
                } else {
                    result += 11;
                }
            },
            _ => {}
        }
    }
    result
}

#[test]
fn queen_and_ace_is21() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clovers, card::Rank::Ace));
    player.deal_card(Card::new(card::Suit::Clovers, card::Rank::Queen));
    assert_eq!(21, get_score(&mut player));
}
#[test]
fn seven_and_ace_is18() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clovers, card::Rank::Seven));
    player.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ace));
    assert_eq!(18, get_score(&mut player));
}
#[test]
fn seven_ace_and_knight_is18() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clovers, card::Rank::Seven));
    player.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ace));
    player.deal_card(Card::new(card::Suit::Tiles, card::Rank::Knight));
    assert_eq!(18, get_score(&mut player));
}
#[test]
fn  five_aces_is15() {
    let mut player = Player::new("Test");
    for _num in 0..5 {
        player.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ace));
    }
    assert_eq!(15, get_score(&mut player));
}