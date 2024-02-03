mod game_logic;
use crate::{game_logic::*, setup::*};

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");

    // All of the functions simply set up a standard game of "Turing Machine," but also allows the player to set varying parameters for the game itself, such as the minimum digit of the codes, maximum digit, the length of the codes themselves, and the Criteria Cards available to the Puzzle-Generation algorithm (Alternative Sets of Criteria Cards for differing parameters need to be hard-coded in their own files, and implemented in the several match statements within the codebase).
    let (min_code, max_code, min_digit, max_digit, mode, difficulty, test_amount, og_tm_game) =
        setup::set_game_parameters();
    println!("Minimum Code: {}, Maximum Code: {},\nSmallest Digit: {}, Largest Digit: {},\nGamemode: {:?}, Difficulty: {:?}", min_code, max_code, min_digit, max_digit, mode, difficulty);
    let matrix: Vec<TuringCodeEval> =
        generate_results_matrix(min_code, max_code, min_digit, max_digit, og_tm_game);
    println!("Matrix generated...");
    let target_code: u32 =
        generate_random_puzzle_code(min_code.to_string().len() as u32, min_digit, max_digit);
    let puzzle: Puzzle = generate_puzzle(
        &matrix,
        &mode,
        &difficulty,
        test_amount,
        target_code,
        og_tm_game,
    );
    // println!("Solution: {}", target_code);
    // for test in puzzle.tests.iter() {
    //     println!(
    //         "Test: {}, Card: {}",
    //         test,
    //         &matrix[0].checks[test.clone()].0
    //     );
    // }
    // print_true_instances(&matrix);
}

//  TODO:
//  - Gameplay
//      - Classic Puzzle Gameplay
//      - Nightmare Puzzle Gameplay
//      -
//      - Extreme Puzzle Generaton
//          - Extreme Puzzle Gameplay
//  - Find out if Standard and Easy difficulty have different Criteria Card picking formulas
