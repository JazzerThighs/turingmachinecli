use crate::game_logic::game_variants::*;
use core::borrow;
use rand::{rngs::ThreadRng, Rng};
use std::{collections::HashMap, io, ops::RangeInclusive, path::Iter};

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

pub fn set_game_parameters() -> (u32, u32, char, char, Gamemode, Difficulty) {
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
            "1" | "2" | "3" | "4" => input.trim().chars().next().expect("empty input"),
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
            "2" | "3" | "4" | "5" => input.trim().chars().next().expect("empty input"),
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

    let mut difficulty: Difficulty;
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

    return (min_code, max_code, min_digit, max_digit, mode, difficulty);
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

pub struct TuringCodeResults {
    // Structure to pair every individual Turing Code with an array of booleans as it is put through every Test on every Criteria Card.
    // This is replacing the pre-calculated Punch Cards used for querying the Turing Machine's Verifier Cards.
    pub code: u32,
    pub checks: Vec<(u8, bool)>, // Vec<(criteria_card_num, pass_fail)>
}

pub fn generate_number_pool(
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
) -> Vec<TuringCodeResults> {
    // Puts every Turing Code from the generate_number_pool() function through every Criteria Card's multiple Tests, and returns the resulting Vector of Structs.

    let codes: Vec<u32> = generate_number_pool(min_code, max_code, min_digit, max_digit);
    let mut results_matrix: Vec<TuringCodeResults> = vec![];

    for code in codes.iter() {
        match (min_code, max_code, min_digit, max_digit) {
            (111, 555, '1', '5') => results_matrix
                .push(len3_min1_max5::criteria_card_tests::evaluate_criteria_results(code.clone())),
            _ => {}
        }
    }

    return results_matrix;
}

pub fn generate_random_puzzle_code(code_length: u32, min_digit: char, max_digit: char) -> u32 {
    // returns a random valid Target Code that is the solution to the Puzzle that will be generated.

    let mut target_code: u32 = 0;
    let mut rng: ThreadRng = rand::thread_rng();

    for _ in 1..=code_length {
        target_code *= 10;
        target_code += rng.gen_range(min_digit..=max_digit) as u32;
    }

    return target_code;
}

pub fn generate_test_index_from_range(
    vec_checks: &Vec<(u8, bool)>,
    test_pool: RangeInclusive<usize>,
    used_cards: &Vec<u8>,
    banned_tests: &Vec<usize>,
) -> usize {
    //returns a random test index where the bool is true for the given target code.

    let mut test_index: usize = 0;
    let mut rng: ThreadRng = rand::thread_rng();
    loop {
        let test_pool_loop = test_pool.clone();
        test_index = rng.gen_range(test_pool_loop);
        if vec_checks[test_index].1
            && !used_cards.contains(&vec_checks[test_index].0)
            && !banned_tests.contains(&test_index)
        {
            // The test_index must have a true result, 
            // AND the test_index cannot be from a card that has already had a test pulled from it,
            // AND the test_index cannot be coupled to previously added tests.
            return test_index;
        }
    }
}

fn generate_unique_test_list(matrix: &Vec<TuringCodeResults>) -> Vec<usize> {
    // returns a list of every test from the various Criteria Cards for which only a single Turing Code passes.
    // The purpose of this is to ensure that no Criteria Test renders any of the other Tests in the Puzzle superfluous.

    let mut counts: HashMap<usize, u32> = HashMap::new();

    for turing_code_result in matrix {
        for (index, (_, value)) in turing_code_result.checks.iter().enumerate() {
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

fn generate_coupled_criteria(matrix: &Vec<TuringCodeResults>) -> Vec<Vec<usize>> {
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
    pub tests: Vec<Vec<usize>>,
}

pub fn generate_puzzle(
    min_code: u32,
    max_code: u32,
    matrix: &Vec<TuringCodeResults>,
    mode: Gamemode,
    difficulty: Difficulty,
    target_code: u32,
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
    let code_length: usize = min_code.to_string().len();
    let test_amount: u8 = match (min_code, max_code, code_length) {
        (111, 555, 3) => match difficulty {
            Difficulty::Easy => 4,
            Difficulty::Standard => 5,
            Difficulty::Hard => 6,
        },
        _ => 6,
    };
    let test_pool: RangeInclusive<usize> = match (min_code, max_code, code_length) {
        (111, 555, 3) => {
            match mode {
                Gamemode::ClassicMode => {
                    match difficulty {
                        Difficulty::Easy => 0..=71,
                        Difficulty::Standard => 0..=71,
                        Difficulty::Hard => 72..=matrix[0].checks.len(), // After (test_amount / 2, or half) tests have been selected from this range, this Range changes to 0..=matrix[0].checks.len()
                    }
                }
                Gamemode::ExtremeMode => 72..=matrix[0].checks.len(), // After (test_amount / 2, or half) tests have been selected from this range, this Range changes to 0..=matrix[0].checks.len()
                Gamemode::NightmareMode => 72..=matrix[0].checks.len(), // After (test_amount / 2, or half) tests have been selected from this range, this Range changes to 0..=matrix[0].checks.len()
            }
        }
        _ => 0..=matrix[0].checks.len(),
    };

    let banned_tests: Vec<usize> = vec![];
    let used_cards: Vec<u8> = vec![];

    let puzzle: Puzzle = Puzzle {
        target_code: target_code,
        tests: vec![Vec::new(); test_amount as usize],
    };
    return puzzle;
}

//  TODO: Generate the Puzzle by taking the generated random target_code, then selecting at least 4 tests which follow the schematic of the game, and setting up the user to play.
//      PUZZLE GENERATION:
//
//          -Ensure that the test which has been selected hasn't had it's criteria card number added to the list of taken criteria cards.
//
//          -Ensure that the test which has been selected is not included in any one of the previous tests' vector of coupled tests (!vec_test_couplings[x].includes(matrix[target_index].checks[x].0)).
//
//          If (the number of tests added < the number of tests specified for the puzzle) {
//              for however many tests have been added to the puzzle's test list, make sure that the collection of tests chosen are not uniquely truthy for that one code.
//          } Else (the number of tests added == the number of tests specified for the puzzle) {
//              ensure that the collection of tests is uniquely truthy for that one target_code, otherwise replace the last test.
//          }
