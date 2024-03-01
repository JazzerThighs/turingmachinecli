mod game_logic;
use crate::game_logic::*;
use serde_json::Value;
use std::{
    fs::File,
    io::Write,
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");

    // Original Turing Machine Game Puzzle Database Population
    generate_og_tm_puzzle_db(0 as usize, 49 as usize, "classic", "easy");
    generate_og_tm_puzzle_db(17 as usize, 62 as usize, "classic", "standard");
    generate_og_tm_puzzle_db(69 as usize, 182 as usize, "classic", "hard");

    generate_og_tm_puzzle_db(29 as usize, 71 as usize, "extreme", "easy");
    generate_og_tm_puzzle_db(29 as usize, 71 as usize, "extreme", "standard");
    generate_og_tm_puzzle_db(63 as usize, 182 as usize, "extreme", "hard");

    generate_og_tm_puzzle_db(29 as usize, 49 as usize, "nightmare", "easy");
    generate_og_tm_puzzle_db(17 as usize, 62 as usize, "nightmare", "standard");
    generate_og_tm_puzzle_db(66 as usize, 182 as usize, "nightmare", "hard");
}

fn generate_og_tm_puzzle_db(half_start: usize, upper_bound: usize, mode: &str, diff: &str) {
    let matrix: Vec<setup::TuringCodeEval> =
        setup::generate_results_matrix(111, 555, '1', '5', true);
    let vec_test_couplings: Vec<Vec<usize>> = setup::generate_coupled_criteria(&matrix);
    /****************************************************************************************************************************************************************************************
     ****************************************************************************************************************************************************************************************
     ****************************************************************************************************************************************************************************************/

    // Four Sections
    let test_amount: u8 = 4;
    let vec_centralized_tests: Vec<usize> =
        setup::generate_centralizing_test_list(&matrix, &test_amount);
    let puzzle_4_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    matrix
        .clone()
        .into_iter()
        .for_each(|tce: setup::TuringCodeEval| {
            let mut data: Vec<[usize; 4]> = vec![];
            let mut valid_puzzle: bool = true;

            for a in 0..=upper_bound - 3 {
                if !tce.checks[a].1 || vec_centralized_tests.contains(&a) {
                    continue;
                }

                for b in a + 1..=upper_bound - 2 {
                    if !tce.checks[b].1
                        || vec_centralized_tests.contains(&b)
                        || tce.checks[a].0 == tce.checks[b].0
                        || vec_test_couplings[a].contains(&b)
                    {
                        continue;
                    }

                    for c in half_start..=upper_bound - 1 {
                        if a >= c
                            || b >= c
                            || !tce.checks[c].1
                            || vec_centralized_tests.contains(&c)
                            || tce.checks[a].0 == tce.checks[c].0
                            || tce.checks[b].0 == tce.checks[c].0
                            || vec_test_couplings[a].contains(&c)
                            || vec_test_couplings[b].contains(&c)
                        {
                            continue;
                        }

                        for d in c + 1..=upper_bound {
                            if !tce.checks[d].1
                                || vec_centralized_tests.contains(&d)
                                || tce.checks[a].0 == tce.checks[d].0
                                || tce.checks[b].0 == tce.checks[d].0
                                || tce.checks[c].0 == tce.checks[d].0
                                || vec_test_couplings[a].contains(&d)
                                || vec_test_couplings[b].contains(&d)
                                || vec_test_couplings[c].contains(&d)
                            {
                                continue;
                            }

                            valid_puzzle = true;
                            for (_, t) in matrix.iter().enumerate() {
                                let all_true = [a, b, c, d]
                                    .iter()
                                    .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                                if all_true && t.code != tce.code {
                                    valid_puzzle = false;
                                    break;
                                }
                            }
                            if !valid_puzzle {
                                continue;
                            }

                            data.push([a, b, c, d]);
                        }
                    }
                }
            }
            
            puzzle_4_count.fetch_add(data.len(), Ordering::Relaxed);
            let mut json_data: serde_json::Map<String, Value> = serde_json::Map::new();
            for (index, vec) in data.iter().enumerate() {
                let formatted_string = vec
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<String>>()
                    .join("|");
                json_data.insert(index.to_string(), Value::String(formatted_string));
            }
            let tcestr: String = format!(
                "puzzle_db/og_tm_game/{mode}/{diff}/four/{}_og_c_ea_4.json",
                tce.code.to_string()
            );
            let json_path: &Path = Path::new(tcestr.as_str());
            let mut file: File = File::create(&json_path).expect("Failed to create file");
            file.write_all(serde_json::to_string(&json_data).ok().unwrap().as_bytes())
                .expect("Failed to write to file");
            print!("({mode}|{diff}|4: {} Done)", tce.code);
        });
    println!(
        "{mode}|{diff}|4: {:>9}",
        puzzle_4_count.load(Ordering::Relaxed)
    );

    // ****************************************************************************************************************************************************************************************

    // Five Sections
    let test_amount: u8 = 5;
    let vec_centralized_tests: Vec<usize> =
        setup::generate_centralizing_test_list(&matrix, &test_amount);
    let puzzle_5_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    matrix.clone().into_iter().for_each(|tce| {
        let mut data: Vec<[usize; 5]> = vec![];
        let mut valid_puzzle: bool = true;

        for a in 0..=upper_bound - 4 {
            if !tce.checks[a].1 || vec_centralized_tests.contains(&a) {
                continue;
            }

            for b in a + 1..=upper_bound - 3 {
                if !tce.checks[b].1
                    || vec_centralized_tests.contains(&b)
                    || tce.checks[a].0 == tce.checks[b].0
                    || vec_test_couplings[a].contains(&b)
                {
                    continue;
                }

                for c in half_start..=upper_bound - 2 {
                    if a >= c
                        || b >= c
                        || !tce.checks[c].1
                        || vec_centralized_tests.contains(&c)
                        || tce.checks[a].0 == tce.checks[c].0
                        || tce.checks[b].0 == tce.checks[c].0
                        || vec_test_couplings[a].contains(&c)
                        || vec_test_couplings[b].contains(&c)
                        
                    {
                        continue;
                    }

                    for d in c + 1..=upper_bound - 1 {
                        if !tce.checks[d].1
                            || vec_centralized_tests.contains(&d)
                            || tce.checks[a].0 == tce.checks[d].0
                            || tce.checks[b].0 == tce.checks[d].0
                            || tce.checks[c].0 == tce.checks[d].0
                            || vec_test_couplings[a].contains(&d)
                            || vec_test_couplings[b].contains(&d)
                            || vec_test_couplings[c].contains(&d)
                        {
                            continue;
                        }

                        for e in d + 1..=upper_bound {
                            if !tce.checks[e].1
                                || vec_centralized_tests.contains(&e)
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

                            valid_puzzle = true;
                            for (_, t) in matrix.iter().enumerate() {
                                let all_true = [a, b, c, d, e]
                                    .iter()
                                    .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                                if all_true && t.code != tce.code {
                                    valid_puzzle = false;
                                    break;
                                }
                            }
                            if !valid_puzzle {
                                continue;
                            }
                            
                            data.push([a, b, c, d, e]);
                        }
                    }
                }
            }
        }
        
        puzzle_5_count.fetch_add(data.len(), Ordering::Relaxed);
        let mut json_data: serde_json::Map<String, Value> = serde_json::Map::new();
        for (index, vec) in data.iter().enumerate() {
            let formatted_string = vec
                .iter()
                .map(usize::to_string)
                .collect::<Vec<String>>()
                .join("|");
            json_data.insert(index.to_string(), Value::String(formatted_string));
        }
        let tcestr: String = format!(
            "puzzle_db/og_tm_game/{mode}/{diff}/five/{}_og_c_ea_5.json",
            tce.code.to_string()
        );
        let json_path: &Path = Path::new(tcestr.as_str());
        let mut file: File = File::create(&json_path).expect("Failed to create file");
        file.write_all(serde_json::to_string(&json_data).ok().unwrap().as_bytes())
            .expect("Failed to write to file");
        print!("({mode}|{diff}|5: {} Done)", tce.code);
    });
    println!(
        "{mode}|{diff}|5: {:>9}",
        puzzle_5_count.load(Ordering::Relaxed)
    );

    // ****************************************************************************************************************************************************************************************

    // Six Sections
    let test_amount: u8 = 6;
    let vec_centralized_tests: Vec<usize> =
        setup::generate_centralizing_test_list(&matrix, &test_amount);
    let puzzle_6_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    matrix.clone().into_iter().for_each(|tce| {
        let mut data: Vec<[usize; 6]> = vec![];
        let mut valid_puzzle: bool = true;

        for a in 0..=upper_bound - 5 {
            if !tce.checks[a].1 || vec_centralized_tests.contains(&a) {
                continue;
            }

            for b in a + 1..=upper_bound - 4 {
                if !tce.checks[b].1
                    || vec_centralized_tests.contains(&b)
                    || tce.checks[a].0 == tce.checks[b].0
                    || vec_test_couplings[a].contains(&b)
                {
                    continue;
                }

                for c in b + 1..=upper_bound - 3 {
                    if !tce.checks[c].1
                        || vec_centralized_tests.contains(&c)
                        || tce.checks[a].0 == tce.checks[c].0
                        || tce.checks[b].0 == tce.checks[c].0
                        || vec_test_couplings[a].contains(&c)
                        || vec_test_couplings[b].contains(&c)
                    {
                        continue;
                    }

                    for d in half_start..=upper_bound - 2 {
                        if a >= d
                            || b >= d
                            || c >= d
                            || !tce.checks[d].1
                            || vec_centralized_tests.contains(&d)
                            || tce.checks[a].0 == tce.checks[d].0
                            || tce.checks[b].0 == tce.checks[d].0
                            || tce.checks[c].0 == tce.checks[d].0
                            || vec_test_couplings[a].contains(&d)
                            || vec_test_couplings[b].contains(&d)
                            || vec_test_couplings[c].contains(&d)
                        {
                            continue;
                        }

                        for e in d + 1..=upper_bound - 1 {
                            if !tce.checks[e].1
                                || vec_centralized_tests.contains(&e)
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

                            for f in e + 1..=upper_bound {
                                if !tce.checks[f].1
                                    || vec_centralized_tests.contains(&f)
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

                                valid_puzzle = true;
                                for (_, t) in matrix.iter().enumerate() {
                                    let all_true = [a, b, c, d, e, f]
                                        .iter()
                                        .all(|&i| t.checks.get(i).map_or(false, |&(_, b)| b));
                                    if all_true && t.code != tce.code {
                                        valid_puzzle = false;
                                        break;
                                    }
                                }
                                if !valid_puzzle {
                                    continue;
                                }
                                
                                data.push([a, b, c, d, e, f]);
                            }
                        }
                    }
                }
            }
        }
        
        puzzle_6_count.fetch_add(data.len(), Ordering::Relaxed);
        let mut json_data: serde_json::Map<String, Value> = serde_json::Map::new();
        for (index, vec) in data.iter().enumerate() {
            let formatted_string = vec
                .iter()
                .map(usize::to_string)
                .collect::<Vec<String>>()
                .join("|");
            json_data.insert(index.to_string(), Value::String(formatted_string));
        }
        let tcestr: String = format!(
            "puzzle_db/og_tm_game/{mode}/{diff}/six/{}_og_c_ea_6.json",
            tce.code.to_string()
        );
        let json_path: &Path = Path::new(tcestr.as_str());
        let mut file: File = File::create(&json_path).expect("Failed to create file");
        file.write_all(serde_json::to_string(&json_data).ok().unwrap().as_bytes())
            .expect("Failed to write to file");
        print!("({mode}|{diff}|6: {} Done)", tce.code);
    });
    println!(
        "{mode}|{diff}|6: {:>9}",
        puzzle_6_count.load(Ordering::Relaxed)
    );
    println!(
        "{mode}|{diff} Total: {:>9}\n",
        puzzle_4_count.load(Ordering::Relaxed)
            + puzzle_5_count.load(Ordering::Relaxed)
            + puzzle_6_count.load(Ordering::Relaxed)
    );
}
