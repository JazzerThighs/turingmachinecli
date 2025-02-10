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

    let mut turing_code_eval = TuringCodeEval {
        code,
        checks: vec![]
    };

    macro_rules! push_eval {
        ($idx:expr, $($cond:expr),+) => {
            machine[$idx - 1].insert(
                code.to_string(),
                vec![$($cond),+],
            );
            for i in &machine[$idx - 1][&code.to_string()] {
                turing_code_eval.checks.push(($idx as u8, *i));
            }
        };
    }

    push_eval!(
        1,
        b == 1, // matrix[_].checks[*0*].1
        b > 1 // 1
    );
    push_eval!(
        2,
        b < 3, // 2
        b == 3, // 3
        b > 3 // 4
    );
    push_eval!(
        3,
        y < 3, // 5
        y == 3, // 6
        y > 3 // 7
    );
    push_eval!(
        4,
        y < 4, // 8
        y == 4, // 9
        y > 4 // 10
    );
    push_eval!(
        5,
        b % 2 == 0, // 11
        b % 2 != 0 // 12
    );
    push_eval!(
        6,
        y % 2 == 0, // 13
        y % 2 != 0 // 14
    );
    push_eval!(
        7,
        p % 2 == 0, // 15
        p % 2 != 0 // 16
    );
    push_eval!(
        8,
        num_of_1s == 0, // 17
        num_of_1s == 1, // 18
        num_of_1s == 2, // 19
        num_of_1s == 3 // 20
    );
    push_eval!(
        9,
        num_of_3s == 0, // 21
        num_of_3s == 1, // 22
        num_of_3s == 2, // 23
        num_of_3s == 3 // 24
    );
    push_eval!(
        10,
        num_of_4s == 0, // 25
        num_of_4s == 1, // 26
        num_of_4s == 2, // 27
        num_of_4s == 3 // 28
    );
    push_eval!(
        11,
        b < y, // 29
        b == y, // 30
        b > y // 31
    );
    push_eval!(
        12,
        b < p, // 32
        b == p, // 33
        b > p // 34
    );
    push_eval!(
        13,
        y < p, // 35
        y == p, // 36
        y > p // 37
    );
    push_eval!(
        14,
        b < y && b < p, // 38
        y < b && y < p, // 39
        p < b && p < y // 40
    );
    push_eval!(
        15,
        b > y && b > p, // 41
        y > b && y > p, // 42
        p > b && p > y // 43
    );
    push_eval!(
        16,
        even_count >= 2, // 44
        even_count < 2 // 45
    );
    push_eval!(
        17,
        even_count == 0, // 46
        even_count == 1, // 47
        even_count == 2, // 48
        even_count == 3 // 49
    );
    push_eval!(
        18,
        sum % 2 == 0, // 50
        sum % 2 != 0 // 51
    );
    push_eval!(
        19,
        b + y < 6, // 52
        b + y == 6, // 53
        b + y > 6 // 54
    );
    push_eval!(
        20,
        repetitions_count == 3, // 55
        repetitions_count == 1, // 56
        repetitions_count == 0 // 57
    );
    push_eval!(
        21,
        repetitions_count == 0, // 58
        repetitions_count != 1 // 59
    );
    push_eval!(
        22,
        b < y && y < p, // 60
        b > y && y > p, // 61
        !(b < y && y < p) && !(b > y && y > p) // 62
    );
    push_eval!(
        23,
        sum < 6, // 63
        sum == 6, // 64
        sum > 6 // 65
    );
    push_eval!(
        24,
        ascending_order_count == 2, // 66
        ascending_order_count == 1, // 67
        ascending_order_count == 0 // 68
    );
    push_eval!(
        25,
        ascending_order_count == 0 && descending_order_count == 0, // 69
        ascending_order_count == 1 || descending_order_count == 1, // 70
        ascending_order_count == 2 || descending_order_count == 2 // 71
    );
    push_eval!(
        26,
        b < 3, // 72
        y < 3, // 73
        p < 3 // 74
    );
    push_eval!(
        27,
        b < 4, // 75
        y < 4, // 76
        p < 4 // 77
    );
    push_eval!(
        28,
        b == 1, // 78
        y == 1, // 79
        p == 1 // 80
    );
    push_eval!(
        29,
        b == 3, // 81
        y == 3, // 82
        p == 3 // 83
    );
    push_eval!(
        30,
        b == 4, // 84
        y == 4, // 85
        p == 4 // 86
    );
    push_eval!(
        31,
        b > 1, // 87
        y > 1, // 88
        p > 1 // 89
    );
    push_eval!(
        32,
        b > 3, // 90
        y > 3, // 91
        p > 3 // 92
    );
    push_eval!(
        33,
        b % 2 == 0, // 93
        b % 2 != 0, // 94
        y % 2 == 0, // 95
        y % 2 != 0, // 96
        p % 2 == 0, // 97
        p % 2 != 0 // 98
    );
    push_eval!(
        34,
        b <= y && b <= p, // 99
        y <= b && y <= p, // 100
        p <= b && p <= y // 101
    );
    push_eval!(
        35,
        b >= y && b >= p, // 102
        y >= b && y >= p, // 103
        p >= b && p >= y // 104
    );
    push_eval!(
        36,
        sum % 3 == 0, // 105
        sum % 4 == 0, // 106
        sum % 5 == 0 // 107
    );
    push_eval!(
        37,
        b + y == 4, // 108
        b + p == 4, // 109
        p + y == 4 // 110
    );
    push_eval!(
        38,
        b + y == 6, // 111
        b + p == 6, // 112
        p + y == 6 // 113
    );
    push_eval!(
        39,
        b == 1, // 114
        b > 1, // 115
        y == 1, // 116
        y > 1, // 117
        p == 1, // 118
        p > 1 // 119
    );
    push_eval!(
        40,
        b < 3, // 120
        b == 3, // 121
        b > 3, // 122
        y < 3, // 123
        y == 3, // 124
        y > 3, // 125
        p < 3, // 126
        p == 3, // 127
        p > 3 // 128
    );
    push_eval!(
        41,
        b < 4, // 129
        b == 4, // 130
        b > 4, // 131
        y < 4, // 132
        y == 4, // 133
        y > 4, // 134
        p < 4, // 135
        p == 4, // 136
        p > 4 // 137
    );
    push_eval!(
        42,
        b < y && b < p, // 138
        b > y && b > p, // 139
        y < b && y < p, // 140
        y > b && y > p, // 141
        p < b && p < y, // 142
        p > b && p > y // 143
    );
    push_eval!(
        43,
        b < y, // 144
        b < p, // 145
        b == y, // 146
        b == p, // 147
        b > y, // 148
        b > p // 149
    );
    push_eval!(
        44,
        y < b, // 150
        y < p, // 151
        y == b, // 152
        y == p, // 153
        y > b, // 154
        y > p // 155
    );
    push_eval!(
        45,
        num_of_1s == 0, // 156
        num_of_3s == 0, // 157
        num_of_1s == 1, // 158
        num_of_3s == 1, // 159
        num_of_1s == 2, // 160
        num_of_3s == 2 // 161
    );
    push_eval!(
        46,
        num_of_3s == 0, // 162
        num_of_4s == 0, // 163
        num_of_3s == 1, // 164
        num_of_4s == 1, // 165
        num_of_3s == 2, // 166
        num_of_4s == 2 // 167
    );
    push_eval!(
        47,
        num_of_1s == 0, // 168
        num_of_4s == 0, // 169
        num_of_1s == 1, // 170
        num_of_4s == 1, // 171
        num_of_1s == 2, // 172
        num_of_4s == 2 // 173
    );
    push_eval!(
        48,
        b < y, // 174
        b == y, // 175
        b > y, // 176
        b < p, // 177
        b == p, // 178
        b > y, // 179
        y < p, // 180
        y == p, // 181
        y > p // 182
    );

    turing_code_eval
}