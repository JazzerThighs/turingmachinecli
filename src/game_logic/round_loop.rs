use crate::game_logic::{Difficulty::*, Gamemode::*, *};
use colored::Colorize;

pub fn play_game(
    puzzle: &mut Puzzle,
    verifiers: &mut Vec<Verifier>,
    mp: &MachineParams,
) {
    let mut score_tally: Vec<usize> = vec![];
    'round_loop: loop {
        // Step 1: Choose if you would like to take a round to test a Turing Code against up to 3 of the Machine Sections, or Go to Step 6.
        'step_one: loop {
            let mut input = String::new();
            println!("Would you like to attempt to solve the puzzle? \"y\" or \"n\"");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim().to_lowercase().as_str() {
                "y" => 'final_guess: loop {
                    // Step 6: Input your guess for the solution to the Puzzle. Both guessing correctly and incorrectly ends the game.
                    println!("What is your guess for the solution to the puzzle?");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    match input.trim() {
                        x if x.parse::<usize>().is_ok()
                            && is_valid_turing_code(mp, Code(x.parse::<usize>().unwrap())) =>
                        {
                            if x.parse::<usize>().unwrap() == *puzzle.target_code {
                                println!("Correct! You Win!");
                            } else {
                                println!(
                                    "Incorrect! The correct solution is {}. You Lose!",
                                    *puzzle.target_code
                                );
                            }
                            return;
                        }
                        _ => {
                            println!("Invalid input \"{}\"", input.trim());
                            continue 'final_guess;
                        }
                    }
                },
                "n" => break 'step_one,
                _ => {
                    println!("Invalid input \"{}\"", input.trim());
                    continue 'step_one;
                }
            }
        }
        // Step 2: Input a valid Turing Code to test against the Machine Sections.
        let test_code: Code;
        'step_two: loop {
            let mut input = String::new();
            println!("Input a valid Turing Code to test against up to three sections of the Turing Machine for this round.");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim() {
                x if x.parse::<usize>().is_ok()
                    && is_valid_turing_code(mp, Code(x.parse::<usize>().unwrap())) =>
                {
                    println!("Code selected: {x}");
                    test_code = Code(x.parse::<usize>().unwrap());
                    break 'step_two;
                }
                _ => {
                    println!("Invalid input \"{}\"", input.trim());
                    continue 'step_two;
                }
            }
        }
        // Step 3: Select up to 3 Machine Sections to test the Turing Code against.
        let mut section_test_count: usize = 0;
        'step_three: loop {
            if section_test_count == 3 {
                break 'step_three;
            }
            let mut input = String::new();
            println!("Select up to {} more section{} to test your Turing Code against, or enter \"done\" to move on.",
                3 - section_test_count,
                if 3 - section_test_count > 1 { "s" } else { "" }
            );
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input.trim() == "done" {
                break 'step_three;
            }
            for i in puzzle.tests.iter() {
                if i.marker == input.trim().to_string() {
                    section_test_count += 1;
                    println!(
                        "Section {} tested against {}: {}",
                        i.marker,
                        *test_code,
                        match puzzle.matrix[*test_code].checks[*i.big_index].passed {
                            true => "TRUE".on_green(),
                            false => "FALSE".on_red(),
                        }
                    );
                    continue 'step_three;
                }
            }
            println!("Invalid section label \"{}\"", input)
        }
        score_tally.push(section_test_count);
        // Step 4: Mark up the cards using the deductions.
        // 'step_four: loop {

        // }
        // Step 5: Go back to Step 1.
    }
}
