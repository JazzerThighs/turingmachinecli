mod game_logic;
use crate::{game_logic::*, setup::*};

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ \n\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.");
    
    let (min_code, max_code, min_digit, max_digit, mode, test_amount) = setup::set_game_parameters();
    println!("Minimum Code: {}, Maximum Code: {},\nSmallest Digit: {}, Largest Digit: {},\nGamemode: {}, Amount of Tests: {}", min_code, max_code, min_digit, max_digit, mode, test_amount);
    let matrix: Vec<setup::TuringCodeResults> = generate_results_matrix(min_code, max_code, min_digit, max_digit);
    let target_code: u32 = generate_random_puzzle_code(min_code.to_string().len() as u32, min_digit, max_digit);
    let target_index: u32 = target_code_index(matrix, target_code);
}
