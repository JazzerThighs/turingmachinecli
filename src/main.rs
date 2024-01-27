mod gamelogic;
use crate::gamelogic::*;

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ \n\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet");
    
    let code_array: Vec<u32> = setup::generate_number_pool(3, 1, 5);
    let mut results_matrix: Vec<setup::TuringCodeResults> = vec![];
    for code in code_array.iter() {
        results_matrix.push(setup::generate_result_matrix(code.clone()));
    }

    let matrix = results_matrix;

    let round_counter: u8 = 1;
}
