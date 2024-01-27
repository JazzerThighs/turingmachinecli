pub fn is_valid_turing_code(code: u32) -> bool {
    (111..=555).contains(&code) && code.to_string().chars().all(|c| c >= '1' && c <= '5')
}

pub struct TuringCodeResults {
    code: u32,
    checks: Vec<(u8, bool)>,
}

pub fn generate_number_pool(length: usize, min: u8, max: u8) -> Vec<u32> {
    print!("Generating Pool of Turing Codes...  ");

    let mut result = Vec::new();
    let mut strs = vec![min; length];

    loop {
        let number: u32 = strs
            .iter()
            .map(|&n| n.to_string())
            .collect::<String>()
            .parse()
            .unwrap_or(0);

        result.push(number);

        for i in (0..length).rev() {
            if strs[i] < max {
                strs[i] += 1;
                break;
            } else {
                if i == 0 {
                    println!("Done!");
                    return result;
                } else {
                    strs[i] = min;
                }
            }
        }
    }
}

pub fn generate_result_matrix(turing_code: u32) -> TuringCodeResults {
    let mut num = turing_code.clone();
    let purple = &num % 10;
    num /= 10;
    let yellow = &num % 10;
    num /= 10;
    let blue = num;

    // generic calculations
    let all_color_sum = &blue + &yellow + &purple;

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
    // Criteria Card 36
    let mut test18_1 = false;
    let mut test18_2 = false;

    let mut test36_1 = false;
    let mut test36_2 = false;
    let mut test36_3 = false;

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
    if yellow == purple {
        num_repetitions += 1;
    }

    match num_repetitions {
        3 => {
            test20_1 = true;
            test21_1 = true;
        }
        2 => {
            test20_2 = true;
            test21_2 = true;
        }
        1 | 0 => {
            test20_3 = true;
            test21_1 = true;
        }
        _ => {}
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

    // Criteria Card 23
    let mut test23_1 = false;
    let mut test23_2 = false;
    let mut test23_3 = false;

    if all_color_sum < 6 {
        test23_1 = true;
    } else if all_color_sum == 6 {
        test23_2 = true;
    } else {
        test23_3 = true;
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

    if (blue < yellow) || (yellow < purple) {
        // check for ascending sequence
        if (blue + 1) == yellow {
            if (yellow + 1) == purple {
                test25_3 = true;
            } else {
                test25_2 = true;
            }
        } else if (yellow + 1) == purple {
            test25_2 = true;
        } else {
            test25_1 = true;
        }
    } else {
        // check for descending sequence
        if (blue - 1) == yellow {
            if (yellow - 1) == purple {
                test25_3 = true;
            } else {
                test25_2 = true;
            }
        } else if (yellow - 1) == purple {
            test25_2 = true;
        } else {
            test25_1 = true;
        }
    }

    // Criteria Card 34
    let mut test34_1 = false;
    let mut test34_2 = false;
    let mut test34_3 = false;

    if (blue <= yellow) && (blue <= purple) {
        test34_1 = true;
    }
    if yellow <= blue && yellow <= purple {
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
    if yellow >= blue && yellow >= purple {
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
        (1, test1_1),
        (1, test1_2),
        (2, test2_1),
        (2, test2_2),
        (2, test2_3),
        (3, test3_1),
        (3, test3_2),
        (3, test3_3),
        (4, test4_1),
        (4, test4_2),
        (4, test4_3),
        (5, test5_1),
        (5, test5_2),
        (6, test6_1),
        (6, test6_2),
        (7, test7_1),
        (7, test7_2),
        (8, test8_1),
        (8, test8_2),
        (8, test8_3),
        (8, test8_4),
        (9, test9_1),
        (9, test9_2),
        (9, test9_3),
        (9, test9_4),
        (10, test10_1),
        (10, test10_2),
        (10, test10_3),
        (10, test10_4),
        (11, test11_1),
        (11, test11_2),
        (11, test11_3),
        (12, test12_1),
        (12, test12_2),
        (12, test12_3),
        (13, test13_1),
        (13, test13_2),
        (13, test13_3),
        (14, test14_1),
        (14, test14_2),
        (14, test14_3),
        (15, test15_1),
        (15, test15_2),
        (15, test15_3),
        (16, test16_1),
        (16, test16_2),
        (17, test17_1),
        (17, test17_2),
        (17, test17_3),
        (17, test17_4),
        (18, test18_1),
        (18, test18_2),
        (19, test19_1),
        (19, test19_2),
        (19, test19_3),
        (20, test20_1),
        (20, test20_2),
        (20, test20_3),
        (21, test21_1),
        (21, test21_2),
        (22, test22_1),
        (22, test22_2),
        (22, test22_3),
        (23, test23_1),
        (23, test23_2),
        (23, test23_3),
        (24, test24_1),
        (24, test24_2),
        (24, test24_3),
        (25, test25_1),
        (25, test25_2),
        (25, test25_3),
        (26, test26_1),
        (26, test26_2),
        (26, test26_3),
        (27, test27_1),
        (27, test27_2),
        (27, test27_3),
        (28, test28_1),
        (28, test28_2),
        (28, test28_3),
        (29, test29_1),
        (29, test29_2),
        (29, test29_3),
        (30, test30_1),
        (30, test30_2),
        (30, test30_3),
        (31, test31_1),
        (31, test31_2),
        (31, test31_3),
        (32, test32_1),
        (32, test32_2),
        (32, test32_3),
        (33, test33_1),
        (33, test33_2),
        (33, test33_3),
        (33, test33_4),
        (33, test33_5),
        (33, test33_6),
        (34, test34_1),
        (34, test34_2),
        (34, test34_3),
        (35, test35_1),
        (35, test35_2),
        (35, test35_3),
        (36, test36_1),
        (36, test36_2),
        (36, test36_3),
        (37, test37_1),
        (37, test37_2),
        (37, test37_3),
        (38, test38_1),
        (38, test38_2),
        (38, test38_3),
        (39, test39_1),
        (39, test39_2),
        (39, test39_3),
        (39, test39_4),
        (39, test39_5),
        (39, test39_6),
        (40, test40_1),
        (40, test40_2),
        (40, test40_3),
        (40, test40_4),
        (40, test40_5),
        (40, test40_6),
        (40, test40_7),
        (40, test40_8),
        (40, test40_9),
        (41, test41_1),
        (41, test41_2),
        (41, test41_3),
        (41, test41_4),
        (41, test41_5),
        (41, test41_6),
        (41, test41_7),
        (41, test41_8),
        (41, test41_9),
        (42, test42_1),
        (42, test42_2),
        (42, test42_3),
        (42, test42_4),
        (42, test42_5),
        (42, test42_6),
        (43, test43_1),
        (43, test43_2),
        (43, test43_3),
        (43, test43_4),
        (43, test43_5),
        (43, test43_6),
        (44, test44_1),
        (44, test44_2),
        (44, test44_3),
        (44, test44_4),
        (44, test44_5),
        (44, test44_6),
        (45, test45_1),
        (45, test45_2),
        (45, test45_3),
        (45, test45_4),
        (45, test45_5),
        (45, test45_6),
        (46, test46_1),
        (46, test46_2),
        (46, test46_3),
        (46, test46_4),
        (46, test46_5),
        (46, test46_6),
        (47, test47_1),
        (47, test47_2),
        (47, test47_3),
        (47, test47_4),
        (47, test47_5),
        (47, test47_6),
        (48, test48_1),
        (48, test48_2),
        (48, test48_3),
        (48, test48_4),
        (48, test48_5),
        (48, test48_6),
        (48, test48_7),
        (48, test48_8),
        (48, test48_9)
    ]);

    let code_struct: TuringCodeResults = TuringCodeResults {
        code: turing_code.clone(),
        checks: criteria_checks.clone(),
    };

    return code_struct;
}
