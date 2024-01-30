mod game_logic;
use crate::{game_logic::*, setup::*};

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");

    let (min_code, max_code, min_digit, max_digit, mode, test_amount) =
        setup::set_game_parameters();
    println!("Minimum Code: {}, Maximum Code: {},\nSmallest Digit: {}, Largest Digit: {},\nGamemode: {:?}, Amount of Tests: {}", min_code, max_code, min_digit, max_digit, mode, test_amount);
    let matrix: Vec<setup::TuringCodeResults> =
        generate_results_matrix(min_code, max_code, min_digit, max_digit);
    let target_code: u32 =
        generate_random_puzzle_code(min_code.to_string().len() as u32, min_digit, max_digit);
    let puzzle = generate_puzzle_wrapper(min_code, max_code, &matrix, test_amount, mode, target_code);
}
