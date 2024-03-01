mod game_logic;
use crate::game_logic::*;
use rayon::prelude::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

fn looppahh() {
    for a in 0..=2 {
        println!("a: {a}");
        for b in a + 1..=2 {
            println!("b:{b}");
        }
    }
}

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");

    // Original Turing Machine Game Puzzle Database Population
    looppahh();
    generate_og_tm_puzzle_db();
}

fn generate_og_tm_puzzle_db() {
    let matrix: Vec<setup::TuringCodeEval> =
        setup::generate_results_matrix(111, 555, '1', '5', true);
    let vec_test_couplings: Vec<Vec<usize>> = setup::generate_coupled_criteria(&matrix);
    /****************************************************************************************************************************************************************************************
     ****************************************************************************************************************************************************************************************
     ****************************************************************************************************************************************************************************************/

    // Four Sections
    let vec_centralized_tests_4: Vec<usize> =
        setup::generate_centralizing_test_list(&matrix, &(4 as u8));
    let puzzle_4_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    matrix
        .clone()
        .into_par_iter()
        .for_each(|tce: setup::TuringCodeEval| {
            #[allow(unused_assignments)]
            let mut valid_puzzle_4: bool = true;

            for a in 0..=179 {
                if !tce.checks[a].1 || vec_centralized_tests_4.contains(&a) {
                    continue;
                }

                for b in a + 1..=180 {
                    if !tce.checks[b].1
                        || vec_centralized_tests_4.contains(&b)
                        || tce.checks[a].0 == tce.checks[b].0
                        || vec_test_couplings[a].contains(&b)
                    {
                        continue;
                    }

                    let mut valid_incomplete_puzzle: u8 = 0;
                    for (_, t) in matrix.iter().enumerate() {
                        let all_true = [a, b]
                            .iter()
                            .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                        if all_true {
                            valid_incomplete_puzzle += 1;
                        }
                    }
                    if valid_incomplete_puzzle < 3 {
                        continue;
                    }

                    for c in b + 1..=181 {
                        if a >= c
                            || b >= c
                            || !tce.checks[c].1
                            || vec_centralized_tests_4.contains(&c)
                            || tce.checks[a].0 == tce.checks[c].0
                            || tce.checks[b].0 == tce.checks[c].0
                            || vec_test_couplings[a].contains(&c)
                            || vec_test_couplings[b].contains(&c)
                        {
                            continue;
                        }

                        let mut valid_incomplete_puzzle: u8 = 0;
                        for (_, t) in matrix.iter().enumerate() {
                            let all_true = [a, b, c]
                                .iter()
                                .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                            if all_true {
                                valid_incomplete_puzzle += 1;
                            }
                        }
                        if valid_incomplete_puzzle < 2 {
                            continue;
                        }

                        for d in c + 1..=182 {
                            if !tce.checks[d].1
                                || vec_centralized_tests_4.contains(&d)
                                || tce.checks[a].0 == tce.checks[d].0
                                || tce.checks[b].0 == tce.checks[d].0
                                || tce.checks[c].0 == tce.checks[d].0
                                || vec_test_couplings[a].contains(&d)
                                || vec_test_couplings[b].contains(&d)
                                || vec_test_couplings[c].contains(&d)
                            {
                                continue;
                            }

                            valid_puzzle_4 = true;
                            for (_, t) in matrix.iter().enumerate() {
                                let all_true = [a, b, c, d]
                                    .iter()
                                    .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                                if all_true && t.code != tce.code {
                                    valid_puzzle_4 = false;
                                    break;
                                }
                            }
                            if valid_puzzle_4 {
                                puzzle_4_count.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                }
            }
        });
    println!(
        "Total # of Puzzles with 4 Criteria Cards: {:>9}",
        puzzle_4_count.load(Ordering::Relaxed)
    );

    // ****************************************************************************************************************************************************************************************

    // Five Sections
    let vec_centralized_tests_5: Vec<usize> =
        setup::generate_centralizing_test_list(&matrix, &(5 as u8));
    let puzzle_5_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    matrix.clone().into_par_iter().for_each(|tce| {
        #[allow(unused_assignments)]
        let mut valid_puzzle_5: bool = true;

        for a in 0..=178 {
            if !tce.checks[a].1 || vec_centralized_tests_5.contains(&a) {
                continue;
            }

            for b in a + 1..=179 {
                if !tce.checks[b].1
                    || vec_centralized_tests_5.contains(&b)
                    || tce.checks[a].0 == tce.checks[b].0
                    || vec_test_couplings[a].contains(&b)
                {
                    continue;
                }

                let mut valid_incomplete_puzzle: u8 = 0;
                for (_, t) in matrix.iter().enumerate() {
                    let all_true = [a, b]
                        .iter()
                        .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                    if all_true {
                        valid_incomplete_puzzle += 1;
                    }
                }
                if valid_incomplete_puzzle < 4 {
                    continue;
                }

                for c in b + 1..=180 {
                    if !tce.checks[c].1
                        || vec_centralized_tests_5.contains(&c)
                        || tce.checks[a].0 == tce.checks[c].0
                        || tce.checks[b].0 == tce.checks[c].0
                        || vec_test_couplings[a].contains(&c)
                        || vec_test_couplings[b].contains(&c)
                    {
                        continue;
                    }

                    let mut valid_incomplete_puzzle: u8 = 0;
                    for (_, t) in matrix.iter().enumerate() {
                        let all_true = [a, b, c]
                            .iter()
                            .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                        if all_true {
                            valid_incomplete_puzzle += 1;
                        }
                    }
                    if valid_incomplete_puzzle < 3 {
                        continue;
                    }

                    for d in c + 1..=181 {
                        if !tce.checks[d].1
                            || vec_centralized_tests_5.contains(&d)
                            || tce.checks[a].0 == tce.checks[d].0
                            || tce.checks[b].0 == tce.checks[d].0
                            || tce.checks[c].0 == tce.checks[d].0
                            || vec_test_couplings[a].contains(&d)
                            || vec_test_couplings[b].contains(&d)
                            || vec_test_couplings[c].contains(&d)
                        {
                            continue;
                        }

                        let mut valid_incomplete_puzzle: u8 = 0;
                        for (_, t) in matrix.iter().enumerate() {
                            let all_true = [a, b, c, d]
                                .iter()
                                .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                            if all_true {
                                valid_incomplete_puzzle += 1;
                            }
                        }
                        if valid_incomplete_puzzle < 2 {
                            continue;
                        }

                        for e in d + 1..=182 {
                            if !tce.checks[e].1
                                || vec_centralized_tests_5.contains(&e)
                                || tce.checks[a].0 == tce.checks[e].0
                                || tce.checks[b].0 == tce.checks[e].0
                                || tce.checks[c].0 == tce.checks[e].0
                                || tce.checks[d].0 == tce.checks[e].0
                                || vec_test_couplings[a].contains(&e)
                                || vec_test_couplings[b].contains(&e)
                                || vec_test_couplings[c].contains(&e)
                                || vec_test_couplings[d].contains(&e)
                            {
                                continue;
                            }

                            valid_puzzle_5 = true;
                            for (_, t) in matrix.iter().enumerate() {
                                let all_true = [a, b, c, d, e]
                                    .iter()
                                    .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                                if all_true && t.code != tce.code {
                                    valid_puzzle_5 = false;
                                    break;
                                }
                            }
                            if valid_puzzle_5 {
                                puzzle_5_count.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                }
            }
        }
    });
    println!(
        "Total # of Puzzles with 5 Criteria Cards: {:>9}",
        puzzle_5_count.load(Ordering::Relaxed)
    );

    // ****************************************************************************************************************************************************************************************

    // Six Sections
    let vec_centralized_tests_6: Vec<usize> =
        setup::generate_centralizing_test_list(&matrix, &(6 as u8));
    let puzzle_6_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    matrix.clone().into_par_iter().for_each(|tce| {
        #[allow(unused_assignments)]
        let mut valid_puzzle_6: bool = true;

        for a in 0..=177 {
            if !tce.checks[a].1 || vec_centralized_tests_6.contains(&a) {
                continue;
            }

            for b in a + 1..=178 {
                if !tce.checks[b].1
                    || vec_centralized_tests_6.contains(&b)
                    || tce.checks[a].0 == tce.checks[b].0
                    || vec_test_couplings[a].contains(&b)
                {
                    continue;
                }

                let mut valid_incomplete_puzzle: u8 = 0;
                for (_, t) in matrix.iter().enumerate() {
                    let all_true = [a, b]
                        .iter()
                        .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                    if all_true {
                        valid_incomplete_puzzle += 1;
                    }
                }
                if valid_incomplete_puzzle < 5 {
                    continue;
                }

                for c in b + 1..=179 {
                    if !tce.checks[c].1
                        || vec_centralized_tests_6.contains(&c)
                        || tce.checks[a].0 == tce.checks[c].0
                        || tce.checks[b].0 == tce.checks[c].0
                        || vec_test_couplings[a].contains(&c)
                        || vec_test_couplings[b].contains(&c)
                    {
                        continue;
                    }

                    let mut valid_incomplete_puzzle: u8 = 0;
                    for (_, t) in matrix.iter().enumerate() {
                        let all_true = [a, b, c]
                            .iter()
                            .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                        if all_true {
                            valid_incomplete_puzzle += 1;
                        }
                    }
                    if valid_incomplete_puzzle < 4 {
                        continue;
                    }

                    for d in c + 1..=180 {
                        if !tce.checks[d].1
                            || vec_centralized_tests_6.contains(&d)
                            || tce.checks[a].0 == tce.checks[d].0
                            || tce.checks[b].0 == tce.checks[d].0
                            || tce.checks[c].0 == tce.checks[d].0
                            || vec_test_couplings[a].contains(&d)
                            || vec_test_couplings[b].contains(&d)
                            || vec_test_couplings[c].contains(&d)
                        {
                            continue;
                        }

                        let mut valid_incomplete_puzzle: u8 = 0;
                        for (_, t) in matrix.iter().enumerate() {
                            let all_true = [a, b, c, d]
                                .iter()
                                .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                            if all_true {
                                valid_incomplete_puzzle += 1;
                            }
                        }
                        if valid_incomplete_puzzle < 3 {
                            continue;
                        }

                        for e in d + 1..=181 {
                            if !tce.checks[e].1
                                || vec_centralized_tests_6.contains(&e)
                                || tce.checks[a].0 == tce.checks[e].0
                                || tce.checks[b].0 == tce.checks[e].0
                                || tce.checks[c].0 == tce.checks[e].0
                                || tce.checks[d].0 == tce.checks[e].0
                                || vec_test_couplings[a].contains(&e)
                                || vec_test_couplings[b].contains(&e)
                                || vec_test_couplings[c].contains(&e)
                                || vec_test_couplings[d].contains(&e)
                            {
                                continue;
                            }

                            let mut valid_incomplete_puzzle: u8 = 0;
                            for (_, t) in matrix.iter().enumerate() {
                                let all_true = [a, b, c, d, e]
                                    .iter()
                                    .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                                if all_true {
                                    valid_incomplete_puzzle += 1;
                                }
                            }
                            if valid_incomplete_puzzle < 2 {
                                continue;
                            }

                            for f in e + 1..=182 {
                                if !tce.checks[f].1
                                    || vec_centralized_tests_6.contains(&f)
                                    || tce.checks[a].0 == tce.checks[f].0
                                    || tce.checks[b].0 == tce.checks[f].0
                                    || tce.checks[c].0 == tce.checks[f].0
                                    || tce.checks[d].0 == tce.checks[f].0
                                    || tce.checks[e].0 == tce.checks[f].0
                                    || vec_test_couplings[a].contains(&f)
                                    || vec_test_couplings[b].contains(&f)
                                    || vec_test_couplings[c].contains(&f)
                                    || vec_test_couplings[d].contains(&f)
                                    || vec_test_couplings[e].contains(&f)
                                {
                                    continue;
                                }

                                valid_puzzle_6 = true;
                                for (_, t) in matrix.iter().enumerate() {
                                    let all_true = [a, b, c, d, e, f]
                                        .iter()
                                        .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                                    if all_true && t.code != tce.code {
                                        valid_puzzle_6 = false;
                                        break;
                                    }
                                }
                                if valid_puzzle_6 {
                                    puzzle_6_count.fetch_add(1, Ordering::Relaxed);
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    println!(
        "Total # of Puzzles with 6 Criteria Cards: {:>9}",
        puzzle_6_count.load(Ordering::Relaxed)
    );
    println!(
        "Total Valid Turing Machine Unbound Classic Puzzles: {:>9}\n(Extreme Mode == (Total * (177!/6!))\n(Nightmare Mode == (Total * 6!)\n(Extreme&Nightmare Mode == (Total * 177!)",
        puzzle_4_count.load(Ordering::Relaxed)
            + puzzle_5_count.load(Ordering::Relaxed)
            + puzzle_6_count.load(Ordering::Relaxed)
    );
}
