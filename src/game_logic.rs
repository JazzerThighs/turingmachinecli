pub mod setup;
pub mod round_loop;
pub mod player_notes;
pub mod game_variants;

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut, RangeInclusive},
    io,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use crate::game_logic::game_variants::*;

pub type Matrix = Vec<TuringCodeEval>;
pub type Machine = HashMap<Card, HashMap<Code, Vec<bool>>>;
pub type CardStrings = HashMap<Card, Vec<String>>;

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

#[derive(Debug)]
pub struct MachineParams {
    pub min_code: Code,
    pub max_code: Code,
    pub min_digit: char,
    pub max_digit: char,
    pub code_len: usize,
    pub gamemode: Gamemode,
    pub difficulty: Difficulty,
    pub test_amount: usize,
    pub og_tm_game: bool,
}

#[derive(Debug, Clone, Default)]
pub struct BigIndex(usize);
impl Deref for BigIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BigIndex {
    fn deref_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct SmallIndex(usize);
impl Deref for SmallIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SmallIndex {
    fn deref_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
pub struct Code(usize);
impl Deref for Code {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Code {
    fn deref_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
pub struct Card(usize);
impl Deref for Card {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Card {
    fn deref_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct SingleEval {
    pub card: Card,
    pub small_index: SmallIndex,
    pub passed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TuringCodeEval {
    pub code: Code,
    pub checks: Vec<SingleEval>,
}

#[derive(Clone, Default)]
pub struct Section {
    pub card: Card,
    pub small_index: SmallIndex,
    pub big_index: BigIndex,
}

#[derive(Clone, Default)]
pub struct Puzzle {
    pub target_code: Code,
    pub tests: Vec<Section>,
}

#[allow(unused_assignments)]
pub fn set_game_parameters() -> MachineParams {
    // This entire function allows the user to set all of the parameters of the Puzzle that will be generated to play.
    // At the moment, only Classic Mode, Original-Parameters are supported.

    let mut min_digit: char;
    let mut max_digit: char;
    let mut code_length: u8;
    let mut min_code: u32 = 0;
    let mut max_code: u32 = 0;

    let mut og_tm_game: bool;
    // If the user sets this bool to 'true' using the "y" match arm, then we can skip inputs for min_digit, max_digit, code_length, min_code, and max_code.
    // Otherwise, we gather that information in the "n" match arm.
    loop {
        let mut input = String::new();
        println!("Are you trying to play a game of the Original \"Turing Machine\" board game? \"y\" or \"n\"");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "y" => {
                og_tm_game = true;
                min_digit = '1';
                max_digit = '5';
                code_length = 3;
                min_code = 111;
                max_code = 555;
                break;
            }
            
            "n" => {
                og_tm_game = false;

                loop {
                    let mut input = String::new();

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

                    let mut input = String::new();
                    
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

                for _ in 1..=code_length {
                    min_code *= 10;
                    min_code += min_digit.to_digit(10).unwrap();
                    max_code *= 10;
                    max_code += max_digit.to_digit(10).unwrap();
                }
            }
            _ => {
                println!("Invalid input \"{}\"", input.trim());
                continue;
            }
        }
    }

    let mode: Gamemode;
    loop {
        let mut input = String::new();
        println!("↓ Please input the gamemode setting; Your choices are \"Classic Mode\"(c), \"Extreme Mode\"(e), and \"Nightmare Mode\"(n).");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        mode = match input.trim() {
            "c" => Gamemode::Classic,
            "e" => Gamemode::Extreme,
            "n" => Gamemode::Nightmare,
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

    let test_amount: usize;
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

    MachineParams {
        min_code: Code(min_code as usize),
        max_code: Code(max_code as usize),
        min_digit,
        max_digit,
        code_len: code_length as usize,
        gamemode: mode,
        difficulty,
        test_amount,
        og_tm_game,
    }
}

pub fn is_valid_turing_code(mp: &MachineParams, test_code: Code) -> bool {
    (*mp.min_code..=*mp.max_code).contains(&test_code)
        && test_code
            .to_string()
            .chars()
            .all(|c| c >= mp.min_digit && c <= mp.max_digit)
}

pub fn generate_random_puzzle_code(mp: &MachineParams) -> Code {
    let mut target_code: Code = Code(0);
    let mut rand_num: ThreadRng = thread_rng();
    for _ in 1..=mp.code_len {
        *target_code *= 10;
        *target_code +=
            rand_num.gen_range(mp.min_digit.to_digit(10).unwrap()..=mp.max_digit.to_digit(10).unwrap()) as usize;
    }
    target_code
}

#[rustfmt::skip]
pub fn set_test_pool_range(mp: &MachineParams, last_index: usize, second_half_of_puzzle: bool,) -> RangeInclusive<usize> {
    use Gamemode::*;
    use Difficulty::*;
    match (
        mp.og_tm_game,
        &mp.gamemode,
        &mp.difficulty,
        second_half_of_puzzle
    ) {
        (false, _, _, _) => 0..=last_index,
        (_, Classic,   Easy,     true ) => 0..=49,
        (_, Classic,   Easy,     false) => 0..=49,
        (_, Classic,   Standard, true ) => 17..=62,
        (_, Classic,   Standard, false) => 0..=62,
        (_, Classic,   Hard,     true ) => 69..=last_index,
        (_, Classic,   Hard,     false) => 0..=last_index,
        (_, Extreme,   Easy,     true ) => 29..=71,
        (_, Extreme,   Easy,     false) => 0..=71,
        (_, Extreme,   Standard, true ) => 29..=71,
        (_, Extreme,   Standard, false) => 0..=71,
        (_, Extreme,   Hard,     true ) => 63..=last_index,
        (_, Extreme,   Hard,     false) => 0..=last_index,
        (_, Nightmare, Easy,     true ) => 29..=49,
        (_, Nightmare, Easy,     false) => 0..=49,
        (_, Nightmare, Standard, true ) => 18..=62,
        (_, Nightmare, Standard, false) => 0..=62,
        (_, Nightmare, Hard,     true ) => 66..=last_index,
        (_, Nightmare, Hard,     false) => 0..=last_index,
    }
}

pub fn generate_results_matrix(mp: &MachineParams) -> (Matrix, Machine, CardStrings) {
    let codes: Vec<Code> = (*mp.min_code..=*mp.max_code)
        .into_iter()
        .filter(|test_code| is_valid_turing_code(&mp, Code(*test_code)))
        .map(|c| Code(c))
        .collect();
    let mut matrix: Matrix = vec![];
    let mut machine: Machine = HashMap::default();
    let mut cards: CardStrings = HashMap::default();
    if mp.og_tm_game {
        for code in codes.iter(){
            matrix.push(
                og_tm_board_game::criteria_card_tests::populate_machine_feedback_2(*code, &mut machine),
            )
        }
        cards = og_tm_board_game::criteria_card_strings::criteria_card_strings_2s();
    }
    // else {
    //     for code in codes.iter() {
    //         match (min_code, max_code, min_digit, max_digit) {
    //             (111, 555, '1', '5') => results_matrix.push(len3_min1_max5::criteria_card_tests::populate_machine_feedback(*code, &mut machine)),
    //             _ => {}
    //         }
    //     }
    //     match (min_code, max_code, min_digit, max_digit) {
    //         (111, 555, '1', '5') => cards.extend(len3_min1_max5::criteria_card_strings::criteria_card_strings()),
    //         _ => {}
    //     }
    // }
    let mut cardmap: HashMap<String, Vec<String>> = HashMap::default();
    for i in 0..cards.len() {
        let c = &cards[i];
        let splits: Vec<&str> = c.split("\n").collect();
        let n: Vec<String> = splits[0..].iter().map(|s| s.to_string()).collect();
        cardmap.insert((i + 1).to_string(), n);
    }
    (matrix, machine, cardmap)

    
    // let codes: Vec<u32> = (min_code..=max_code).into_iter().filter(|code| is_valid_turing_code(min_code, max_code, min_digit, max_digit, *code)).collect();
    // let mut results_matrix: Vec<TuringCodeEval> = vec![];
    // let mut machine: Vec<HashMap<String, Vec<bool>>> = vec![];
    // let mut cards: Vec<String> = vec![];
    // if og_tm_game {
    //     for code in codes.iter() {
    //         results_matrix.push(
    //             og_tm_board_game::criteria_card_tests::populate_machine_feedback(*code, &mut machine),
    //         )
    //     }
    //     cards.extend(og_tm_board_game::criteria_card_strings::criteria_card_strings());
    // }
    // // else {
    // //     for code in codes.iter() {
    // //         match (min_code, max_code, min_digit, max_digit) {
    // //             (111, 555, '1', '5') => results_matrix.push(len3_min1_max5::criteria_card_tests::populate_machine_feedback(*code, &mut machine)),
    // //             _ => {}
    // //         }
    // //     }
    // //     match (min_code, max_code, min_digit, max_digit) {
    // //         (111, 555, '1', '5') => cards.extend(len3_min1_max5::criteria_card_strings::criteria_card_strings()),
    // //         _ => {}
    // //     }
    // // }
    // let mut cardmap: HashMap<String, Vec<String>> = HashMap::default();
    // for i in 0..cards.len() {
    //     let c = &cards[i];
    //     let splits: Vec<&str> = c.split("\n").collect();
    //     let n: Vec<String> = splits[0..].iter().map(|s| s.to_string()).collect();
    //     cardmap.insert((i + 1).to_string(), n);
    // }
    // (results_matrix, machine, cardmap)
}