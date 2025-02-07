mod game_logic;
use std::collections::HashMap;
use clearscreen::*;

use crate::game_logic::*;

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");

    // All of the functions simply set up a standard game of "Turing Machine," but also allows the player to set varying parameters for the game itself, such as the minimum digit of the codes, maximum digit, the length of the codes themselves, and the Criteria Cards available to the Puzzle-Generation algorithm (Alternative Sets of Criteria Cards for differing parameters need to be hard-coded in their own files, and implemented in the several match statements within the codebase).
    let (min_code, max_code, min_digit, max_digit, mode, difficulty, test_amount, og_tm_game) =
        setup::set_game_parameters();
    println!("Minimum Code: {}, Maximum Code: {},\nSmallest Digit: {}, Largest Digit: {},\nGamemode: {:?}, Difficulty: {:?}", min_code, max_code, min_digit, max_digit, mode, difficulty);
    
    let (matrix, cards) =
        setup::generate_results_matrix(min_code, max_code, min_digit, max_digit, og_tm_game);
    println!("Matrix generated...");
    // debug_helpers::print_true_instances(&matrix);
    
    let target_code: u32 =
        setup::generate_random_puzzle_code(min_code.to_string().len() as u32, min_digit, max_digit);
    
    let puzzle: setup::Puzzle = setup::generate_puzzle(
        &matrix,
        &mode,
        &difficulty,
        test_amount,
        target_code,
        og_tm_game,
    );
    clear().unwrap();
    println!("Solution: {}", puzzle.target_code);
    for test in puzzle.tests.iter() {
        println!(
            "Test: {}, Card: {}",
            test,
            &matrix[0].checks[*test].0
        );
        println!(
            "Card {} Critera:\n {}",
            &matrix[0].checks[*test].0,
            cards[*test]
        );
    }
    struct CriteriaCard {
        card: u8,
        test: bool
    }
    let mut codemap: HashMap<String, Vec<CriteriaCard>> = HashMap::default();
    for i in matrix.iter() {
        codemap.insert(i.code.to_string(), i.checks.iter().map(|ch| CriteriaCard {card: ch.0, test: ch.1}).collect());
    }
    let mut cardmap: HashMap<String, Vec<String>> = HashMap::default();
    for i in 0..cards.len() {
        let c = cards[i].clone();
        let splits: Vec<&str> = c.split("\n").collect();
        let n: Vec<String> = splits[0..].iter().map(|s| s.to_string().clone()).collect();
        cardmap.insert((i+1).to_string(), n);
    }

    let s1 = "5".to_string();
    let s2 = "312".to_string();

    if cardmap.contains_key(&s1) {
        println!("{s1}: OK!");
    }
    if !cardmap.contains_key(&s2) {
        println!("{s2}: OOPS!");
    }

}
//  TODO:
//  - Gameplay
//      - Classic Puzzle Gameplay
//      - Nightmare Puzzle Gameplay
//      -
//      - Extreme Puzzle Generaton
//          - Extreme Puzzle Gameplay
//  - Find out if Standard and Easy difficulty have different Criteria Card picking formulas



