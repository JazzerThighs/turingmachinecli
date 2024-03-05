// use std::collections::HashMap;

// pub fn print_true_instances(matrix: &[crate::setup::TuringCodeEval]) {
//     // Prints the number of codes from within a given matrix that each particular indexed test would return a 'true' value for.
//     // i.e. if the test is {There are three 1's in the code}, and the length of all valid Turing Codes is 3, only the Turing Code "111" would ever pass the test; This would result in that test's print statement to show 1 'true' instance.

//     let mut counts: HashMap<usize, u32> = HashMap::new();
//     let mut last_index = 0;
//     // Calculate the counts of true values and find the last index
//     for turing_code_result in matrix {
//         for (index, (_, value)) in turing_code_result.checks.iter().enumerate() {
//             if *value {
//                 *counts.entry(index).or_insert(0) += 1;
//             }
//             last_index = last_index.max(index); // Ensure we capture the highest index
//         }
//     }
//     // Print the counts for each index, ensuring order from 0 to last_index
//     for index in 0..=last_index {
//         if let Some(count) = counts.get(&index) {
//             let card_number = if let Some((card_number, _)) = matrix[0].checks.get(index) {
//                 *card_number
//             } else {
//                 0 // Default to 0 if not found, though this case should ideally not occur
//             };
//             println!(
//                 "Card {:03} of {:03}, Test {:03} of {:03}: {:04} TRUE Instances.",
//                 card_number,
//                 matrix[0].checks.last().map_or(0, |(num, _)| *num), // Extract the last card number
//                 index,
//                 last_index,
//                 count
//             );
//         } else {
//             // In case there's an index with 0 TRUE instances, we still handle the formatting
//             println!(
//                 "Card {:03} of {:03}, Test {:03} of {:03}: 0 TRUE Instances.",
//                 matrix[0].checks.get(index).map_or(0, |(num, _)| *num),
//                 matrix[0].checks.last().map_or(0, |(num, _)| *num),
//                 index,
//                 last_index
//             );
//         }
//     }
// }
