use std::io;
use super::*;

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Whoops something went wrong");
    input
}

pub fn display_playerhand(player: &str, hand: &Vec<Card>, score: u8) {
    println!("{} \n=============", player);
    for card in hand.iter() {
        println!("{}", card)
    }
    println!("Score: {}\n", score)
}

pub fn player_wants_to_hit() -> bool {
    let res = get_input("Do you want to hit? \n 'y' for yes and 'n' for no.");
    
    if res.contains("y") {return true} 
    else if res.contains("n") {return false}
    else {
        println!("What? Try again.\n");
        player_wants_to_hit()
    }

}

pub fn announce_winner(name: &str, score: u8) {
    println!("The winner is: {} with a score of {}!", name, score);
}