use crate::game_logic::game_variants::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TuringCodeEval {
    // Structure to pair every individual Turing Code with an array of booleans as it is put through every Test on every Criteria Card. This is replacing the pre-calculated Punch Cards used for querying the Turing Machine's Verifier Cards.
    pub checks: Vec<(u8, bool)>, // Vec<(criteria_card_number, pass_or_fail)>
    pub code: u32,
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