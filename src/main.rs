mod game_logic;
use crate::game_logic::*;

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");
    
    let puzzle: setup::Puzzle = setup::generate_puzzle();

}

//  TODO:
//  - Gameplay
//      - Classic Puzzle Gameplay
//      - Nightmare Puzzle Gameplay
//      -
//      - Extreme Puzzle Generaton
//          - Extreme Puzzle Gameplay
