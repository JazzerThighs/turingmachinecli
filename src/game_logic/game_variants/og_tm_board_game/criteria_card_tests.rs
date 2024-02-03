use crate::game_logic::setup::TuringCodeEval;

pub fn evaluate_criteria_results(turing_code: u32) -> TuringCodeEval {
    let mut num = turing_code.clone();
    let purple = &num % 10;
    num /= 10;
    let yellow = &num % 10;
    num /= 10;
    let blue = num;

    // Criteria Card 1,
    // Criteria Card 2,
    // Criteria Card 3,
    // Criteria Card 4,
    // Criteria Card 26,
    // Criteria Card 27,
    // Criteria Card 28,
    // Criteria Card 29,
    // Criteria Card 30,
    // Criteria Card 31,
    // Criteria Card 32,
    // Criteria Card 39,
    // Criteria Card 40,
    // Criteria Card 41
    let mut test1_1 = false;
    let mut test1_2 = false;

    let mut test2_1 = false;
    let mut test2_2 = false;
    let mut test2_3 = false;

    let mut test3_1 = false;
    let mut test3_2 = false;
    let mut test3_3 = false;

    let mut test4_1 = false;
    let mut test4_2 = false;
    let mut test4_3 = false;

    let mut test26_1 = false;
    let mut test26_2 = false;
    let mut test26_3 = false;

    let mut test27_1 = false;
    let mut test27_2 = false;
    let mut test27_3 = false;

    let mut test28_1 = false;
    let mut test28_2 = false;
    let mut test28_3 = false;

    let mut test29_1 = false;
    let mut test29_2 = false;
    let mut test29_3 = false;

    let mut test30_1 = false;
    let mut test30_2 = false;
    let mut test30_3 = false;

    let mut test31_1 = false;
    let mut test31_2 = false;
    let mut test31_3 = false;

    let mut test32_1 = false;
    let mut test32_2 = false;
    let mut test32_3 = false;

    let mut test39_1 = false;
    let mut test39_2 = false;
    let mut test39_3 = false;
    let mut test39_4 = false;
    let mut test39_5 = false;
    let mut test39_6 = false;

    let mut test40_1 = false;
    let mut test40_2 = false;
    let mut test40_3 = false;
    let mut test40_4 = false;
    let mut test40_5 = false;
    let mut test40_6 = false;
    let mut test40_7 = false;
    let mut test40_8 = false;
    let mut test40_9 = false;

    let mut test41_1 = false;
    let mut test41_2 = false;
    let mut test41_3 = false;
    let mut test41_4 = false;
    let mut test41_5 = false;
    let mut test41_6 = false;
    let mut test41_7 = false;
    let mut test41_8 = false;
    let mut test41_9 = false;

    if blue == 1 {
        test1_1 = true;
        test28_1 = true;
        test39_1 = true;
    } else {
        test1_2 = true;
        test31_1 = true;
        test39_2 = true;
    }

    if blue < 3 {
        test2_1 = true;
        test26_1 = true;
        test40_1 = true;
    } else if blue == 3 {
        test2_2 = true;
        test29_1 = true;
        test40_2 = true;
    } else {
        test2_3 = true;
        test32_1 = true;
        test40_3 = true;
    }

    if blue < 4 {
        test27_1 = true;
        test41_1 = true;
    } else if blue == 4 {
        test30_1 = true;
        test41_2 = true;
    } else {
        test41_3 = true;
    }

    if yellow == 1 {
        test28_2 = true;
        test39_3 = true;
    } else {
        test31_2 = true;
        test39_4 = true;
    }

    if yellow < 3 {
        test3_1 = true;
        test26_2 = true;
        test40_4 = true;
    } else if yellow == 3 {
        test3_2 = true;
        test29_2 = true;
        test40_5 = true;
    } else {
        test3_3 = true;
        test32_2 = true;
        test40_6 = true;
    }

    if yellow < 4 {
        test4_1 = true;
        test27_2 = true;
        test41_4 = true;
    } else if yellow == 4 {
        test4_2 = true;
        test30_2 = true;
        test41_5 = true;
    } else {
        test4_3 = true;
        test41_6 = true;
    }

    if purple == 1 {
        test28_3 = true;
        test39_5 = true;
    } else {
        test31_3 = true;
        test39_6 = true;
    }

    if purple < 3 {
        test26_3 = true;
        test40_7 = true;
    } else if purple == 3 {
        test29_3 = true;
        test40_8 = true;
    } else {
        test32_3 = true;
        test40_9 = true;
    }

    if purple < 4 {
        test27_3 = true;
        test41_7 = true;
    } else if purple == 4 {
        test30_3 = true;
        test41_8 = true;
    } else {
        test41_9 = true;
    }

    // Criteria Card 5,
    // Criteria Card 6,
    // Criteria Card 7,
    // Criteria Card 33
    let mut test5_1 = false;
    let mut test5_2 = false;

    let mut test6_1 = false;
    let mut test6_2 = false;

    let mut test7_1 = false;
    let mut test7_2 = false;

    let mut test33_1 = false;
    let mut test33_2 = false;
    let mut test33_3 = false;
    let mut test33_4 = false;
    let mut test33_5 = false;
    let mut test33_6 = false;

    if blue % 2 == 0 {
        test5_1 = true;
        test33_1 = true;
    } else {
        test5_2 = true;
        test33_2 = true;
    }

    if yellow % 2 == 0 {
        test6_1 = true;
        test33_3 = true;
    } else {
        test6_2 = true;
        test33_4 = true;
    }

    if purple % 2 == 0 {
        test7_1 = true;
        test33_5 = true;
    } else {
        test7_2 = true;
        test33_6 = true;
    }

    // Criteria Card 8,
    // Criteria Card 9,
    // Criteria Card 10,
    // Criteria Card 45,
    // Criteria Card 46,
    // Criteria Card 47
    let mut test8_1 = false;
    let mut test8_2 = false;
    let mut test8_3 = false;
    let mut test8_4 = false;

    let mut test9_1 = false;
    let mut test9_2 = false;
    let mut test9_3 = false;
    let mut test9_4 = false;

    let mut test10_1 = false;
    let mut test10_2 = false;
    let mut test10_3 = false;
    let mut test10_4 = false;

    let mut test45_1 = false;
    let mut test45_2 = false;
    let mut test45_3 = false;
    let mut test45_4 = false;
    let mut test45_5 = false;
    let mut test45_6 = false;

    let mut test46_1 = false;
    let mut test46_2 = false;
    let mut test46_3 = false;
    let mut test46_4 = false;
    let mut test46_5 = false;
    let mut test46_6 = false;

    let mut test47_1 = false;
    let mut test47_2 = false;
    let mut test47_3 = false;
    let mut test47_4 = false;
    let mut test47_5 = false;
    let mut test47_6 = false;

    let mut num_of_1s = 0;
    let mut num_of_3s = 0;
    let mut num_of_4s = 0;
    if blue == 1 {
        num_of_1s += 1;
    } else if blue == 3 {
        num_of_3s += 1;
    } else if blue == 4 {
        num_of_4s += 1;
    }
    if yellow == 1 {
        num_of_1s += 1;
    } else if yellow == 3 {
        num_of_3s += 1;
    } else if yellow == 4 {
        num_of_4s += 1;
    }
    if purple == 1 {
        num_of_1s += 1;
    } else if purple == 3 {
        num_of_3s += 1;
    } else if purple == 4 {
        num_of_4s += 1;
    }

    match num_of_1s {
        0 => {
            test8_1 = true;
            test45_1 = true;
            test47_1 = true;
        }
        1 => {
            test8_2 = true;
            test45_3 = true;
            test47_3 = true;
        }
        2 => {
            test8_3 = true;
            test45_5 = true;
            test47_5 = true;
        }
        3 => test8_4 = true,
        _ => {}
    }

    match num_of_3s {
        0 => {
            test9_1 = true;
            test45_2 = true;
            test46_1 = true;
        }
        1 => {
            test9_2 = true;
            test45_4 = true;
            test46_3 = true;
        }
        2 => {
            test9_3 = true;
            test45_6 = true;
            test46_5 = true;
        }
        3 => test9_4 = true,
        _ => {}
    }

    match num_of_4s {
        0 => {
            test10_1 = true;
            test46_2 = true;
            test47_2 = true;
        }
        1 => {
            test10_2 = true;
            test46_4 = true;
            test47_4 = true;
        }
        2 => {
            test10_3 = true;
            test46_6 = true;
            test47_6 = true;
        }
        3 => test10_4 = true,
        _ => {}
    }

    // Criteria Card 11,
    // Criteria Card 12,
    // Criteria Card 13,
    // Criteria Card 43,
    // Criteria Card 44,
    // Criteria Card 48
    let mut test11_1 = false;
    let mut test11_2 = false;
    let mut test11_3 = false;

    let mut test12_1 = false;
    let mut test12_2 = false;
    let mut test12_3 = false;

    let mut test13_1 = false;
    let mut test13_2 = false;
    let mut test13_3 = false;

    let mut test43_1 = false;
    let mut test43_2 = false;
    let mut test43_3 = false;
    let mut test43_4 = false;
    let mut test43_5 = false;
    let mut test43_6 = false;

    let mut test44_1 = false;
    let mut test44_2 = false;
    let mut test44_3 = false;
    let mut test44_4 = false;
    let mut test44_5 = false;
    let mut test44_6 = false;

    let mut test48_1 = false;
    let mut test48_2 = false;
    let mut test48_3 = false;
    let mut test48_4 = false;
    let mut test48_5 = false;
    let mut test48_6 = false;
    let mut test48_7 = false;
    let mut test48_8 = false;
    let mut test48_9 = false;

    if blue < yellow {
        test11_1 = true;
        test43_1 = true;
        test44_5 = true;
        test48_1 = true;
    } else if blue == yellow {
        test11_2 = true;
        test43_3 = true;
        test44_3 = true;
        test48_2 = true;
    } else {
        test11_3 = true;
        test43_5 = true;
        test44_1 = true;
        test48_3 = true;
    }

    if blue < purple {
        test12_1 = true;
        test43_2 = true;
        test48_4 = true;
    } else if blue == purple {
        test12_2 = true;
        test43_4 = true;
        test48_5 = true;
    } else {
        test12_3 = true;
        test43_6 = true;
        test48_6 = true;
    }

    if yellow < purple {
        test13_1 = true;
        test44_2 = true;
        test48_7 = true;
    } else if yellow == purple {
        test13_2 = true;
        test44_4 = true;
        test48_8 = true;
    } else {
        test13_3 = true;
        test44_6 = true;
        test48_9 = true;
    }

    // Criteria Card 14
    let mut test14_1 = false;
    let mut test14_2 = false;
    let mut test14_3 = false;

    if (blue < yellow) && (blue < purple) {
        test14_1 = true;
    }
    if (yellow < blue) && (yellow < purple) {
        test14_2 = true;
    }
    if (purple < blue) && (purple < yellow) {
        test14_3 = true;
    }

    // Criteria Card 15
    let mut test15_1 = false;
    let mut test15_2 = false;
    let mut test15_3 = false;

    if (blue > yellow) && (blue > purple) {
        test15_1 = true;
    }
    if (yellow > blue) && (yellow > purple) {
        test15_2 = true;
    }
    if (purple > blue) && (purple > yellow) {
        test15_3 = true;
    }

    // Criteria Card 16,
    // Criteria Card 17
    let mut test16_1 = false;
    let mut test16_2 = false;

    let mut test17_1 = false;
    let mut test17_2 = false;
    let mut test17_3 = false;
    let mut test17_4 = false;

    let mut even_count = 0;
    if blue % 2 == 0 {
        even_count += 1;
    }
    if yellow % 2 == 0 {
        even_count += 1;
    }
    if purple % 2 == 0 {
        even_count += 1;
    }

    match even_count {
        0 => {
            test17_1 = true;
            test16_2 = true;
        }
        1 => {
            test17_2 = true;
            test16_2 = true;
        }
        2 => {
            test17_3 = true;
            test16_1 = true;
        }
        3 => {
            test17_4 = true;
            test16_1 = true;
        }
        _ => {}
    }

    // Criteria Card 18,
    // Criteria Card 23,
    // Criteria Card 36
    let mut test18_1 = false;
    let mut test18_2 = false;

    let mut test23_1 = false;
    let mut test23_2 = false;
    let mut test23_3 = false;

    let mut test36_1 = false;
    let mut test36_2 = false;
    let mut test36_3 = false;

    let all_color_sum = &blue + &yellow + &purple;

    if all_color_sum % 2 == 0 {
        test18_1 = true;
    } else {
        test18_2 = true;
    }

    if all_color_sum % 3 == 0 {
        test36_1 = true;
    }
    if all_color_sum % 4 == 0 {
        test36_2 = true;
    }
    if all_color_sum % 5 == 0 {
        test36_3 = true;
    }

    if all_color_sum < 6 {
        test23_1 = true;
    } else if all_color_sum == 6 {
        test23_2 = true;
    } else {
        test23_3 = true;
    }

    // Criteria Card 19,
    // Criteria Card 37,
    // Criteria Card 38
    let mut test19_1 = false;
    let mut test19_2 = false;
    let mut test19_3 = false;

    let mut test37_1 = false;
    let mut test37_2 = false;
    let mut test37_3 = false;

    let mut test38_1 = false;
    let mut test38_2 = false;
    let mut test38_3 = false;

    if (blue + yellow) == 4 {
        test37_1 = true;
    }
    if (blue + yellow) < 6 {
        test19_1 = true;
    } else if (blue + yellow) == 6 {
        test19_2 = true;
        test38_1 = true;
    } else {
        test19_3 = true;
    }

    if (blue + purple) == 4 {
        test37_2 = true;
    } else if (blue + purple) == 6 {
        test38_2 = true;
    }

    if (yellow + purple) == 4 {
        test37_3 = true;
    } else if (yellow + purple) == 6 {
        test38_3 = true;
    }

    // Criteria Card 20,
    // Criteria Card 21
    let mut test20_1 = false;
    let mut test20_2 = false;
    let mut test20_3 = false;

    let mut test21_1 = false;
    let mut test21_2 = false;

    let mut num_repetitions = 0;
    if blue == yellow {
        num_repetitions += 1;
    }
    if blue == purple {
        num_repetitions += 1;
    }
    if yellow == purple{
        num_repetitions += 1;
    }

    match num_repetitions {
        1 => {
            test20_2 = true;
            test21_2 = true;
        },
        0 => {
            test20_3 = true;
            test21_1 = true;
        },
        _ => {
            test20_1 = true;
            test21_1 = true;
        }
    }

    // Criteria Card 22
    let mut test22_1 = false;
    let mut test22_2 = false;
    let mut test22_3 = false;

    if (blue < yellow) && (yellow < purple) {
        test22_1 = true;
    } else if (blue > yellow) && (yellow > purple) {
        test22_2 = true;
    } else {
        test22_3 = true;
    }

    // Criteria Card 24
    let mut test24_1 = false;
    let mut test24_2 = false;
    let mut test24_3 = false;
    let mut test24_count = 0;

    if (blue + 1) == yellow {
        test24_count += 1;
    }
    if (yellow + 1) == purple {
        test24_count += 1;
    }
    match test24_count {
        2 => test24_1 = true,
        1 => test24_2 = true,
        0 => test24_3 = true,
        _ => {}
    }

    // Criteria Card 25
    let mut test25_1 = false;
    let mut test25_2 = false;
    let mut test25_3 = false;
    let mut test25_ascend_count: u8 = 0;
    let mut test25_descend_count: u8 = 0;

    if (blue + 1) == yellow {
        test25_ascend_count += 1;
    }
    if (blue - 1) == yellow {
        test25_descend_count += 1;
    }
    if (yellow + 1) == purple {
        test25_ascend_count += 1;
    }
    if (yellow - 1) == purple {
        test25_descend_count += 1;
    }
    match (test25_ascend_count, test25_descend_count) {
        (2, _) | (_, 2) => test25_3 = true,
        (1, _) | (_, 1) => test25_2 = true,
        (0, 0) => test25_1 = true,
        _ => {},
    }

    // Criteria Card 34
    let mut test34_1 = false;
    let mut test34_2 = false;
    let mut test34_3 = false;

    if (blue <= yellow) && (blue <= purple) {
        test34_1 = true;
    }
    if (yellow <= blue) && (yellow <= purple) {
        test34_2 = true;
    }
    if (purple <= blue) && (purple <= yellow) {
        test34_3 = true;
    }

    // Criteria Card 35
    let mut test35_1 = false;
    let mut test35_2 = false;
    let mut test35_3 = false;

    if (blue >= yellow) && (blue >= purple) {
        test35_1 = true;
    }
    if (yellow >= blue) && (yellow >= purple) {
        test35_2 = true;
    }
    if (purple >= blue) && (purple >= yellow) {
        test35_3 = true;
    }

    // Criteria Card 42
    let mut test42_1 = false;
    let mut test42_2 = false;
    let mut test42_3 = false;
    let mut test42_4 = false;
    let mut test42_5 = false;
    let mut test42_6 = false;

    if (blue < yellow) && (blue < purple) {
        test42_1 = true;
    } else if (blue > yellow) && (blue > purple) {
        test42_2 = true;
    }

    if (yellow < blue) && (yellow < purple) {
        test42_3 = true;
    } else if (yellow > blue) && (yellow > purple) {
        test42_4 = true;
    }

    if (purple < blue) && (purple < yellow) {
        test42_5 = true;
    } else if (purple > blue) && (purple > yellow) {
        test42_6 = true;
    }

    // Output Logic
    let criteria_checks: Vec<(u8, bool)> = Vec::from([
        (1, test1_1),   //0
        (1, test1_2),   //1
        (2, test2_1),   //2
        (2, test2_2),   //3
        (2, test2_3),   //4
        (3, test3_1),   //5
        (3, test3_2),   //6
        (3, test3_3),   //7
        (4, test4_1),   //8
        (4, test4_2),   //9
        (4, test4_3),   //10
        (5, test5_1),   //11
        (5, test5_2),   //12
        (6, test6_1),   //13
        (6, test6_2),   //14
        (7, test7_1),   //15
        (7, test7_2),   //16
        (8, test8_1),   //17
        (8, test8_2),   //18
        (8, test8_3),   //19
        (8, test8_4),   //20
        (9, test9_1),   //21
        (9, test9_2),   //22
        (9, test9_3),   //23
        (9, test9_4),   //24
        (10, test10_1), //25
        (10, test10_2), //26
        (10, test10_3), //27
        (10, test10_4), //28
        (11, test11_1), //29
        (11, test11_2), //30
        (11, test11_3), //31
        (12, test12_1), //32
        (12, test12_2), //33
        (12, test12_3), //34
        (13, test13_1), //35
        (13, test13_2), //36
        (13, test13_3), //37
        (14, test14_1), //38
        (14, test14_2), //39
        (14, test14_3), //40
        (15, test15_1), //41
        (15, test15_2), //42
        (15, test15_3), //43
        (16, test16_1), //44
        (16, test16_2), //45
        (17, test17_1), //46
        (17, test17_2), //47
        (17, test17_3), //48
        (17, test17_4), //49
        (18, test18_1), //50
        (18, test18_2), //51
        (19, test19_1), //52
        (19, test19_2), //53
        (19, test19_3), //54
        (20, test20_1), //55
        (20, test20_2), //56
        (20, test20_3), //57
        (21, test21_1), //58
        (21, test21_2), //59
        (22, test22_1), //60
        (22, test22_2), //61
        (22, test22_3), //62
        (23, test23_1), //63
        (23, test23_2), //64
        (23, test23_3), //65
        (24, test24_1), //66
        (24, test24_2), //67
        (24, test24_3), //68
        (25, test25_1), //69
        (25, test25_2), //70
        (25, test25_3), //71
        (26, test26_1), //72
        (26, test26_2), //73
        (26, test26_3), //74
        (27, test27_1), //75
        (27, test27_2), //76
        (27, test27_3), //77
        (28, test28_1), //78
        (28, test28_2), //79
        (28, test28_3), //80
        (29, test29_1), //81
        (29, test29_2), //82
        (29, test29_3), //83
        (30, test30_1), //84
        (30, test30_2), //85
        (30, test30_3), //86
        (31, test31_1), //87
        (31, test31_2), //88
        (31, test31_3), //89
        (32, test32_1), //90
        (32, test32_2), //91
        (32, test32_3), //92
        (33, test33_1), //93
        (33, test33_2), //94
        (33, test33_3), //95
        (33, test33_4), //96
        (33, test33_5), //97
        (33, test33_6), //98
        (34, test34_1), //99
        (34, test34_2), //100
        (34, test34_3), //101
        (35, test35_1), //102
        (35, test35_2), //103
        (35, test35_3), //104
        (36, test36_1), //105
        (36, test36_2), //106
        (36, test36_3), //107
        (37, test37_1), //108
        (37, test37_2), //109
        (37, test37_3), //110
        (38, test38_1), //111
        (38, test38_2), //112
        (38, test38_3), //113
        (39, test39_1), //114
        (39, test39_2), //115
        (39, test39_3), //116
        (39, test39_4), //117
        (39, test39_5), //118
        (39, test39_6), //119
        (40, test40_1), //120
        (40, test40_2), //121
        (40, test40_3), //122
        (40, test40_4), //123
        (40, test40_5), //124
        (40, test40_6), //125
        (40, test40_7), //126
        (40, test40_8), //127
        (40, test40_9), //128
        (41, test41_1), //129
        (41, test41_2), //130
        (41, test41_3), //131
        (41, test41_4), //132
        (41, test41_5), //133
        (41, test41_6), //134
        (41, test41_7), //135
        (41, test41_8), //136
        (41, test41_9), //137
        (42, test42_1), //138
        (42, test42_2), //139
        (42, test42_3), //140
        (42, test42_4), //141
        (42, test42_5), //142
        (42, test42_6), //143
        (43, test43_1), //144
        (43, test43_2), //145
        (43, test43_3), //146
        (43, test43_4), //147
        (43, test43_5), //148
        (43, test43_6), //149
        (44, test44_1), //150
        (44, test44_2), //151
        (44, test44_3), //152
        (44, test44_4), //153
        (44, test44_5), //154
        (44, test44_6), //155
        (45, test45_1), //156
        (45, test45_2), //157
        (45, test45_3), //158
        (45, test45_4), //159
        (45, test45_5), //160
        (45, test45_6), //161
        (46, test46_1), //162
        (46, test46_2), //163
        (46, test46_3), //164
        (46, test46_4), //165
        (46, test46_5), //166
        (46, test46_6), //167
        (47, test47_1), //168
        (47, test47_2), //169
        (47, test47_3), //170
        (47, test47_4), //171
        (47, test47_5), //172
        (47, test47_6), //173
        (48, test48_1), //174
        (48, test48_2), //175
        (48, test48_3), //176
        (48, test48_4), //177
        (48, test48_5), //178
        (48, test48_6), //179
        (48, test48_7), //180
        (48, test48_8), //181
        (48, test48_9), //182
    ]);

    let code_struct: TuringCodeEval = TuringCodeEval {
        code: turing_code.clone(),
        checks: criteria_checks.clone(),
    };

    return code_struct;
}
