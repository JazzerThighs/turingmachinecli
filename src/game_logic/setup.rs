use crate::game_logic::{game_variants::*, debug_helpers};
use rand::{rngs::ThreadRng, Rng};
use std::{
    collections::HashMap,
    io,
    ops::RangeInclusive,
    // time::{Duration, Instant},
};

#[derive(Debug)]
pub enum Gamemode {
    Classic,
    Extreme,
    Nightmare,
}

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Standard,
    Hard,
}

struct GameParameters {
    pub og_tm_game: bool,
    pub min_code: u32,
    pub max_code: u32,
    pub min_digit: char,
    pub max_digit: char,
    pub mode: Gamemode,
    pub difficulty: Difficulty,
    pub sections: u8,
    pub cards_per_section: u8,
    pub shuffle: bool,
}

#[derive(Clone)]
pub struct TuringCodeEval {
    // Structure to pair every individual Turing Code with an array of booleans as it is put through every Test on every Criteria Card. This is replacing the pre-calculated Punch Cards used for querying the Turing Machine's Verifier Cards.
    pub checks: Vec<(u8, bool)>, // Vec<(criteria_card_number, pass_or_fail)>
    pub code: u32,
}

#[derive(Clone)]
pub struct Puzzle {
    // Struct to contain all of the necessary data for the Puzzle that is generated by the associated algorithm
    pub matrix: Vec<TuringCodeEval>,
    pub tests: Vec<usize>,
    pub target_code: u32,
}

