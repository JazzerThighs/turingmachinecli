mod game_logic;
mod exhaustive_tally;
use clearscreen::*;
use setup::*;

use crate::game_logic::*;

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");

    // All of the functions simply set up a standard game of "Turing Machine," but also allows the player to set varying parameters for the game itself, such as the minimum digit of the codes, maximum digit, the length of the codes themselves, and the Criteria Cards available to the Puzzle-Generation algorithm (Alternative Sets of Criteria Cards for differing parameters need to be hard-coded in their own files, and implemented in the several match statements within the codebase).
    let (min_code, max_code, min_digit, max_digit, mode, difficulty, test_amount, og_tm_game) =
        set_game_parameters();
    println!("Minimum Code: {}, Maximum Code: {},\nSmallest Digit: {}, Largest Digit: {},\nGamemode: {:?}, Difficulty: {:?}", min_code, max_code, min_digit, max_digit, mode, difficulty);
    
    let (matrix, machine, cards) =
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
    clear().unwrap();
    println!("Solution: {}", puzzle.target_code);
    for (i, test) in puzzle.tests.iter().enumerate() {
        println!(
            "Section {}: Test: {}, Card: {}",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(i % 26).unwrap(),
            test,
            &matrix[0].checks[*test].0
        );
        println!(
            "Card {} Critera:\n This Verifier verifies... {}",
            &matrix[0].checks[*test].0,
            cards[&matrix[0].checks[*test].0.to_string()].join("\n")
        );
    }

}
//  TODO:
//  - Revamp puzzle generation algo so that it doesn't need to time out, or at least not as often using the methods in the exhaustive count functions
//
//  - Gameplay
//      - Classic Puzzle Gameplay
//      - Nightmare Puzzle Gameplay
//      -
//      - Extreme Puzzle Generaton
//          - Extreme Puzzle Gameplay
//  - Find out if Standard and Easy difficulty have different Criteria Card picking formulas



