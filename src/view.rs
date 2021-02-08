use super::*;
use std::io;
use std::{thread, time};

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Whoops something went wrong");
    print!("{}[2J", 27 as char);
    input
}

pub fn display_playerhand(player: &str, hand: &Vec<Card>, score: game::Score) {
    println!("{} \n=============", player);
    for card in hand.iter() {
        println!("{}", card)
    }
    println!("Score: {}\n", score)
}

pub fn player_wants_to_hit() -> bool {
    let res = get_input("Do you want to hit? \n 'y' for yes and 'n' for no.");

    if res.contains("y") {
        return true;
    } else if res.contains("n") {
        return false;
    } else {
        println!("What? Try again.\n");
        player_wants_to_hit()
    }
}

pub fn announce_winner(name: &str, score: game::Score) {
    println!("The winner is....");
    thread::sleep(time::Duration::from_millis(1500));
    println!("{} with a score of {}!\n", name, score);
}

pub fn play_again() -> bool {
    let res = get_input("Play again?\n 'y' for yes and 'n' for no.");
    if res.contains("y") {
        return true;
    } else if res.contains("n") {
        return false;
    } else {
        println!("What? Try again.\n");
        play_again()
    }
}

pub fn announce_dealing(card: &Card, name: &str) {
    print!("{}[2J", 27 as char);
    println!("Dealing card...");
    thread::sleep(time::Duration::from_millis(1500));
    print!("{}[2J", 27 as char);
    println!("Dealing card {} to {}.\n\n", card, name);
    thread::sleep(time::Duration::from_millis(1500));
    print!("{}[2J", 27 as char);
}
