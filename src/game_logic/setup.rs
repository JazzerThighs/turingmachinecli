use crate::game_logic::game_variants::*;
use rand::{rngs::ThreadRng, Rng};
use std::{
    collections::HashMap,
    io,
    ops::RangeInclusive,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub enum Gamemode {
    ClassicMode,
    ExtremeMode,
    NightmareMode,
}

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Standard,
    Hard,
}

pub fn set_game_parameters() -> (u32, u32, char, char, Gamemode, Difficulty, u8, bool) {
    // This entire function allows the user to set all of the parameters of the Puzzle that will be generated to play.
    // At the moment, only Classic Mode, Original-Parameters are supported.

    let mut min_digit: char;
    let mut max_digit: char;
    loop {
        let mut input = String::new();

        println!(
            "↓ Please input the smallest digit character (In the original game, this is '1')."
        );
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        min_digit = match input.trim() {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                input.trim().chars().next().expect("empty input")
            }
            _ => {
                println!("Invalid smallest digit character \"{}\"", input.trim());
                continue;
            }
        };

        let mut input = String::new();
        println!("↓ Please input the largest digit character (In the original game, this is '5').");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        max_digit = match input.trim() {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                input.trim().chars().next().expect("empty input")
            }
            _ => {
                println!("Invalid largest digit character \"{}\"", input.trim());
                continue;
            }
        };
        if max_digit <= min_digit {
            println!(
                "Largest digit character must be greater than smallest digit character: {} <= {}",
                max_digit, min_digit
            );
            continue;
        }
        break;
    }

    let mut code_length: u8;
    loop {
        let mut input = String::new();
        println!("↓ Please input the number of digits in the valid codes (In the original game, this is 3, resulting in codes ranging from 111 to 555, inclusive).");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        code_length = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid code length entered \"{}\"", input.trim());
                continue;
            }
        };
        match code_length {
            3 => break,
            _ => {
                println!("Chosen length not implemented \"{}\"", code_length);
                continue;
            }
        }
    }

    let mut min_code: u32 = 0;
    let mut max_code: u32 = 0;
    for _ in 1..=code_length {
        min_code *= 10;
        min_code += min_digit.to_digit(10).unwrap();
        max_code *= 10;
        max_code += max_digit.to_digit(10).unwrap();
    }

    let mode: Gamemode;
    loop {
        let mut input = String::new();
        println!("↓ Please input the gamemode setting; Your choices are \"Classic Mode\"(c), \"Extreme Mode\"(e), and \"Nightmare Mode\"(n).");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        mode = match input.trim() {
            "c" => Gamemode::ClassicMode,
            "e" => Gamemode::ExtremeMode,
            "n" => Gamemode::NightmareMode,
            _ => {
                println!("Invalid mode selection \"{}\"", input.trim());
                continue;
            }
        };
        break;
    }

    let difficulty: Difficulty;
    loop {
        let mut input = String::new();
        println!("↓ Please input the difficulty setting; Your choices are \"Easy\"(e), \"Standard\"(s), and \"Hard\"(h).");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        difficulty = match input.trim() {
            "e" => Difficulty::Easy,
            "s" => Difficulty::Standard,
            "h" => Difficulty::Hard,
            _ => {
                println!("Invalid difficulty selection \"{}\"", input.trim());
                continue;
            }
        };
        break;
    }

    let test_amount: u8;
    loop {
        let mut input = String::new();
        println!("↓ Please input the number of sections on the machine that are assigned Criteria Verifiers (In the original game, this is from 4 to 6, inclusive)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        test_amount = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid criteria test amount entered \"{}\"", input.trim());
                continue;
            }
        };
        break;
    }

    let og_tm_game: bool = match (min_code, max_code, code_length) {
        (111, 555, 3) => {
            let mut input = String::new();
            println!(
                "Are you trying to play a game of the Original \"Turing Machine\" board game?"
            );
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim() {
                "y" => true,
                _ => false,
            }
        }
        _ => false,
    };

    return (
        min_code,
        max_code,
        min_digit,
        max_digit,
        mode,
        difficulty,
        test_amount,
        og_tm_game,
    );
}

