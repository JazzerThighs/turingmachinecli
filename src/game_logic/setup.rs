pub fn select_code_structure() -> (u32, u32, char, char) {
    let mut min_digit: char;
    let mut max_digit: char;
    let mut min_code: u32 = 0;
    let mut max_code: u32 = 0;

    loop {
        use std::io;
        let mut input = String::new();

        println!("Please input the smallest digit character. (In the original game, this is '1'.)");
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
        println!("Please input the largest digit character. (In the original game, this is '5'.)");
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
            println!("Largest digit character must be greater than smallest digit character: {} <= {}", max_digit, min_digit);
            continue;
        }

        let mut input = String::new();
        println!("Please input the number of digits in the valid codes. (In the original game, this is 3, resulting in codes ranging from 111 to 555, inclusive.)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let code_length: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid code length entered \"{}\"", input.trim());
                continue;
            }
        };
        match code_length {
            3 => {},
            _ => {
                println!("Chosen length not implemented \"{}\"", code_length);
                continue;
            }
        }

        for _ in 1..=code_length {
            min_code *= 10;
            min_code += min_digit.to_digit(10).unwrap();
            max_code *= 10;
            max_code += max_digit.to_digit(10).unwrap();
        }

        break;
    }

    return (min_code, max_code, min_digit, max_digit);
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

pub fn generate_random_puzzle_code(
    min_code: u32,
    max_code: u32,
    min_digit: char,
    max_digit: char,
) -> u32 {
    use rand::{rngs::ThreadRng, Rng};
    let mut target_code: u32;
    let mut rng: ThreadRng = rand::thread_rng();

    loop {
        target_code = rng.gen_range(min_code..=max_code);
        if is_valid_turing_code(min_code, max_code, min_digit, max_digit, target_code) {
            return target_code;
        }
    }
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
