#![allow(dead_code)]

mod game_logic;
mod exhaustive_tally;
use clearscreen::*;

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");

    // All of the functions simply set up a standard game of "Turing Machine," but also allows the player to set varying parameters for the game itself, such as the minimum digit of the codes, maximum digit, the length of the codes themselves, and the Criteria Cards available to the Puzzle-Generation algorithm (Alternative Sets of Criteria Cards for differing parameters need to be hard-coded in their own files, and implemented in the several match statements within the codebase).
    let (min_code, max_code, min_digit, max_digit, mode, difficulty, test_amount, og_tm_game) =
        game_logic::setup::set_game_parameters();
    let (matrix, machine, cards) =
        game_logic::setup::generate_results_matrix(min_code, max_code, min_digit, max_digit, og_tm_game);
    let puzzle: game_logic::setup::Puzzle = game_logic::setup::generate_puzzle(
        min_code.to_string().len(),
        min_digit,
        max_digit,
        &matrix,
        &mode,
        &difficulty,
        test_amount,
        og_tm_game,
    );


    clear().unwrap();
    let mut vec_card_candidates: Vec<Vec<String>> = vec![vec![]; puzzle.tests.len()];
    let mut vec_batches: Vec<Vec<Vec<String>>> = vec![vec![]; puzzle.tests.len()];
    let mut card_nums: Vec<String> = vec![];
    // println!("Solution: {}", puzzle.target_code);
    for (i, test) in puzzle.tests.iter().enumerate() {
        card_nums.push(matrix[0].checks[*test].0.to_string());
        println!(
            "Section {}: Card: {}",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(i % 26).unwrap(),
            &matrix[0].checks[*test].0
        );
        println!(
            "Card {} Critera:\n This Verifier verifies... {}",
            &matrix[0].checks[*test].0,
            cards[&matrix[0].checks[*test].0.to_string()].join("\n")
        );
    }

    let any_tests_positive = |card: &String, code: &String| machine[card.parse::<usize>().unwrap() - 1][code].iter().filter(|a| **a).count() > 0;
    let all_cards_matched = |code: &String| card_nums.iter().all(|card| any_tests_positive(card, &code));
    let mut solution_pool: Vec<String> = vec![];
    for (_, tce) in matrix.iter().enumerate() {
        let code = tce.code.to_string();
        if all_cards_matched(&code) {
            solution_pool.push(code.clone());
        }
    }
    for i in solution_pool {
        println!("{i}");
    }
}

//  TODO:
//  - Find out if Standard and Easy difficulty have different Criteria Card picking formulas
//  - Gameplay
//      - Classic Puzzle Gameplay
//      - Nightmare Puzzle Gameplay
//      - Extreme Puzzle Generaton
//          - Extreme Puzzle Gameplay
//  - AI SuperPlayer
//      - Calculates the valid test codes for each card, and the blunder codes for each card
//      - If there is only one valid code for every card that matches up for the whole puzzle, then 0 deductions needed to be made and the answer is clear.
//      - If not, then they need to eliminate the possibilities like a real player and come to an answer that way.