pub fn is_valid_turing_code(
    min_code: u32,
    max_code: u32,
    min_digit: char,
    max_digit: char,
    test_code: u32,
) -> bool {
    //Simply a reusable statement returning True if the proposed Code is valid as per the parameters passed into the program by the user.

    return (min_code..=max_code).contains(&test_code)
        && test_code
            .to_string()
            .chars()
            .all(|c| c >= min_digit && c <= max_digit);
}

pub struct TuringCodeEval {
    // Structure to pair every individual Turing Code with an array of booleans as it is put through every Test on every Criteria Card.
    // This is replacing the pre-calculated Punch Cards used for querying the Turing Machine's Verifier Cards.
    pub code: u32,
    pub checks: Vec<(u8, bool)>, // Vec<(criteria_card_num, pass_fail)>
}

fn generate_number_pool(
    min_code: u32,
    max_code: u32,
    min_digit: char,
    max_digit: char,
) -> Vec<u32> {
    // returns a list of every valid Turing Code as per the parameters set by the user.

    let mut number_pool: Vec<u32> = vec![];

    for num in min_code..=max_code {
        if is_valid_turing_code(min_code, max_code, min_digit, max_digit, num) {
            number_pool.push(num);
        }
    }

    return number_pool;
}

pub fn generate_results_matrix(
    min_code: u32,
    max_code: u32,
    min_digit: char,
    max_digit: char,
    og_tm_game: bool
) -> Vec<TuringCodeEval> {
    // Puts every Turing Code from the generate_number_pool() function through every Criteria Card's multiple Tests, and returns the resulting Vector of Structs.

    let codes: Vec<u32> = generate_number_pool(min_code, max_code, min_digit, max_digit);
    let mut results_matrix: Vec<TuringCodeEval> = vec![];

    if og_tm_game {
        for code in codes.iter() {
            results_matrix
                .push(og_tm_board_game::criteria_card_tests::evaluate_criteria_results(code.clone()))
        }
    }
    //else {
    //     for code in codes.iter() {
    //         match (min_code, max_code, min_digit, max_digit) {
    //             (111, 555, '1', '5') => results_matrix
    //                 .push(len3_min1_max5::criteria_card_tests::evaluate_criteria_results(code.clone())),
    //             _ => {}
    //         }
    //     }
    // }

    return results_matrix;
}

pub fn generate_random_puzzle_code(code_length: u32, min_digit: char, max_digit: char) -> u32 {
    // returns a random valid Target Code that is the solution to the Puzzle that will be generated.

    let mut target_code: u32 = 0;
    let mut rng: ThreadRng = rand::thread_rng();

    for _ in 1..=code_length {
        target_code *= 10;
        target_code +=
            rng.gen_range(min_digit.to_digit(10).unwrap()..=max_digit.to_digit(10).unwrap()) as u32;
    }

    return target_code;
}

fn generate_unique_test_list(matrix: &Vec<TuringCodeEval>) -> Vec<usize> {
    // returns a list of every test from the various Criteria Cards for which only a single Turing Code passes.
    // The purpose of this is to ensure that no Criteria Test renders any of the other Tests in the Puzzle superfluous.

    let mut counts: HashMap<usize, u32> = HashMap::new();

    for turing_code_eval in matrix {
        for (index, (_, value)) in turing_code_eval.checks.iter().enumerate() {
            if *value {
                *counts.entry(index).or_insert(0) += 1;
            }
        }
    }

    return counts
        .into_iter()
        .filter_map(|(index, count)| if count == 1 { Some(index) } else { None })
        .collect();
}

