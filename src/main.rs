mod gamelogic;
use crate::gamelogic::*;

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ \n\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet");
    
    let cpu_opponent_guess: u32 = setup::rng_target_code();
    let round_counter: u8 = 1;
}
