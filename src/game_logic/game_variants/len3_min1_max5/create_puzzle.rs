use crate::{game_logic::setup::Puzzle, Gamemode, TuringCodeResults};

pub fn puzzle_maker(
    matrix: &Vec<TuringCodeResults>,
    test_amount: u8,
    mode: Gamemode,
    target_code: u32,
    target_index: usize,
    vec_test_couplings: &Vec<Vec<usize>>,
    vec_unique_tests: &Vec<usize>,
) -> Puzzle {
    let puzzle_code = 0;
    let puzzle_tests = vec![Vec::new(); test_amount as usize];
    let puzzle: Puzzle = Puzzle {
        target_code: puzzle_code,
        tests: puzzle_tests,
    };
    return puzzle;
}