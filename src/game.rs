use super::*;
use std::fmt;

pub fn deal_players(deck: &mut Deck, dealer: &mut Player, player: &mut Player) {
    player.deal_card(deck.get_card());
    dealer.deal_card(deck.get_card());
    player.deal_card(deck.get_card());
}

pub fn get_score(player: &Player) -> Score {
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
            _ => {}
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
            }
            _ => {}
        }
    }
    if result > 21 {return Score::Busted}
    Score::Points(result)
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Score {
    Busted,
    Points(u8),
    Blackjack,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Score::Blackjack => write!(f, "{:?}", &self),
            Score::Busted => write!(f, "{:?}", &self),
            Score::Points(num) => write!(f, "{}", num),
        
        }
    }
}

pub fn get_winner<'a>(dealer: &'a Player, player: &'a Player) -> Option<&'a Player> {
    let player_score = get_score(&player); // Get their scores. 
    let dealer_score = get_score(&dealer);
    if player_score == dealer_score {return None};
    if player_score > dealer_score {return Some(player)} else {
        return Some(dealer)
    }
/*
    // If the player has blackjack and the dealer has not 
    if player_score == Score::Blackjack && dealer_score != Score::Blackjack {
        return Some(player); // Player wins. 
    } else if dealer_score == Score::Blackjack && player_score != Score::Blackjack {
        return Some(dealer); // Or dealer if they have blackjack but the player has not.
    }
    if player_score == Score::Busted && dealer_score == Score::Busted {
        return None;
    }; // Both are busted
    
    if dealer_score >= player_score {
        return Some(dealer);
    } else {
        return Some(player);
    }; // Highest wins. Dealer wins on equal. 
    */
}

#[test]
fn queen_and_ace_is21() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clovers, card::Rank::Ace));
    player.deal_card(Card::new(card::Suit::Clovers, card::Rank::Queen));
    assert_eq!(Score::Points(21), get_score(&mut player));
}
#[test]
fn seven_and_ace_is18() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clovers, card::Rank::Seven));
    player.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ace));
    assert_eq!(Score::Points(18), get_score(&mut player));
}
#[test]
fn seven_ace_and_knight_is18() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clovers, card::Rank::Seven));
    player.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ace));
    player.deal_card(Card::new(card::Suit::Tiles, card::Rank::Knight));
    assert_eq!(Score::Points(18), get_score(&mut player));
}
#[test]
fn five_aces_is15() {
    let mut player = Player::new("Test");
    for _num in 0..5 {
        player.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ace));
    }
    assert_eq!(Score::Points(15), get_score(&mut player));
}

#[test]
fn blackjack_wins_over_21() {
    let mut player = Player::new("Test");
    let mut dealer = Player::new("Test2");
    player.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ace));
    player.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ten));

    dealer.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ten));
    dealer.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ten));
    dealer.deal_card(Card::new(card::Suit::Hearts, card::Rank::Ace));
    let winner = game::get_winner(&dealer, &player).unwrap();
    assert_eq!(player.name, winner.name);
}
