use std::io;
use super::*;

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Whoops something went wrong");
    println!("You said: {}", input);
    input
}

pub fn display_playerhand(player: &str, hand: &Vec<Card>, score: u8) {
    println!("{} \n=============", player);
    for card in hand.iter() {
        println!("{}", card)
    }
    println!("Score: {}", score)
}
