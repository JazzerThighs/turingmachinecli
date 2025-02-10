use std::collections::HashMap;
use crate::game_logic::setup::TuringCodeEval;

#[rustfmt::skip]
pub fn populate_machine_feedback(
    code: u32, 
    machine: &mut Vec<HashMap<String, Vec<bool>>>
) -> TuringCodeEval {
    while machine.len() < 48 {
        machine.push(HashMap::default())
    }
    let mut num = code;
    let p = &num % 10;
    num /= 10;
    let y = &num % 10;
    num /= 10;
    let b = num;
    let sum = b + y + p;
    let num_of_1s = [b, y, p]
        .iter()
        .filter(|x| **x == 1)
        .count();
    let num_of_3s = [b, y, p]
        .iter()
        .filter(|x| **x == 3)
        .count();
    let num_of_4s = [b, y, p]
        .iter()
        .filter(|x| **x == 4)
        .count();
    let even_count = [b, y, p]
        .iter()
        .filter(|x| **x % 2 == 0)
        .count();
    let repetitions_count = [(b, y), (b, p), (y, p)]
        .iter()
        .filter(|(x1, x2)| x1 == x2)
        .count();
    let ascending_order_count = [(b, y), (y, p)]
        .iter()
        .filter(|(x1, x2)| x1 + 1 == *x2)
        .count();
    let descending_order_count = [(b, y), (y, p)]
        .iter()
        .filter(|(x1, x2)| x1 - 1 == *x2)
        .count();

    let mut t = TuringCodeEval {
        code,
        checks: vec![]
    };

    macro_rules! push_eval {
        ($idx:expr, $($cond:expr),+ $(,)?) => {
            machine[$idx - 1].insert(
                code.to_string(),
                vec![$($cond),+],
            );
            for i in &machine[$idx - 1][&code.to_string()] {
                t.checks.push(($idx as u8, *i));
            }
        };
    }

    push_eval!(
        1,
        b == 1,
        b > 1,
    );
    push_eval!(
        2,
        b < 3,
        b == 3,
        b > 3,
    );
    push_eval!(
        3,
        y < 3,
        y == 3,
        y > 3,
    );
    push_eval!(
        4,
        y < 4,
        y == 4,
        y > 4,
    );
    push_eval!(
        5,
        b % 2 == 0,
        b % 2 != 0
    );
    push_eval!(
        6,
        y % 2 == 0,
        y % 2 != 0
    );
    push_eval!(
        7,
        p % 2 == 0,
        p % 2 != 0
    );
    push_eval!(
        8,
        num_of_1s == 0,
        num_of_1s == 1,
        num_of_1s == 2,
        num_of_1s == 3,
    );
    push_eval!(
        9,
        num_of_3s == 0,
        num_of_3s == 1,
        num_of_3s == 2,
        num_of_3s == 3,
    );
    push_eval!(
        10,
        num_of_4s == 0,
        num_of_4s == 1,
        num_of_4s == 2,
        num_of_4s == 3,
    );
    push_eval!(
        11,
        b < y,
        b == y,
        b > y
    );
    push_eval!(
        12,
        b < p,
        b == p,
        b > p
    );
    push_eval!(
        13,
        y < p,
        y == p,
        y > p
    );
    push_eval!(
        14,
        b < y && b < p,
        y < b && y < p,
        p < b && p < y
    );
    push_eval!(
        15,
        b > y && b > p,
        y > b && y > p,
        p > b && p > y
    );
    push_eval!(
        16,
        even_count >= 2,
        even_count < 2
    );
    push_eval!(
        17,
        even_count == 0,
        even_count == 1,
        even_count == 2,
        even_count == 3,
    );
    push_eval!(
        18,
        sum % 2 == 0,
        sum % 2 != 0,
    );
    push_eval!(
        19,
        b + y < 6,
        b + y == 6,
        b + y > 6
    );
    push_eval!(
        20,
        repetitions_count == 3,
        repetitions_count == 1,
        repetitions_count == 0
    );
    push_eval!(
        21,
        repetitions_count == 0,
        repetitions_count != 1
    );
    push_eval!(
        22,
        b < y && y < p,
        b > y && y > p,
        (b <= y && y >= p) || (b >= y && y <= p),
    );
    push_eval!(
        23,
        sum < 6,
        sum == 6,
        sum > 6
    );
    push_eval!(
        24,
        ascending_order_count == 2,
        ascending_order_count == 1,
        ascending_order_count == 0,
    );
    push_eval!(
        25,
        ascending_order_count == 0 && descending_order_count == 0,
        ascending_order_count == 1 || descending_order_count == 1,
        ascending_order_count == 2 || descending_order_count == 2,
    );
    push_eval!(
        26,
        b < 3,
        y < 3,
        p < 3
    );
    push_eval!(
        27,
        b < 4,
        y < 4,
        p < 4
    );
    push_eval!(
        28,
        b == 1,
        y == 1,
        p == 1
    );
    push_eval!(
        29,
        b == 3,
        y == 3,
        p == 3,
    );
    push_eval!(
        30,
        b == 4,
        y == 4,
        p == 4,
    );
    push_eval!(
        31,
        b > 1,
        y > 1,
        p > 1,
    );
    push_eval!(
        32,
        b > 3,
        y > 3,
        p > 3,
    );
    push_eval!(
        33,
        b % 2 == 0,
        b % 2 != 0,
        y % 2 == 0,
        y % 2 != 0,
        p % 2 == 0,
        p % 2 != 0
    );
    push_eval!(
        34,
        b <= y && b <= p,
        y <= b && y <= p,
        p <= b && p <= y,
    );
    push_eval!(
        35,
        b >= y && b >= p,
        y >= b && y >= p,
        p >= b && p >= y,
    );
    push_eval!(
        36,
        sum % 3 == 0,
        sum % 4 == 0,
        sum % 5 == 0,
    );
    push_eval!(
        37,
        b + y == 4,
        b + p == 4,
        p + y == 4
    );
    push_eval!(
        38,
        b + y == 6,
        b + p == 6,
        p + y == 6
    );
    push_eval!(
        39,
        b == 1,
        b > 1,
        y == 1,
        y > 1,
        p == 1,
        p > 1,
    );
    push_eval!(
        40,
        b < 3,
        b == 3,
        b > 3,
        y < 3,
        y == 3,
        y > 3,
        p < 3,
        p == 3,
        p > 3,
    );
    push_eval!(
        41,
        b < 4,
        b == 4,
        b > 4,
        y < 4,
        y == 4,
        y > 4,
        p < 4,
        p == 4,
        p > 4,
    );
    push_eval!(
        42,
        b < y && b < p,
        b > y && b > p,
        y < b && y < p,
        y > b && y > p,
        p < b && p < y,
        p > b && p > y
    );
    push_eval!(
        43,
        b < y,
        b < p,
        b == y,
        b == p,
        b > y,
        b > p
    );
    push_eval!(
        44,
        y < b,
        y < p,
        y == b,
        y == p,
        y > b,
        y > p,
    );
    push_eval!(
        45,
        num_of_1s == 0,
        num_of_3s == 0,
        num_of_1s == 1,
        num_of_3s == 1,
        num_of_1s == 2,
        num_of_3s == 2,
    );
    push_eval!(
        46,
        num_of_3s == 0,
        num_of_4s == 0,
        num_of_3s == 1,
        num_of_4s == 1,
        num_of_3s == 2,
        num_of_4s == 2,
    );
    push_eval!(
        47,
        num_of_1s == 0,
        num_of_4s == 0,
        num_of_1s == 1,
        num_of_4s == 1,
        num_of_1s == 2,
        num_of_4s == 2,
    );
    push_eval!(
        48,
        b < y,
        b == y,
        b > y,
        b < p,
        b == p,
        b > y,
        y < p,
        y == p,
        y > p
    );

    t
}