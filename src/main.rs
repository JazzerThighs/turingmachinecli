#![allow(dead_code)]

mod game_logic;
use crate::game_logic::{cpu_player::*, round_loop::*, *};
use clearscreen::*;

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");
    // All of the functions simply set up a standard game of "Turing Machine," but also allows the player to set varying parameters for the game itself, such as the minimum digit of the codes, maximum digit, the length of the codes themselves, and the Criteria Cards available to the Puzzle-Generation algorithm (Alternative Sets of Criteria Cards for differing parameters need to be hard-coded in their own files, and implemented in the several match statements within the codebase).
    let mp: MachineParams = set_game_parameters();
    let (matrix, machine, cards) = generate_results_matrix(&mp);
    let puzzle: Puzzle = generate_puzzle(matrix, machine, cards, &mp);
    let mut verifiers: Vec<Verifier> = generate_verifiers(&puzzle, &mp);
    clear().unwrap();
    // println!("Solution: {}", *puzzle.target_code);
    println!("{puzzle}");
    let mut solution_pool: Vec<Code> = vec![];
    for (_, tce) in puzzle.matrix.iter().enumerate() {
        let code = &tce.code;
        if all_cards_matched(&code, &puzzle) {
            solution_pool.push(code.clone());
        }
    }
    // for i in solution_pool {
    //     println!("{}", *i);
    // }
    play_game(&puzzle, &mut verifiers, &mp);
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
