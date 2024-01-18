mod gamelogic;
use crate::gamelogic::*;

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ \n\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet");
    
    let turing_code_pool: Vec<u32> = setup::generate_number_pool(3, 1, 5);
    let target: u32 = setup::rng_target_code(turing_code_pool);
    let round_num: u8 = 1;
    let guesses: Vec<u32> = vec![];
}
