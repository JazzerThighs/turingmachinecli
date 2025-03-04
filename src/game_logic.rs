pub mod setup;
pub mod round_loop;
pub mod player_notes;
pub mod game_variants;

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

pub type Machine = HashMap<Card, HashMap<Code, Vec<bool>>>;
pub type CardStrings = HashMap<Card, Vec<String>>;
pub type Matrix = Vec<TuringCodeEval>;

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
    pub min_code: usize,
    pub max_code: usize,
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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
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
