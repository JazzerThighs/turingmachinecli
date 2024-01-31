mod game_logic;
use crate::{game_logic::*, setup::*};

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");

    // All of the functions simply set up a standard game of "Turing Machine," but also allows the player to set varying parameters for the game itself, such as the minimum digit of the codes, maximum digit, the length of the codes themselves, and the Criteria Cards available to the Puzzle-Generation algorithm (Alternative Sets of Criteria Cards for differing parameters need to be hard-coded in their own files, and implemented in the several match statements within the codebase).
    let (min_code, max_code, min_digit, max_digit, mode, difficulty) =
        setup::set_game_parameters();
    println!("Minimum Code: {}, Maximum Code: {},\nSmallest Digit: {}, Largest Digit: {},\nGamemode: {:?}, Difficulty: {:?}", min_code, max_code, min_digit, max_digit, mode, difficulty);
    let matrix: Vec<setup::TuringCodeResults> =
        generate_results_matrix(min_code, max_code, min_digit, max_digit);
    let target_code: u32 =
        generate_random_puzzle_code(min_code.to_string().len() as u32, min_digit, max_digit);
    
    // Assuming all parameters were set to the Original Specifications of the Board Game, 
    // min_digit = '1', max_digit = '5',
    // min_code = 111, max_code = 555;
    // The first Puzzles to be implemented will be exclusively Classic Mode Puzzles.
    let puzzle: Puzzle = generate_puzzle(min_code, max_code, &matrix, mode, difficulty, target_code);
}
