pub mod round_loop;
pub mod display;
pub mod game_variants;
pub mod cpu_player;

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut, RangeInclusive},
    io,
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use crate::game_logic::game_variants::*;

pub type Matrix = Vec<TuringCodeEval>;
pub type Machine = HashMap<Card, HashMap<Code, Vec<bool>>>;
pub type CardStrings = HashMap<Card, Vec<String>>;
pub type Couplings = Vec<Vec<bool>>;
pub type Centralized = Vec<bool>;

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
    pub marker: String,
    pub card: Card,
    pub small_index: SmallIndex,
    pub big_index: BigIndex,
}

#[derive(Clone, Default)]
pub struct Puzzle {
    pub target_code: Code,
    pub tests: Vec<Section>,
}

pub struct Verifier<'a> {
    pub test: &'a BigIndex,
    section_marker: String,
    pub label: String
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
                        x if "0123456789".contains(x.trim()) => x.trim().chars().next().expect("empty input"),
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
                        x if "0123456789".contains(x.trim()) => x.trim().chars().next().expect("empty input"),
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

fn generate_random_puzzle_code(mp: &MachineParams) -> Code {
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
fn set_test_pool_range(mp: &MachineParams, last_index: usize, second_half_of_puzzle: bool,) -> RangeInclusive<usize> {
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
    let mut cards: Vec<String> = vec![];
    if mp.og_tm_game {
        for code in codes.iter(){
            matrix.push(
                og_tm_board_game::criteria_card_tests::populate_machine_feedback(code, &mut machine),
            )
        }
        cards = og_tm_board_game::criteria_card_strings::criteria_card_strings();
    }
    // else {
    //     for code in codes.iter() {
    //         match (mp.min_code, mp.max_code, mp.min_digit, mp.max_digit) {
    //             (111, 555, '1', '5') => results_matrix.push(len3_min1_max5::criteria_card_tests::populate_machine_feedback(*code, &mut machine)),
    //             _ => {}
    //         }
    //     }
    //     match (mp.min_code, mp.max_code, mp.min_digit, mp.max_digit) {
    //         (111, 555, '1', '5') => cards.extend(len3_min1_max5::criteria_card_strings::criteria_card_strings()),
    //         _ => {}
    //     }
    // }
    let mut cardmap: CardStrings = HashMap::default();
    for i in 0..cards.len() {
        let c = &cards[i];
        let splits: Vec<&str> = c.split("\n").collect();
        let n: Vec<String> = splits[0..].iter().map(|s| s.to_string()).collect();
        cardmap.insert(Card(i + 1), n);
    }
    (matrix, machine, cardmap)
}

fn is_unique_solution(
    target_index: &BigIndex,
    puzzle_tests: &Vec<BigIndex>,
    matrix: &Matrix,
) -> bool {
    // returns true if puzzle_tests argument is a unique set of true booleans among all of the codes.
    for (index, turing_code_result) in matrix.iter().enumerate() {
        let all_true = puzzle_tests
            .iter()
            .all(|i| turing_code_result.checks.get(**i).map_or(false, |b| b.passed));
        if all_true && index != **target_index {
            return false;
        }
    }
    true
}

fn generate_coupled_criteria(matrix: &Matrix) -> Couplings {
    // Returns a 2D array of Coupled Tests. A test is coupled to another test if for every possible Turing Code, the result of Test X matches the result of Test Y. 
    // By definition, this renders one of the tests superfluous; Test X should not be paired with Test Y in a valid Puzzle, and vice versa.
    // Also declares if two tests are coupled if they lie on the same Criteria Card, so they could never validly appear together in a puzzle anyway.
    let mut vec_test_couplings: Couplings = vec![vec![false; matrix[0].checks.len()]; matrix[0].checks.len()];
    let is_coupled = |x: usize, y: usize| -> bool {
        matrix
            .iter()
            .all(|turing_result| turing_result.checks[x].passed == turing_result.checks[y].passed)
    };
    for x in 0..matrix[0].checks.len() {
        for y in 0..matrix[0].checks.len() {
            if x != y && (*matrix[0].checks[x].card == *matrix[0].checks[y].card || is_coupled(x, y)) {
                vec_test_couplings[x][y] = true;
                vec_test_couplings[y][x] = true;
            }
        }
    }
    vec_test_couplings
}

fn generate_centralizing_test_list_whole_range(matrix: &Matrix, mp: &MachineParams) -> Centralized {
    // The purpose of this is to ensure that no Criteria Test renders any of the other Tests in the Puzzle superfluous by being true for a number of tests that is fewer than the number of tests in the puzzle.
    let mut vec_centralized_tests: Centralized = vec![false; matrix[0].checks.len()];
    for x in 0..matrix[0].checks.len() {
        let mut count = 0;
        for y in 0..matrix.len() {
            if matrix[y].checks[x].passed {
                count += 1;
            }
        }
        if count < mp.test_amount {
            vec_centralized_tests[x] = true;
        }
    }
    vec_centralized_tests
}

pub fn generate_puzzle(matrix: &Vec<TuringCodeEval>, mp: &MachineParams) -> Puzzle {
    let couplings: Vec<Vec<bool>> = generate_coupled_criteria(&matrix);
    let vct_whole_range: Vec<bool> = generate_centralizing_test_list_whole_range(&matrix, mp);
    let mut vct_whole_range_count = 0;
    for (i, _) in vct_whole_range.iter().enumerate().filter(|a| *a.1) {
        vct_whole_range_count += 1;
        println!(
            "Banned Test for uniqueness: Card {}/{}, Test {}/{};",
            *matrix[0].checks[i].card,
            *matrix[0].checks[matrix[0].checks.len() - 1].card,
            i,
            matrix[0].checks.len() - 1
        );
    }
    println!("Number of centralizing tests: {vct_whole_range_count}");
    // let half_tests = match mp.test_amount % 2 {
    //     0 => mp.test_amount / 2,
    //     _ => (mp.test_amount / 2) + 1,
    // };
    // let mut ranges: Vec<RangeInclusive<usize>> = (0..mp.test_amount)
    //     .map(|_| 0..=1)
    //     .collect();
    // for i in 0..mp.test_amount {
    //     ranges[i] = set_test_pool_range(
    //         mp,
    //         matrix[0].checks.len() - 1,
    //         i >= half_tests,
    //     );
    // }
    // let adjusted_ranges: Vec<Vec<usize>> = ranges
    //     .iter()
    //     .map(|r| {
    //         let mut rand_num = thread_rng();
    //         let start = rand_num.gen_range(*r.start()..=*r.end());
    //         let full_cycle: Vec<usize> = (*r).clone().collect();
    //         full_cycle.iter().cycle().skip(start).take(full_cycle.len()).copied().collect()
    //     })
    //     .collect();
    let adjusted_ranges: Vec<Vec<usize>> = (0..mp.test_amount)
        .map(|i| {
            let r = set_test_pool_range(mp, matrix[0].checks.len() - 1, i >= mp.test_amount.div_ceil(2));
            let mut rand_num = thread_rng();
            let start = rand_num.gen_range(*r.start()..=*r.end());
            let full_cycle: Vec<usize> = r.clone().collect();
            full_cycle.iter().cycle().skip(start).take(full_cycle.len()).copied().collect()
        })
        .collect();

    print!("Generating the puzzle...");
    let mut puzzle = Puzzle::default();

    while puzzle.tests.len() < mp.test_amount {
        puzzle = Puzzle {
            target_code: generate_random_puzzle_code(mp),
            tests: vec![]
        };
        let target_index = matrix.iter().position(|item| item.code == puzzle.target_code).unwrap();
        puzzle = puzzle_gen_algo(
            puzzle,
            &mp.test_amount,
            target_index,
            &matrix,
            &adjusted_ranges,
            &vct_whole_range,
            &couplings
        );
    }
    for (i, test) in puzzle.tests.iter_mut().enumerate() {
        test.marker = make_label(&i, 'A');
    }
    puzzle
}

fn puzzle_gen_algo(
    mut puzzle: Puzzle,
    test_amount: &usize,
    target_index: usize,
    matrix: &Vec<TuringCodeEval>,
    adjusted_ranges: &Vec<Vec<usize>>,
    vct_whole_range: &Vec<bool>,
    couplings: &Vec<Vec<bool>>
) -> Puzzle {
    let pool: &Vec<usize> = &adjusted_ranges[puzzle.tests.len()];
    'pool_loop: for i in pool
        .iter()
        .filter(|i| matrix[target_index].checks[**i].passed)
        .filter(|i| !vct_whole_range[**i])
    {
        if !puzzle.tests.iter().all(|existing_test| !couplings[*existing_test.big_index][*i])
            || puzzle.tests.iter().any(|a| *matrix[0].checks[*a.big_index].card == *matrix[0].checks[*i].card) 
        {
            continue 'pool_loop;
        }
        if puzzle.tests.len() == *test_amount - 1 {
            puzzle.tests.push(
                Section {
                    marker: "".to_string(),
                    card: matrix[0].checks[*i].card.clone(),
                    small_index: matrix[0].checks[*i].small_index.clone(),
                    big_index: BigIndex(*i)
                }
            );
            if is_unique_solution(&BigIndex(target_index), &puzzle.tests.iter().map(|t| t.big_index.clone()).collect(), &matrix) {
                println!("{} is a valid code for a test amount of {test_amount}.", *puzzle.target_code);
                return puzzle;
            } else {
                puzzle.tests.pop();
                continue 'pool_loop;
            }
        } else if puzzle.tests.len() < test_amount - 1 {
            puzzle.tests.push(
                Section {
                    marker: "".to_string(),
                    card: matrix[0].checks[*i].card.clone(),
                    small_index: matrix[0].checks[*i].small_index.clone(),
                    big_index: BigIndex(*i)
                }
            );
            let valid_solution_minimum: usize = (test_amount - puzzle.tests.len()) + 1;
            let mut solution_count = 0;
            'solution_counting: for (_, turing_code_result) in matrix.iter().enumerate() {
                if puzzle.tests
                    .iter()
                    .all(|i| turing_code_result.checks.get(*i.big_index).map_or(false, |b| b.passed)) 
                {
                    solution_count += 1;
                    if solution_count == valid_solution_minimum {
                        break 'solution_counting;
                    }
                }
            }
            if solution_count == valid_solution_minimum {
                puzzle = puzzle_gen_algo(
                    puzzle,
                    test_amount,
                    target_index,
                    &matrix,
                    &adjusted_ranges,
                    &vct_whole_range,
                    &couplings
                );
                break 'pool_loop;
            } else {
                puzzle.tests.pop();
                continue 'pool_loop;
            }
        }
    }
    puzzle.tests.sort_by(|a, b| a.big_index.cmp(&*b.big_index));
    puzzle
}

pub fn make_label(i: &usize, character: char) -> String {
    let mut index = i.clone();
    let mut label = String::new();
    loop {
        label.insert(0, (character as u8 + (index % 26) as u8) as char);
        index /= 26;
        if index == 0 {
            break;
        }
        index -= 1;
    }
    label
}

pub fn generate_verifiers<'a, 'b>(puzzle: &'a Puzzle, mp: &'b MachineParams) -> Vec<Verifier<'a>> {
    let mut verifiers: Vec<Verifier> = vec![];
    for (i, t) in puzzle.tests.iter().enumerate() {
        verifiers.push(
            Verifier {
                test: &t.big_index, 
                section_marker: make_label(&i, 'A'),
                label: make_label(&i, 'a')
            }
        );
    }
    match mp.gamemode {
        Gamemode::Nightmare => {
            let mut rng = thread_rng();
            let mut refs: Vec<(&BigIndex, String)> = verifiers.iter().map(|v| (v.test, v.section_marker.clone())).collect();
            refs.shuffle(&mut rng);
            for (i, b) in refs.iter().enumerate() {
                verifiers[i].test = b.0;
                verifiers[i].section_marker = b.1.clone()
            }
        }
        _ => {}
    }
    verifiers
}


