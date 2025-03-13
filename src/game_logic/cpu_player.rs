use crate::game_logic::*;

pub fn any_tests_positive(card: &Card, code: &Code, machine: &Machine) -> bool {
    machine[card][code]
        .iter()
        .filter(|a| **a)
        .count() > 0
}

pub fn all_cards_matched(code: &Code, puzzle: &Puzzle) -> bool {
    puzzle
        .tests
        .iter()
        .map(|t| t.card.clone())
        .collect::<Vec<Card>>()
        .iter()
        .all(|card| any_tests_positive(card, &code, &puzzle.machine))
}