mod gamelogic;
use crate::gamelogic::*;

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ \n\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet");
    
    let turing_code_pool = setup::generate_number_pool(3, 1, 5);
    let target = setup::rng_target_code(turing_code_pool);
    
}