fn set_game_parameters() -> GameParameters {
    // This entire function allows the user to set all of the parameters of the Puzzle that will be generated to play.
    // At the moment, only Classic Mode, Original-Parameters are supported.
    
    let mut og_tm_game: bool; // If the user sets this bool to 'true' using the "y" match arm, then we can skip inputs for min_digit, max_digit, code_length, min_code, and max_code. Otherwise, we gather that information in the "n" match arm.
    let mut min_digit: char;
    let mut max_digit: char;
    let mut code_length: u8;
    let mut min_code: u32 = 0;
    let mut max_code: u32 = 0;
    loop {
        let mut input: String = String::new();
        println!("Are you trying to play a game of the Original \"Turing Machine\" board game? (y/n)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "Y" | "y" => {
                og_tm_game = true;
                min_digit = '1';
                max_digit = '5';
                min_code = 111;
                max_code = 555;
                break;
            },
            
            "N" | "n" => {
                og_tm_game = false;

                loop {
                    let mut input: String = String::new();

                    println!(
                        "↓ Please input the smallest digit character (In the original game, this is '1')."
                    );
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    min_digit = match input.trim() {
                        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => input.trim().chars().next().expect("empty input"),
                        _ => {
                            println!("Invalid smallest digit character \"{}\"", input.trim());
                            continue;
                        }
                    };

                    let mut input: String = String::new();
                    
                    println!("↓ Please input the largest digit character (In the original game, this is '5').");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    max_digit = match input.trim() {
                        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => input.trim().chars().next().expect("empty input"),
                        _ => {
                            println!("Invalid largest digit character \"{}\"", input.trim());
                            continue;
                        }
                    };
                    if max_digit <= min_digit {
                        println!(
                            "Largest digit character must be greater than smallest digit character: {} <= {}",
                            max_digit, 
                            min_digit
                        );
                        continue;
                    }
                    break;
                }

                loop {
                    let mut input: String = String::new();
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

                for _ in 1..=code_length {
                    min_code *= 10;
                    min_code += min_digit.to_digit(10).unwrap();
                    max_code *= 10;
                    max_code += max_digit.to_digit(10).unwrap();
                }
            },
            
            _ => {
                println!("Invalid input \"{}\"", input.trim());
                continue;
            },
        }
    }

    let mode: Gamemode;
    loop {
        let mut input: String = String::new();
        println!("↓ Please input the gamemode setting; Your choices are \"Classic Mode\"(c), \"Extreme Mode\"(e), and \"Nightmare Mode\"(n).");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        mode = match input.trim() {
            "c" | "C" => Gamemode::Classic,
            "e" | "E" => Gamemode::Extreme,
            "n" | "N" => Gamemode::Nightmare,
            _ => {
                println!("Invalid mode selection \"{}\"", input.trim());
                continue;
            }
        };
        break;
    }

    let shuffle: bool;
    loop {
        shuffle = match mode {
            Gamemode::Classic | Gamemode::Extreme => {
                let mut input: String = String::new();
                println!("↓ Would you like the Test Verifiers to be Shuffled, akin to the Nightmare Mode?");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                match input.trim() {
                    "y" | "Y" => true,
                    "n" | "N" => false,
                    _ => {
                        println!("Invalid shuffle selection \"{}\"", input.trim());
                        continue;
                    },
                }
            },
            Gamemode::Nightmare => true,
        };
        break;
    }

    let cards_per_section: u8;
    loop {
        let mut input: String = String::new();
        println!("↓ Please input the number of Criteria Cards present per section of the machine (In Extreme Mode, it's 2, otherwise it's usually just 1.");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let parsed_num: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Cards-Per-Test amount \"{}\"", input.trim());
                continue;
            }
        };
        if !(1.0 <= parsed_num && parsed_num < 256.0) {
            println!("Ensure that 1 <= Cards-Per-Test < 256  \"{}\"", parsed_num);
            continue;
        } else {
            cards_per_section = parsed_num.floor() as u8;
        }
        break;
    }

    let difficulty: Difficulty;
    loop {
        let mut input: String = String::new();
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

    let sections: u8;
    loop {
        let mut input: String = String::new();
        println!("↓ Please input the number of Sections on the machine that are assigned Criteria Verifiers (In the original game, this is from 4 to 6, inclusive)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let parsed_num: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ensure that 1 <= Section Amount < 256 \"{}\"", input.trim());
                continue;
            }
        };
        if parsed_num < 1.0 || parsed_num % 1.0 != 0.0 || parsed_num > 255.0 {
            println!("Section Amount must be a positive integer: \"{}\"", parsed_num);
            continue;
        } else {
            sections = parsed_num as u8;
        }
        break;
    }

    let params: GameParameters = GameParameters {
        og_tm_game: og_tm_game,
        min_code: min_code,
        max_code: max_code,
        min_digit: min_digit,
        max_digit: max_digit,
        mode: mode,
        difficulty: difficulty,
        sections: sections,
        cards_per_section: cards_per_section,
        shuffle: shuffle
    };

    return params;
}

