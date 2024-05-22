mod game_logic;
use crate::game_logic::*;
use rayon::prelude::*;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game called \"Turing Machine\" designed by Fabien Gridel & Yoann Levet.\n~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~\n");

    // Original Turing Machine Game Puzzle Database Population
    generate_og_tm_puzzle_db();
}

fn generate_og_tm_puzzle_db() {
    println!("Generating Matrix...");
    let matrix: Vec<setup::TuringCodeEval> =
        setup::generate_results_matrix(111, 555, '1', '5', true);
    println!("Generating Matrix of Test Couplings...");
    let mut couplings: [[bool; 183]; 183] = [[false; 183]; 183];
    setup::generate_coupled_criteria(&matrix, &mut couplings);
    let couplings: [[bool; 183]; 183] = couplings;
    println!("Generating 4-Puzzle, 5-Puzzle, and 6-Puzzle Centralizing Tests...");
    let mut vct_4: [bool; 183] = [false; 183];
    let mut vct_5: [bool; 183] = [false; 183];
    let mut vct_6: [bool; 183] = [false; 183];
    for x in 0..matrix[0].checks.len() {
        let mut count: u8 = 0;
        for y in 0..matrix.len() {
            if matrix[y].checks[x].1 {
                count += 1;
            }
        }
        if count < 4 {
            vct_4[x] = true;
        }
        if count < 5 {
            vct_5[x] = true;
        }
        if count < 6 {
            vct_6[x] = true;
        }
    }
    let puzzle_4_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let puzzle_5_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let puzzle_6_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    let p4_start: Instant = Instant::now();

    (0..matrix[0].checks.len() - 3)
        .into_par_iter()
        .filter(|a: &usize| vct_4[*a] == false)
        .for_each(|a: usize| {
            for b in a + 1..matrix[0].checks.len() - 2 {
                if vct_4[b] || matrix[0].checks[a].0 == matrix[0].checks[b].0 || couplings[a][b] {
                    continue;
                }

                let mut p2_count: u8 = 0;
                let mut p2: bool = false;
                for x in 0..matrix.len() {
                    if matrix[x].checks[a].1 && matrix[x].checks[b].1 {
                        p2_count += 1;
                        if p2_count >= 3 {
                            p2 = true;
                            break;
                        }
                    }
                }
                if !p2 {
                    continue;
                }

                for c in b + 1..matrix[0].checks.len() - 1 {
                    if vct_4[c]
                        || matrix[0].checks[a].0 == matrix[0].checks[c].0
                        || matrix[0].checks[b].0 == matrix[0].checks[c].0
                        || couplings[a][c]
                        || couplings[b][c]
                    {
                        continue;
                    }

                    let mut p3_count: u8 = 0;
                    let mut p3: bool = false;
                    for x in 0..matrix.len() {
                        if matrix[x].checks[a].1 && matrix[x].checks[b].1 && matrix[x].checks[c].1 {
                            p3_count += 1;
                            if p3_count >= 2 {
                                p3 = true;
                                break;
                            }
                        }
                    }
                    if !p3 {
                        continue;
                    }

                    for d in c + 1..matrix[0].checks.len() {
                        if vct_4[d]
                            || matrix[0].checks[a].0 == matrix[0].checks[d].0
                            || matrix[0].checks[b].0 == matrix[0].checks[d].0
                            || matrix[0].checks[c].0 == matrix[0].checks[d].0
                            || couplings[a][d]
                            || couplings[b][d]
                            || couplings[c][d]
                        {
                            continue;
                        }

                        let mut p4_count: u8 = 0;
                        let mut p4_fail: bool = false;
                        for x in 0..matrix.len() {
                            if matrix[x].checks[a].1
                                && matrix[x].checks[b].1
                                && matrix[x].checks[c].1
                                && matrix[x].checks[d].1
                            {
                                p4_count += 1;
                                if p4_count > 1 {
                                    p4_fail = true;
                                    break;
                                }
                            }
                        }
                        if !p4_fail && p4_count == 1 {
                            puzzle_4_count.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
            }
        });

    let p4_duration: Duration = p4_start.elapsed();
    println!("Duration of p4 calculation: {p4_duration:?}");
    println!(
        "Total # of Puzzles with 4 Criteria Cards: {:>9}",
        puzzle_4_count.load(Ordering::Relaxed)
    );

    let p5_start: Instant = Instant::now();

    (0..matrix[0].checks.len() - 4)
        .into_par_iter()
        .filter(|a: &usize| vct_5[*a] == false)
        .for_each(|a: usize| {
            for b in a + 1..matrix[0].checks.len() - 3 {
                if vct_5[b] || matrix[0].checks[a].0 == matrix[0].checks[b].0 || couplings[a][b] {
                    continue;
                }

                let mut p2_count: u8 = 0;
                let mut p2: bool = false;
                for x in 0..matrix.len() {
                    if matrix[x].checks[a].1 && matrix[x].checks[b].1 {
                        p2_count += 1;
                        if p2_count >= 4 {
                            p2 = true;
                            break;
                        }
                    }
                }
                if !p2 {
                    continue;
                }

                for c in b + 1..matrix[0].checks.len() - 2 {
                    if vct_5[c]
                        || matrix[0].checks[a].0 == matrix[0].checks[c].0
                        || matrix[0].checks[b].0 == matrix[0].checks[c].0
                        || couplings[a][c]
                        || couplings[b][c]
                    {
                        continue;
                    }

                    let mut p3_count: u8 = 0;
                    let mut p3: bool = false;
                    for x in 0..matrix.len() {
                        if matrix[x].checks[a].1 && matrix[x].checks[b].1 && matrix[x].checks[c].1 {
                            p3_count += 1;
                            if p3_count >= 3 {
                                p3 = true;
                                break;
                            }
                        }
                    }
                    if !p3 {
                        continue;
                    }

                    for d in c + 1..matrix[0].checks.len() - 1 {
                        if vct_5[d]
                            || matrix[0].checks[a].0 == matrix[0].checks[d].0
                            || matrix[0].checks[b].0 == matrix[0].checks[d].0
                            || matrix[0].checks[c].0 == matrix[0].checks[d].0
                            || couplings[a][d]
                            || couplings[b][d]
                            || couplings[c][d]
                        {
                            continue;
                        }

                        let mut p4_count: u8 = 0;
                        let mut p4: bool = false;
                        for x in 0..matrix.len() {
                            if matrix[x].checks[a].1
                                && matrix[x].checks[b].1
                                && matrix[x].checks[c].1
                                && matrix[x].checks[d].1
                            {
                                p4_count += 1;
                                if p4_count >= 2 {
                                    p4 = true;
                                    break;
                                }
                            }
                        }
                        if !p4 {
                            continue;
                        }

                        for e in d + 1..matrix[0].checks.len() {
                            if vct_5[e]
                                || matrix[0].checks[a].0 == matrix[0].checks[e].0
                                || matrix[0].checks[b].0 == matrix[0].checks[e].0
                                || matrix[0].checks[c].0 == matrix[0].checks[e].0
                                || matrix[0].checks[d].0 == matrix[0].checks[e].0
                                || couplings[a][e]
                                || couplings[b][e]
                                || couplings[c][e]
                                || couplings[d][e]
                            {
                                continue;
                            }

                            let mut p5_count: u8 = 0;
                            let mut p5_fail: bool = false;
                            for x in 0..matrix.len() {
                                if matrix[x].checks[a].1
                                    && matrix[x].checks[b].1
                                    && matrix[x].checks[c].1
                                    && matrix[x].checks[d].1
                                    && matrix[x].checks[e].1
                                {
                                    p5_count += 1;
                                    if p5_count > 1 {
                                        p5_fail = true;
                                        break;
                                    }
                                }
                            }
                            if !p5_fail && p5_count == 1 {
                                puzzle_5_count.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                }
            }
        });

    let p5_duration: Duration = p5_start.elapsed();
    println!("Duration of p5 calculation: {p5_duration:?}");
    println!(
        "Total # of Puzzles with 5 Criteria Cards: {:>9}",
        puzzle_5_count.load(Ordering::Relaxed)
    );

    let p6_start: Instant = Instant::now();

    (0..matrix[0].checks.len() - 5)
        .into_par_iter()
        .filter(|a: &usize| vct_6[*a] == false)
        .for_each(|a: usize| {
            for b in a + 1..matrix[0].checks.len() - 4 {
                if vct_6[b] || matrix[0].checks[a].0 == matrix[0].checks[b].0 || couplings[a][b] {
                    continue;
                }

                let mut p2_count: u8 = 0;
                let mut p2: bool = false;
                for x in 0..matrix.len() {
                    if matrix[x].checks[a].1 && matrix[x].checks[b].1 {
                        p2_count += 1;
                        if p2_count >= 5 {
                            p2 = true;
                            break;
                        }
                    }
                }
                if !p2 {
                    continue;
                }

                for c in b + 1..matrix[0].checks.len() - 3 {
                    if vct_6[c]
                        || matrix[0].checks[a].0 == matrix[0].checks[c].0
                        || matrix[0].checks[b].0 == matrix[0].checks[c].0
                        || couplings[a][c]
                        || couplings[b][c]
                    {
                        continue;
                    }

                    let mut p3_count: u8 = 0;
                    let mut p3: bool = false;
                    for x in 0..matrix.len() {
                        if matrix[x].checks[a].1 && matrix[x].checks[b].1 && matrix[x].checks[c].1 {
                            p3_count += 1;
                            if p3_count >= 4 {
                                p3 = true;
                                break;
                            }
                        }
                    }
                    if !p3 {
                        continue;
                    }

                    for d in c + 1..matrix[0].checks.len() - 2 {
                        if vct_6[d]
                            || matrix[0].checks[a].0 == matrix[0].checks[d].0
                            || matrix[0].checks[b].0 == matrix[0].checks[d].0
                            || matrix[0].checks[c].0 == matrix[0].checks[d].0
                            || couplings[a][d]
                            || couplings[b][d]
                            || couplings[c][d]
                        {
                            continue;
                        }

                        let mut p4_count: u8 = 0;
                        let mut p4: bool = false;
                        for x in 0..matrix.len() {
                            if matrix[x].checks[a].1
                                && matrix[x].checks[b].1
                                && matrix[x].checks[c].1
                                && matrix[x].checks[d].1
                            {
                                p4_count += 1;
                                if p4_count >= 3 {
                                    p4 = true;
                                    break;
                                }
                            }
                        }
                        if !p4 {
                            continue;
                        }

                        for e in d + 1..matrix[0].checks.len() - 1 {
                            if vct_6[e]
                                || matrix[0].checks[a].0 == matrix[0].checks[e].0
                                || matrix[0].checks[b].0 == matrix[0].checks[e].0
                                || matrix[0].checks[c].0 == matrix[0].checks[e].0
                                || matrix[0].checks[d].0 == matrix[0].checks[e].0
                                || couplings[a][e]
                                || couplings[b][e]
                                || couplings[c][e]
                                || couplings[d][e]
                            {
                                continue;
                            }

                            let mut p5_count: u8 = 0;
                            let mut p5: bool = false;
                            for x in 0..matrix.len() {
                                if matrix[x].checks[a].1
                                    && matrix[x].checks[b].1
                                    && matrix[x].checks[c].1
                                    && matrix[x].checks[d].1
                                    && matrix[x].checks[e].1
                                {
                                    p5_count += 1;
                                    if p5_count >= 2 {
                                        p5 = true;
                                        break;
                                    }
                                }
                            }
                            if !p5 {
                                continue;
                            }

                            for f in e + 1..matrix[0].checks.len() {
                                if vct_6[f]
                                    || matrix[0].checks[a].0 == matrix[0].checks[f].0
                                    || matrix[0].checks[b].0 == matrix[0].checks[f].0
                                    || matrix[0].checks[c].0 == matrix[0].checks[f].0
                                    || matrix[0].checks[d].0 == matrix[0].checks[f].0
                                    || matrix[0].checks[e].0 == matrix[0].checks[f].0
                                    || couplings[a][f]
                                    || couplings[b][f]
                                    || couplings[c][f]
                                    || couplings[d][f]
                                    || couplings[e][f]
                                {
                                    continue;
                                }

                                let mut p6_count: u8 = 0;
                                let mut p6_fail: bool = false;
                                for x in 0..matrix.len() {
                                    if matrix[x].checks[a].1
                                        && matrix[x].checks[b].1
                                        && matrix[x].checks[c].1
                                        && matrix[x].checks[d].1
                                        && matrix[x].checks[e].1
                                        && matrix[x].checks[f].1
                                    {
                                        p6_count += 1;
                                        if p6_count > 1 {
                                            p6_fail = true;
                                            break;
                                        }
                                    }
                                }
                                if !p6_fail && p6_count == 1 {
                                    puzzle_6_count.fetch_add(1, Ordering::Relaxed);
                                }
                            }
                        }
                    }
                }
            }
        });

    let p6_duration: Duration = p6_start.elapsed();
    println!("Duration of p6 calculation: {p6_duration:?}");
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
