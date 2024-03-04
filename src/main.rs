mod game_logic;
use crate::game_logic::*;
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
    let matrix: Vec<setup::TuringCodeEval> =
        setup::generate_results_matrix(111, 555, '1', '5', true);
    let couplings: Vec<Vec<usize>> = setup::generate_coupled_criteria(&matrix);

    let vct_4: Vec<usize> =
        setup::generate_centralizing_test_list(&matrix, &(4 as u8));
    let puzzle_4_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let vct_5: Vec<usize> =
        setup::generate_centralizing_test_list(&matrix, &(5 as u8));
    let puzzle_5_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let vct_6: Vec<usize> =
        setup::generate_centralizing_test_list(&matrix, &(6 as u8));
    let puzzle_6_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    for a in 0..=179 {
        if vct_4.contains(&a) { continue; }

        for b in a + 1..=180 {
            if vct_4.contains(&b)
                || matrix[0].checks[a].0 == matrix[0].checks[b].0
                || couplings[a].contains(&b)
            { continue; }

            for c in b + 1..=181 {
                if vct_4.contains(&c)
                    || matrix[0].checks[a].0 == matrix[0].checks[c].0
                    || matrix[0].checks[b].0 == matrix[0].checks[c].0
                    || couplings[a].contains(&c)
                    || couplings[b].contains(&c)
                { continue; }

                for d in c + 1..=182 {
                    if vct_4.contains(&d)
                        || matrix[0].checks[a].0 == matrix[0].checks[d].0
                        || matrix[0].checks[b].0 == matrix[0].checks[d].0
                        || matrix[0].checks[c].0 == matrix[0].checks[d].0
                        || couplings[a].contains(&d)
                        || couplings[b].contains(&d)
                        || couplings[c].contains(&d)
                    { continue; }

                    for e in d + 1..=182 {
                        if vct_5.contains(&e)
                            || matrix[0].checks[a].0 == matrix[0].checks[e].0
                            || matrix[0].checks[b].0 == matrix[0].checks[e].0
                            || matrix[0].checks[c].0 == matrix[0].checks[e].0
                            || matrix[0].checks[d].0 == matrix[0].checks[e].0
                            || couplings[a].contains(&e)
                            || couplings[b].contains(&e)
                            || couplings[c].contains(&e)
                            || couplings[d].contains(&e)
                        { continue; }

                        for f in e + 1..=182 {
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
                            { continue; }
                            
                        }
                    }
                }
            }
        }
    }

    //puzzle_4_count.fetch_add(1, Ordering::Relaxed);
    //puzzle_5_count.fetch_add(1, Ordering::Relaxed);
    //puzzle_6_count.fetch_add(1, Ordering::Relaxed);
    
    println!(
        "Total # of Puzzles with 4 Criteria Cards: {:>9}",
        puzzle_4_count.load(Ordering::Relaxed)
    );
    println!(
    "Total # of Puzzles with 5 Criteria Cards: {:>9}",
    puzzle_5_count.load(Ordering::Relaxed)
    );
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