pub fn is_valid_turing_code(
    min_code: u32,
    max_code: u32,
    min_digit: char,
    max_digit: char,
    test_code: u32,
) -> bool {
    //Simply a reusable statement returning True if the proposed Turing Code is valid as per the parameters passed into the program by the user.

    return 
        (min_code..=max_code).contains(&test_code)
        && test_code
            .to_string()
            .chars()
            .all(|c| c >= min_digit && c <= max_digit);
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
    og_tm_game: bool,
) -> Vec<TuringCodeEval> {
    // Puts every Turing Code from the generate_number_pool() function through every Criteria Card's multiple Tests, and returns the resulting Vector of Structs.

    let codes: Vec<u32> = generate_number_pool(min_code, max_code, min_digit, max_digit);
    let mut results_matrix: Vec<TuringCodeEval> = vec![];

    if og_tm_game {
        for code in codes.iter() {
            results_matrix.push(
                og_tm_game_variant::criteria_card_tests::evaluate_criteria_results(code.clone()),
            )
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

fn generate_random_puzzle_code(matrix: &Vec<TuringCodeEval>) -> (u32, usize) {
    // returns a random Turing Code from the matrix argument that is the solution to the Puzzle that will be generated.

    let mut rng: ThreadRng = rand::thread_rng();
    let target_index: usize = rng.gen_range(0..matrix.len());
    let target_code:u32 = matrix[target_index].code;

    return (target_code, target_index);
}

pub fn generate_coupled_criteria(matrix: &Vec<TuringCodeEval>) -> Vec<Vec<usize>> {
    // returns a 2D array of Coupled Tests. A test is coupled to another test if for every possible Turing Code, the result of Test X matches the result of Test Y. By definition, this renders one of the tests superfluous; Test X should not be paired with Test Y in a valid Puzzle, and vice versa.

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

pub fn generate_centralizing_test_list(matrix: &Vec<TuringCodeEval>, test_amount: &u8) -> Vec<usize> {
    // returns a list of every test from the various Criteria Cards for which the number of solutions is not high enough to ensure that each does not render any of the other Tests in the Puzzle superfluous.

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
        .filter_map(|(index, count)|
            if count < *test_amount as u32 { 
                Some(index) 
            } else { 
                None }
            )
        .collect();
}

fn set_test_pool_range(
    og_tm_game: bool,
    last_index: usize,
    mode: &Gamemode,
    difficulty: &Difficulty,
    second_half_of_puzzle: bool,
) -> RangeInclusive<usize> {
    let test_pool: RangeInclusive<usize>;

    if og_tm_game {
        if !second_half_of_puzzle {
            test_pool = match (mode, difficulty) {
                (Gamemode::Classic, Difficulty::Easy) => 0..=49,
                (Gamemode::Classic, Difficulty::Standard) => 17..=62,
                (Gamemode::Classic, Difficulty::Hard) => 69..=last_index,
                (Gamemode::Extreme, Difficulty::Easy) => 29..=71,
                (Gamemode::Extreme, Difficulty::Standard) => 29..=71,
                (Gamemode::Extreme, Difficulty::Hard) => 63..=last_index,
                (Gamemode::Nightmare, Difficulty::Easy) => 29..=49,
                (Gamemode::Nightmare, Difficulty::Standard) => 18..=62,
                (Gamemode::Nightmare, Difficulty::Hard) => 66..=last_index,
            }
        } else {
            test_pool = match (mode, difficulty) {
                (Gamemode::Classic, Difficulty::Easy) => 0..=49,
                (Gamemode::Classic, Difficulty::Standard) => 0..=62,
                (Gamemode::Classic, Difficulty::Hard) => 0..=last_index,
                (Gamemode::Extreme, Difficulty::Easy) => 0..=71,
                (Gamemode::Extreme, Difficulty::Standard) => 0..=71,
                (Gamemode::Extreme, Difficulty::Hard) => 0..=last_index,
                (Gamemode::Nightmare, Difficulty::Easy) => 0..=49,
                (Gamemode::Nightmare, Difficulty::Standard) => 0..=62,
                (Gamemode::Nightmare, Difficulty::Hard) => 0..=last_index,
            }
        }
    } else {
        test_pool = 0..=last_index;
    }

    return test_pool;
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

fn puzzle_building_validation(
    puzzle_tests: &Vec<usize>,
    matrix: &Vec<TuringCodeEval>,
    unique_solutions_needed: usize,
    target_index: &usize,
    banned_tests: Vec<usize>,
    used_cards: Vec<u8>,
    vec_test_couplings: &Vec<Vec<usize>>,
    test_pool: RangeInclusive<usize>
) -> bool {
    // returns true if puzzle_tests argument is validly diverse/unique enough for the current state of the puzzle.

    let mut num_of_solutions: usize = 0;

    for (_, turing_code_result) in matrix.iter().enumerate() {
        let all_true: bool = puzzle_tests
            .iter()
            .all(|&i| turing_code_result.checks.get(i).map_or(false, |&(_, b)| b));

        if all_true {
            num_of_solutions += 1;
        }
    }

    if num_of_solutions == unique_solutions_needed && unique_solutions_needed == 1 {
        return true;
    } else if num_of_solutions < unique_solutions_needed {
        return false;
    } else {
        // testing if there is an existing path forwards if this new_test_index is added (Future check)
        let last_index: usize = puzzle_tests.len() - 1;
        let new_test_index: usize = puzzle_tests[last_index];
        let mut tmp_banned_tests: Vec<usize> = banned_tests;
        let mut tmp_used_cards: Vec<u8> = used_cards;
        
        for index in vec_test_couplings[new_test_index].iter() {
            tmp_banned_tests.push(*index);
        }
        tmp_used_cards.push(matrix[*target_index].checks[new_test_index].0);
        
        for index in test_pool {
            if matrix[*target_index].checks[index].1 == true
                && !tmp_used_cards.contains(&matrix[0].checks[index].0)
                && !tmp_banned_tests.contains(&index)
            {
                return true;
            }
        }

        return false;
    }
}

pub fn generate_puzzle() -> Puzzle {
    let params = set_game_parameters();
    println!(
        "Minimum Code: {}; Maximum Code: {};\nSmallest Digit: {}; Largest Digit: {};\nGamemode: {:?}; Difficulty: {:?};\nNumber of Sections: {}; Criteria Cards Per Section: {};\nShuffled Verifiers?: {};", 
        params.min_code, 
        params.max_code, 
        params.min_digit, 
        params.max_digit, 
        params.mode, 
        params.difficulty,
        params.sections,
        params.cards_per_section,
        params.shuffle
    );
    let matrix: Vec<TuringCodeEval> = generate_results_matrix(params.min_code, params.max_code, params.min_digit, params.max_digit, params.og_tm_game);
    println!("Matrix generated...");
    debug_helpers::print_true_instances(&matrix);

    let last_index: usize = matrix[0].checks.len() - 1;
    let last_card: u8 = matrix[0].checks[last_index].0;
    if params.sections > ((last_card / params.sections) + (last_card % params.sections)) {
        panic!("Number of Criteria Cards avaiable exeeds needed number of Criteria Cards assignable to Puzzle");
    }
    
    let vec_test_couplings: Vec<Vec<usize>> = generate_coupled_criteria(&matrix);
    let vec_centralized_tests: Vec<usize> = generate_centralizing_test_list(&matrix, &params.sections);
    
    for centralized_banned_test in vec_centralized_tests.iter() {
        println!(
            "Banned Test for centralization: Card {}/{}, Test {}/{};",
            matrix[0].checks[*centralized_banned_test].0,
            matrix[0].checks[last_index].0,
            &centralized_banned_test,
            &last_index
        );
    }
    let mut banned_tests: Vec<usize> = vec_centralized_tests.clone();
    let mut used_cards: Vec<u8> = vec![];
    let mut tests_added: usize = 0;
    let half_tests: u8 = match params.sections % 2 {
        0 => params.sections / 2,
        _ => (params.sections / 2) + 1,
    };
    let mut second_half_of_puzzle: bool = false;
    let mut future_second_half_of_puzzle: bool = false;

    println!("Generating the puzzle...");

    let (mut target_code, mut target_index) = generate_random_puzzle_code(&matrix);
    let mut puzzle: Puzzle = Puzzle {
        matrix: matrix,
        tests: vec![],
        target_code: target_code,
    };

    loop {
        let unique_solutions_needed: usize = params.sections as usize - tests_added;

        if tests_added >= half_tests as usize {
            second_half_of_puzzle = true;
        } else {
            second_half_of_puzzle = false;
        }
        let test_pool: RangeInclusive<usize> = set_test_pool_range(
            params.og_tm_game,
            last_index,
            &params.mode,
            &params.difficulty,
            second_half_of_puzzle,
        ); 
        if tests_added + 1 >= half_tests as usize {
            future_second_half_of_puzzle = true;
        } else {
            future_second_half_of_puzzle = false;
        }
        let future_test_pool: RangeInclusive<usize> = set_test_pool_range(
            params.og_tm_game,
            last_index,
            &params.mode,
            &params.difficulty,
            future_second_half_of_puzzle,
        );

        if tests_added + 1 < params.sections as usize {
            // This future_check 'if' block tests whether or not any of the presently-available paths forward is indeed viable. 
            // Rather than narrowing down the available options one by one until there are none left, this checks makes sure that at least one such check exists. 
            // If it does not, then the entire puzzle is reset. This is far easier than walking back the last test that was added, because that would require all of the associated coupled tests and Criteria Card be removed from their respective vectors, and that would be too much to keep track of.
            let mut future_check: bool = false;
            
            for new_test_index in test_pool.clone() {
                if puzzle.matrix[target_index].checks[new_test_index].1 
                && !banned_tests.contains(&new_test_index)
                && !used_cards.contains(&puzzle.matrix[0].checks[new_test_index].0) {
                    let mut puzzle_tests: Vec<usize> = puzzle.tests.clone();
                    puzzle_tests.push(new_test_index);

                    if puzzle_building_validation(
                        &puzzle_tests, 
                        &puzzle.matrix, 
                        unique_solutions_needed, 
                        &target_index, 
                        banned_tests.clone(), 
                        used_cards.clone(), 
                        &vec_test_couplings, 
                        future_test_pool.clone()
                    ) {
                        future_check = true;
                        break;
                    }
                }
            }

            if !future_check {
                println!(
                "Infallible Wall Hit. Resetting puzzle generation...",
                );
                
                tests_added = 0;
                banned_tests = vec_centralized_tests.clone();
                used_cards.clear();

                (target_code, target_index) = generate_random_puzzle_code(&puzzle.matrix);
                puzzle.target_code = target_code;
                puzzle.tests.clear();
                
                continue;
            }
        }
        
        let new_test_index: usize = generate_test_index_from_range(
            &puzzle.matrix[target_index].checks,
            test_pool,
            &used_cards,
            &banned_tests,
        );
        puzzle.tests.push(new_test_index);
        tests_added += 1;

        if tests_added == params.sections as usize {
            // The Puzzle is populated, pending a validation check:
            if !puzzle_building_validation( 
                &puzzle.tests, 
                &puzzle.matrix, 
                unique_solutions_needed, 
                &target_index, 
                banned_tests.clone(), 
                used_cards.clone(), 
                &vec_test_couplings, 
                future_test_pool
            ) {
                puzzle.tests.pop();
                tests_added -= 1;
            } else {
                println!("Successfully Created Valid Puzzle!");
                break;
            }
        } else {
            // The Puzzle still needs more tests added after new_test_index, if new_test_index doesn't invalidate the incomplete Puzzle:
            if !puzzle_building_validation( 
                &puzzle.tests, 
                &puzzle.matrix, 
                unique_solutions_needed, 
                &target_index, 
                banned_tests.clone(), 
                used_cards.clone(), 
                &vec_test_couplings, 
                future_test_pool
            ) {
                // new_test_index invalidates the puzzle by eliminating too many possible solutions; Redundancy would be required to complete Puzzle with new_test_index added.
                puzzle.tests.pop();
                tests_added -= 1;
            } else {
                // new_test_index doesn't invalidate the incomplete Puzzle; Proceed.
                for index in vec_test_couplings[new_test_index].iter() {
                    banned_tests.push(*index);
                }
                used_cards.push(puzzle.matrix[target_index].checks[new_test_index].0);
            }
        }
    }


    println!("Solution: {}", puzzle.target_code);
    for test in puzzle.tests.iter() {
        println!(
            "Test: {}, Card: {}",
            test,
            &puzzle.matrix[0].checks[*test].0
        );
    }

    return puzzle;
}