pub fn print_true_instances(matrix: &[TuringCodeEval]) {
    let mut counts: HashMap<usize, u32> = HashMap::new();
    let mut last_index = 0;

    // Calculate the counts of true values and find the last index
    for turing_code_result in matrix {
        for (index, (_, value)) in turing_code_result.checks.iter().enumerate() {
            if *value {
                *counts.entry(index).or_insert(0) += 1;
            }
            last_index = last_index.max(index); // Ensure we capture the highest index
        }
    }

    // Print the counts for each index, ensuring order from 0 to last_index
    for index in 0..=last_index {
        if let Some(count) = counts.get(&index) {
            let card_number = if let Some((card_number, _)) = matrix[0].checks.get(index) {
                *card_number
            } else {
                0 // Default to 0 if not found, though this case should ideally not occur
            };

            println!(
                "Card {:03} of {:03}, Test {:03} of {:03}: {:04} TRUE Instances.",
                card_number, 
                matrix[0].checks.last().map_or(0, |(num, _)| *num), // Extract the last card number
                index,
                last_index,
                count
            );
        } else {
            // In case there's an index with 0 TRUE instances, we still handle the formatting
            println!(
                "Card {:03} of {:03}, Test {:03} of {:03}: 0 TRUE Instances.", 
                matrix[0].checks.get(index).map_or(0, |(num, _)| *num),
                matrix[0].checks.last().map_or(0, |(num, _)| *num),
                index,
                last_index
            );
        }
    }
}

fn generate_coupled_criteria(matrix: &Vec<TuringCodeEval>) -> Vec<Vec<usize>> {
    // returns a 2D array of Coupled Tests.
    // A test is coupled to another test if for every possible Turing Code, the result of Test A matches the result of Test B.
    // By definition, this renders one of the tests superfluous, and should not be paired with each other in a valid Puzzle.

    let mut vec_test_couplings: Vec<Vec<usize>> = vec![Vec::new(); matrix[0].checks.len()];

    let is_coupled = |x: usize, y: usize| -> bool {
        matrix
            .iter()
            .all(|turing_result| turing_result.checks[x].1 == turing_result.checks[y].1)
    };

    for x in 0..matrix[0].checks.len() {
        for y in 0..matrix[0].checks.len() {
            if x != y && is_coupled(x, y) {
                vec_test_couplings[x].push(y);
            }
        }
    }

    return vec_test_couplings;
}

pub struct Puzzle {
    // WIP struct to contain all of the necessary data for the Puzzle that is generated by the associated algorithm
    // Should be an enum containing all of the different variations of the Game's Puzzles
    pub target_code: u32,
    pub tests: Vec<usize>,
}

fn generate_test_index_from_range(
    vec_checks: &Vec<(u8, bool)>,
    test_pool: RangeInclusive<usize>,
    used_cards: &Vec<u8>,
    banned_tests: &Vec<usize>,
) -> usize {
    //returns a random test index where the bool is true for the given target code.

    let mut rng: ThreadRng = rand::thread_rng();
    loop {
        let test_pool_loop = test_pool.clone();
        let new_test_index: usize = rng.gen_range(test_pool_loop);
        if vec_checks[new_test_index].1
            && !used_cards.contains(&vec_checks[new_test_index].0)
            && !banned_tests.contains(&new_test_index)
        {
            // The test_index must have a true result,
            // AND the test_index cannot be from a card that has already had a test pulled from it,
            // AND the test_index cannot be coupled to previously added tests.
            return new_test_index;
        }
    }
}

fn is_unique_solution(
    target_index: &usize,
    puzzle_tests: &Vec<usize>,
    matrix: &Vec<TuringCodeEval>,
) -> bool {
    // returns true if puzzle_tests argument is a unique set of true booleans among all of the codes.

    for (index, turing_code_result) in matrix.iter().enumerate() {
        let all_true = puzzle_tests
            .iter()
            .all(|&i| turing_code_result.checks.get(i).map_or(false, |&(_, b)| b));

        if all_true && &index != target_index {
            return false;
        }
    }

    return true;
}

