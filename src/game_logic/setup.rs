use std::io;
pub fn set_game_parameters() -> (u32, u32, char, char, String, u8) {
    let mut min_digit: char;
    let mut max_digit: char;
    loop {
        let mut input = String::new();

        println!("Please input the smallest digit character (In the original game, this is '1').");
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
        println!("Please input the largest digit character (In the original game, this is '5').");
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
        println!("Please input the number of digits in the valid codes (In the original game, this is 3, resulting in codes ranging from 111 to 555, inclusive).");
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
    
    let mode: String;
    loop {
        let mut input = String::new();
        println!("Please input the gamemode setting; Your choices are \"Classic Mode\"(c), \"Extreme Mode\"(e), and \"Nightmare Mode\"(n).");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        mode = match input.trim() {
            "c" | "e" | "n" => input.trim().to_owned(),
            _ => {
                println!("Invalid mode selection \"{}\"", input.trim());
                continue;
            }
        };
        break;
    }
    
    let mut test_amount: u8;
    loop {
        let mut input = String::new();
        println!("Please input the number of tests that the puzzle shall contain (In the original game, there may be 4, 5, or 6 tests).");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        test_amount = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid test number entered \"{}\"", input.trim());
                continue;
            }
        };
        match test_amount {
            4 | 5 | 6 => break,
            _ => {
                println!("Chosen number of tests not implemented \"{}\"", test_amount);
                continue;
            }
        }
    }

    return (min_code, max_code, min_digit, max_digit, mode, test_amount);
}

pub fn is_valid_turing_code(
    min_code: u32,
    max_code: u32,
    min_digit: char,
    max_digit: char,
    test_code: u32,
) -> bool {
    (min_code..=max_code).contains(&test_code)
        && test_code
            .to_string()
            .chars()
            .all(|c| c >= min_digit && c <= max_digit)
}

pub fn generate_random_puzzle_code(code_length: u32, min_digit: char, max_digit: char) -> u32 {
    use rand::{rngs::ThreadRng, Rng};
    let mut target_code: u32 = 0;
    let mut rng: ThreadRng = rand::thread_rng();
    for _ in 1..=code_length {
        target_code *= 10;
        target_code += rng.gen_range(min_digit..=max_digit) as u32;
    }
    return target_code;
}

pub struct TuringCodeResults {
    pub code: u32,
    pub checks: Vec<(u8, bool)>,
}

pub fn generate_number_pool(
    min_code: u32,
    max_code: u32,
    min_digit: char,
    max_digit: char,
) -> Vec<u32> {
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
    let codes: Vec<u32> = generate_number_pool(min_code, max_code, min_digit, max_digit);
    let mut results_matrix: Vec<TuringCodeResults> = vec![];

    for code in codes.iter() {
        use crate::game_logic::game_variants::*;
        match (min_code, max_code, min_digit, max_digit) {
            (111, 555, '1', '5') => results_matrix.push(
                len3_min1_max5::criteria_cards::evaluate_criteria_results(code.clone()),
            ),
            _ => {}
        }
    }

    return results_matrix;
}

pub fn target_code_index(matrix: Vec<TuringCodeResults>, target_code: u32) -> u32 {
    for index in 0..matrix.len() {
        if matrix[index].code == target_code {
            return index as u32;
        }
    }
    return 0;
}

//  TODO: Generate the Puzzle by taking the generated random target_code, then selecting at least 4 tests which follow the schematic of the game, and setting up the user to play.
//      PUZZLE GENERATION:
//          -Generate a Vec<Vec<u32>> that has the same length as the results matrix. Each index contains a vector of test indexes that are coupled to that index's test.
//          -Select a random test index which is true in the target_index's checks vector, and add it to the list of tests for the puzzle.
//              -Ensure that the test index has been chosen based on the range specified by the selected difficulty:
//                  Easy | Standard => Criteria Cards 1..=25,
//                  Hard            => Criteria Cards 26..=48 for at least the first half of the tests, and 1..=48 for the rest of the tests.
//              -Ensure that the test which has been selected hasn't had it's criteria card number added to the list of taken criteria cards.
//              -Ensure that the test which has been selected is not included in any one of the previous tests' vector of coupled tests.
//              If (the number of tests < the number of tests specified for the puzzle) {
//                  for however many tests have been added to the puzzle's test list, make sure that the collection of tests chosen are not uniquely true for that one code.
//              } Else If (the number of tests == the number of tests specified for the puzzle) {
//                  ensure that the collection of tests is uniquely truthy for that one target_code, otherwise throw out the last code.
//              }
