mod game_logic;
use crate::game_logic::*;
use rayon::prelude::*;
use serde_json::Value;
use std::{
    fs::File, 
    io::Write, 
    path::Path
};


fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");
    
    // Original Turing Machine Game Puzzle Database Population
    let matrix: Vec<setup::TuringCodeEval> = setup::generate_results_matrix(111, 555, '1', '5', true);
    let vec_test_couplings: Vec<Vec<usize>> = setup::generate_coupled_criteria(&matrix);
    
    // Classic Mode | Easy | Four Sections
    let test_amount: u8 = 4;
    let vec_centralized_tests: Vec<usize> = setup::generate_centralizing_test_list(&matrix, &test_amount);
    matrix.clone().into_par_iter().for_each(|tce| {
        
        
        for a in 0..=46 {
            if vec_centralized_tests.contains(&a) { continue; }

            for b in a+1..=47 {
                if vec_centralized_tests.contains(&b) || tce.checks[a].0 == tce.checks[b].0 || vec_test_couplings[a].contains(&b) { continue; }
                let uc_b: Vec<u8> = vec![tce.checks[a].0, tce.checks[b].0];
                let xt_b: Vec<usize> = vec![vec_test_couplings[a].clone(), vec_test_couplings[b].clone()].concat();
                
                for c in b+1..=48 {
                    if vec_centralized_tests.contains(&c) || uc_b.contains(&tce.checks[c].0) || xt_b.contains(&c) { continue; }
                    let uc_c: Vec<u8> = vec![uc_b.clone(), vec![tce.checks[c].0]].concat();
                    let xt_c: Vec<usize> = vec![xt_b.clone(), vec_test_couplings[c].clone()].concat();

                    for d in c+1..=49 {
                        if vec_centralized_tests.contains(&d) || uc_c.contains(&tce.checks[d].0) || xt_c.contains(&d) { continue; }
                        
                        let puzzle_tests: Vec<usize> = vec![a, b, c, d];
                        let mut valid_puzzle: bool = true;
                        for (index, tcr) in matrix.iter().enumerate() {
                            let all_true = puzzle_tests.iter()
                                .all(|&i| tcr.checks.get(i).map_or(false, |&(_, b)| b));
                            if all_true && index != tce.code as usize { valid_puzzle = false; }
                        }
                        if !valid_puzzle { continue; }
                        let uc_d: Vec<u8> = vec![uc_c.clone(), vec![tce.checks[d].0]].concat();
                        let xt_d: Vec<usize> = vec![xt_c.clone(), vec_test_couplings[d].clone()].concat();

                    }
                }
            }
        }

    });



    // // Classic Mode | Easy | Five Sections
    // let test_amount: u8 = 4;
    // let vec_centralized_tests: Vec<usize> = setup::generate_centralizing_test_list(&matrix, &test_amount);
    // matrix.clone().into_par_iter().for_each(|tce| {
    
    //     for a in 0..=45 {
    //         if vec_centralized_tests.contains(&a) { continue; }

    //         for b in a+1..=46 {
    //             if vec_centralized_tests.contains(&b) || tce.checks[a].0 == tce.checks[b].0 || vec_test_couplings[a].contains(&b) { continue; }
    //             let uc_b: Vec<u8> = vec![tce.checks[a].0, tce.checks[b].0];
    //             let xt_b: Vec<usize> = vec![vec_test_couplings[a].clone(), vec_test_couplings[b].clone()].concat();
                
    //             for c in b+1..=47 {
    //                 if vec_centralized_tests.contains(&c) || uc_b.contains(&tce.checks[c].0) || xt_b.contains(&c) { continue; }
    //                 let uc_c: Vec<u8> = vec![uc_b.clone(), vec![tce.checks[c].0]].concat();
    //                 let xt_c: Vec<usize> = vec![xt_b.clone(), vec_test_couplings[c].clone()].concat();

    //                 for d in c+1..=48 {
    //                     if vec_centralized_tests.contains(&d) || uc_c.contains(&tce.checks[d].0) || xt_c.contains(&d) { continue; }
    //                     let uc_d: Vec<u8> = vec![uc_c.clone(), vec![tce.checks[d].0]].concat();
    //                     let xt_d: Vec<usize> = vec![xt_c.clone(), vec_test_couplings[d].clone()].concat();

    //                     for e in d+1..=49 {
    //                         if vec_centralized_tests.contains(&e) || uc_d.contains(&tce.checks[e].0) || xt_d.contains(&e) { continue; }
                            
    //                         let puzzle_tests: Vec<usize> = vec![a, b, c, d];
    //                         let mut valid_puzzle: bool = true;
    //                         for (index, tcr) in matrix.iter().enumerate() {
    //                             let all_true = puzzle_tests.iter()
    //                                 .all(|&i| tcr.checks.get(i).map_or(false, |&(_, 
    //                         b)| b));
    //                             if all_true && index != tce.code as usize { valid_puzzle = false; }
    //                         }
    //                         if !valid_puzzle { continue; }
    //                         let uc_e: Vec<u8> = vec![uc_d.clone(), vec![tce.checks[e].0]].concat();
    //                         let xt_e: Vec<usize> = vec![xt_d.clone(), vec_test_couplings[e].clone()].concat();

    //                     }
    //                 }
    //             }
    //         }
    //     }

    // });



    // // Classic Mode | Easy | Six Sections
    // let test_amount: u8 = 4;
    // let vec_centralized_tests: Vec<usize> = setup::generate_centralizing_test_list(&matrix, &test_amount);
    // matrix.clone().into_par_iter().for_each(|tce| {
        
    //     for a in 0..=44 {
    //         if vec_centralized_tests.contains(&a) { continue; }

    //         for b in a+1..=45 {
    //             if vec_centralized_tests.contains(&b) || tce.checks[a].0 == tce.checks[b].0 || vec_test_couplings[a].contains(&b) { continue; }
    //             let uc_b: Vec<u8> = vec![tce.checks[a].0, tce.checks[b].0];
    //             let xt_b: Vec<usize> = vec![vec_test_couplings[a].clone(), vec_test_couplings[b].clone()].concat();
                
    //             for c in b+1..=46 {
    //                 if vec_centralized_tests.contains(&c) || uc_b.contains(&tce.checks[c].0) || xt_b.contains(&c) { continue; }
    //                 let uc_c: Vec<u8> = vec![uc_b.clone(), vec![tce.checks[c].0]].concat();
    //                 let xt_c: Vec<usize> = vec![xt_b.clone(), vec_test_couplings[c].clone()].concat();

    //                 for d in c+1..=47 {
    //                     if vec_centralized_tests.contains(&d) || uc_c.contains(&tce.checks[d].0) || xt_c.contains(&d) { continue; }
    //                     let uc_d: Vec<u8> = vec![uc_c.clone(), vec![tce.checks[d].0]].concat();
    //                     let xt_d: Vec<usize> = vec![xt_c.clone(), vec_test_couplings[d].clone()].concat();

    //                     for e in d+1..=48 {
    //                         if vec_centralized_tests.contains(&e) || uc_d.contains(&tce.checks[e].0) || xt_d.contains(&e) { continue; }
    //                         let uc_e: Vec<u8> = vec![uc_d.clone(), vec![tce.checks[e].0]].concat();
    //                         let xt_e: Vec<usize> = vec![xt_d.clone(), vec_test_couplings[e].clone()].concat();

    //                         for f in e+1..=49 {
    //                             if vec_centralized_tests.contains(&f) || uc_e.contains(&tce.checks[f].0) || xt_e.contains(&f) { continue; }
                                
    //                             let puzzle_tests: Vec<usize> = vec![a, b, c, d];
    //                             let mut valid_puzzle: bool = true;
    //                             for (index, tcr) in matrix.iter().enumerate() {
    //                                 let all_true = puzzle_tests.iter()
    //                                     .all(|&i| tcr.checks.get(i).map_or(false, |&(_, 
    //                             b)| b));
    //                                 if all_true && index != tce.code as usize { valid_puzzle = false; }
    //                             }
    //                             if !valid_puzzle { continue; }
                                
    //                             let uc_f: Vec<u8> = vec![uc_e.clone(), vec![tce.checks[f].0]].concat();
    //                             let xt_f: Vec<usize> = vec![xt_e.clone(), vec_test_couplings[f].clone()].concat();

    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }

    // });


}

