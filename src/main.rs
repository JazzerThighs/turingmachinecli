mod game_logic;
use crate::game_logic::*;
use rayon::prelude::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
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
    println!("Generating Vector of Test Couplings...");
    let couplings: Vec<Vec<usize>> = setup::generate_coupled_criteria(&matrix);
    println!("Generating 4-Puzzle, 5-Puzzle, and 6-Puzzle Centralizing Tests...");
    let mut vct_4: Vec<usize> = vec![];
    let mut vct_5: Vec<usize> = vec![];
    let mut vct_6: Vec<usize> = vec![];
    for x in 0..matrix[0].checks.len() {
        let mut count: u8 = 0;
        for y in 0..matrix.len() {
            if matrix[y].checks[x].1 {
                count += 1;
            }
        }
        if count < 4 {
            vct_4.push(x);
        }
        if count < 5 {
            vct_5.push(x);
        }
        if count < 6 {
            vct_6.push(x);
        }
    }
    let puzzle_4_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let puzzle_5_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let puzzle_6_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    (0..matrix[0].checks.len() - 3)
        .into_par_iter()
        .filter(|a: &usize| !vct_4.contains(a))
        .for_each(|a: usize| {
            for b in a + 1..matrix[0].checks.len() - 2 {
                if vct_4.contains(&b)
                    || matrix[0].checks[a].0 == matrix[0].checks[b].0
                    || couplings[a].contains(&b)
                {
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
                    if vct_4.contains(&c)
                        || matrix[0].checks[a].0 == matrix[0].checks[c].0
                        || matrix[0].checks[b].0 == matrix[0].checks[c].0
                        || couplings[a].contains(&c)
                        || couplings[b].contains(&c)
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
                        if vct_4.contains(&d)
                            || matrix[0].checks[a].0 == matrix[0].checks[d].0
                            || matrix[0].checks[b].0 == matrix[0].checks[d].0
                            || matrix[0].checks[c].0 == matrix[0].checks[d].0
                            || couplings[a].contains(&d)
                            || couplings[b].contains(&d)
                            || couplings[c].contains(&d)
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
    println!(
        "Total # of Puzzles with 4 Criteria Cards: {:>9}",
        puzzle_4_count.load(Ordering::Relaxed)
    );

    (0..matrix[0].checks.len() - 4)
        .into_par_iter()
        .filter(|a: &usize| !vct_5.contains(a))
        .for_each(|a: usize| {
            for b in a + 1..matrix[0].checks.len() - 3 {
                if vct_5.contains(&b)
                    || matrix[0].checks[a].0 == matrix[0].checks[b].0
                    || couplings[a].contains(&b)
                {
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
                    if vct_5.contains(&c)
                        || matrix[0].checks[a].0 == matrix[0].checks[c].0
                        || matrix[0].checks[b].0 == matrix[0].checks[c].0
                        || couplings[a].contains(&c)
                        || couplings[b].contains(&c)
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
                        if vct_5.contains(&d)
                            || matrix[0].checks[a].0 == matrix[0].checks[d].0
                            || matrix[0].checks[b].0 == matrix[0].checks[d].0
                            || matrix[0].checks[c].0 == matrix[0].checks[d].0
                            || couplings[a].contains(&d)
                            || couplings[b].contains(&d)
                            || couplings[c].contains(&d)
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
                            if vct_5.contains(&e)
                                || matrix[0].checks[a].0 == matrix[0].checks[e].0
                                || matrix[0].checks[b].0 == matrix[0].checks[e].0
                                || matrix[0].checks[c].0 == matrix[0].checks[e].0
                                || matrix[0].checks[d].0 == matrix[0].checks[e].0
                                || couplings[a].contains(&e)
                                || couplings[b].contains(&e)
                                || couplings[c].contains(&e)
                                || couplings[d].contains(&e)
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
    println!(
        "Total # of Puzzles with 5 Criteria Cards: {:>9}",
        puzzle_5_count.load(Ordering::Relaxed)
    );

    (0..matrix[0].checks.len() - 5)
        .into_par_iter()
        .filter(|a: &usize| !vct_6.contains(a))
        .for_each(|a: usize| {
            for b in a + 1..matrix[0].checks.len() - 4 {
                if vct_6.contains(&b)
                    || matrix[0].checks[a].0 == matrix[0].checks[b].0
                    || couplings[a].contains(&b)
                {
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
                    if vct_6.contains(&c)
                        || matrix[0].checks[a].0 == matrix[0].checks[c].0
                        || matrix[0].checks[b].0 == matrix[0].checks[c].0
                        || couplings[a].contains(&c)
                        || couplings[b].contains(&c)
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
                        if vct_6.contains(&d)
                            || matrix[0].checks[a].0 == matrix[0].checks[d].0
                            || matrix[0].checks[b].0 == matrix[0].checks[d].0
                            || matrix[0].checks[c].0 == matrix[0].checks[d].0
                            || couplings[a].contains(&d)
                            || couplings[b].contains(&d)
                            || couplings[c].contains(&d)
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
                            if vct_6.contains(&e)
                                || matrix[0].checks[a].0 == matrix[0].checks[e].0
                                || matrix[0].checks[b].0 == matrix[0].checks[e].0
                                || matrix[0].checks[c].0 == matrix[0].checks[e].0
                                || matrix[0].checks[d].0 == matrix[0].checks[e].0
                                || couplings[a].contains(&e)
                                || couplings[b].contains(&e)
                                || couplings[c].contains(&e)
                                || couplings[d].contains(&e)
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
                                if vct_6.contains(&f)
                                    || matrix[0].checks[a].0 == matrix[0].checks[f].0
                                    || matrix[0].checks[b].0 == matrix[0].checks[f].0
                                    || matrix[0].checks[c].0 == matrix[0].checks[f].0
                                    || matrix[0].checks[d].0 == matrix[0].checks[f].0
                                    || matrix[0].checks[e].0 == matrix[0].checks[f].0
                                    || couplings[a].contains(&f)
                                    || couplings[b].contains(&f)
                                    || couplings[c].contains(&f)
                                    || couplings[d].contains(&f)
                                    || couplings[e].contains(&f)
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