pub fn generate_puzzle(
    matrix: &Vec<TuringCodeEval>,
    mode: &Gamemode,
    difficulty: &Difficulty,
    test_amount: u8,
    target_code: u32,
    og_tm_game: bool,
) -> Puzzle {
    let mut target_index: usize = 0;
    for index in 0..matrix.len() {
        if matrix[index].code == target_code {
            target_index = index;
            break;
        }
    }
    let vec_test_couplings: Vec<Vec<usize>> = generate_coupled_criteria(&matrix);
    let vec_unique_tests: Vec<usize> = generate_unique_test_list(&matrix);
    let last_index: usize = matrix[0].checks.len() - 1;
    for unique_banned_test in vec_unique_tests.iter() {
        println!(
            "Banned Test for uniqueness: Card {}/{}, Test {}/{};", 
            matrix[0].checks[*unique_banned_test].0, 
            matrix[0].checks[last_index].0, 
            &unique_banned_test,
            &last_index
        );
    }
    let code_length: usize = matrix[0].code.to_string().len();
    let last_index: usize = matrix[0].checks.len() - 1;
    let mut banned_tests: Vec<usize> = vec_unique_tests.clone();
    let mut used_cards: Vec<u8> = vec![];
    let mut puzzle: Puzzle = Puzzle {
        target_code: target_code,
        tests: vec![],
    };
    let mut tests_added: usize = 0;
    let half_plus_one: u8 = match test_amount % 2 {
        0 => test_amount / 2,
        _ => (test_amount / 2) + 1,
    };
    print!("Generating the puzzle...");
    let timeout: Duration = Duration::new(0, 100_000_000);
    let mut start_time: Instant = Instant::now();
    loop {
        if start_time.elapsed() > timeout {
            println!(
                "Timeout reached. Failed at {} out of {} tests. Resetting puzzle generation...",
                tests_added, test_amount
            );
            tests_added = 0;
            puzzle.tests.clear();
            banned_tests = vec_unique_tests.clone();
            used_cards.clear();
            start_time = Instant::now();
        }
        let mut test_pool: RangeInclusive<usize> = match (&og_tm_game, &difficulty, &mode) {
            (true, _, Gamemode::ExtremeMode) => 0..=last_index.clone(),
            (true, Difficulty::Easy, _) | (true, Difficulty::Standard, _) => 0..=71,
            (true, _, Gamemode::ClassicMode) | (true, _, Gamemode::NightmareMode) => {
                72..=last_index.clone()
            }
            _ => 0..=last_index.clone(),
        };
        if og_tm_game {
            match &difficulty {
                Difficulty::Hard => {
                    if tests_added >= half_plus_one as usize {
                        test_pool = 0..=last_index.clone();
                    };
                }
                _ => {}
            }
        }

        if tests_added < test_amount as usize {
            let new_test_index: usize = generate_test_index_from_range(
                &matrix[target_index].checks,
                test_pool.clone(),
                &used_cards,
                &banned_tests,
            );
            puzzle.tests.push(new_test_index);
            tests_added += 1;

            if tests_added == test_amount as usize {
                if !is_unique_solution(&target_index, &puzzle.tests, matrix) {
                    tests_added -= 1;
                    puzzle.tests.pop();
                } else {
                    println!();
                    break;
                }
            } else {
                tests_added -= 1;

                if is_unique_solution(&target_index, &puzzle.tests, matrix) {
                    puzzle.tests.pop();
                } else {
                    for index in vec_test_couplings[new_test_index].iter() {
                        banned_tests.push(index.clone());
                    }
                    used_cards.push(matrix[target_index].checks[new_test_index].0.clone());
                    tests_added += 1;
                }
            }
        }
    }

    return puzzle;
}
