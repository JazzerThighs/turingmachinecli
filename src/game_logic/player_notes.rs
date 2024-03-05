// use std::fmt;

// pub enum Checkbox {
//     Blank,
//     Passed,
//     Failed,
// }

// impl fmt::Display for Checkbox {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let symbol = match self {
//             Checkbox::Blank => "☐", 
//             Checkbox::Passed => "P", 
//             Checkbox::Failed => "F", 
//         };
//         write!(f, "{}", symbol)
//     }
// }

// pub enum DigitValueState {
//     Blank,
//     RuledOut,
//     Deduced,
// }

// impl fmt::Display for DigitValueState {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let symbol = match self {
//             DigitValueState::Blank => " ",
//             DigitValueState::RuledOut => "X",
//             DigitValueState::Deduced => "✓",
//         };
//         write!(f, "{}", symbol)
//     }
// }

// pub struct TestNotes {
//     notes: Vec<String>,
//     conclusive_result: Option<String>,
// }

// pub struct Player {
//     name: String,
//     game_id: String,
//     guesses: Vec<u32>, // A vector to hold the three-digit guesses as u32
//     test_results: Vec<Vec<Checkbox>>, // A vector of vectors to hold test results per round
//     code_digit_values: [[(DigitValueState, u8); 5]; 3], // A fixed-size array of tuples for each digit
//     notes: Vec<TestNotes>,
// }

// impl fmt::Display for Player {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Player: {}\nGame ID: {}\n", self.name, self.game_id)?;
//         write!(f, "Guesses:\n")?;
//         for &guess in &self.guesses {
//             writeln!(f, "{:03}", guess)?; // Assumes guesses are always 3 digits
//         }
//         write!(f, "Test Results:\n")?;
//         for round in &self.test_results {
//             for test in round {
//                 write!(f, "{} ", test)?;
//             }
//             writeln!(f)?;
//         }
//         write!(f, "Code Digit Values:\n")?;
//         for digit_values in &self.code_digit_values {
//             for (state, value) in digit_values {
//                 write!(f, "{}{}", state, value)?;
//             }
//             writeln!(f)?;
//         }
//         write!(f, "Notes:\n")?;
//         for test_note in &self.notes {
//             for note in &test_note.notes {
//                 write!(f, "{}\n", note)?;
//             }
//             if let Some(conclusive) = &test_note.conclusive_result {
//                 write!(f, "Conclusive Result: {}\n", conclusive)?;
//             }
//         }
//         Ok(())
//     }
// }
