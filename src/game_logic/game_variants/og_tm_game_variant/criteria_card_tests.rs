use crate::game_logic::setup::TuringCodeEval;

pub fn evaluate_criteria_results(turing_code: u32) -> TuringCodeEval {
    let mut num: u32 = turing_code.clone();
    let purple: u32 = &num % 10;
    num /= 10;
    let yellow: u32 = &num % 10;
    num /= 10;
    let blue: u32 = num;

    // Criteria Card 1, "the Blue number compared to 1"
    let mut test1_1: bool = false; // Blue == 1
    let mut test1_2: bool = false; // Blue > 1
    // Criteria Card 2, "the Blue number compared to 3"
    let mut test2_1: bool = false; // Blue < 3
    let mut test2_2: bool = false; // Blue == 3
    let mut test2_3: bool = false; // Blue > 3
    // Criteria Card 3, "the Yellow number compared to 3"
    let mut test3_1: bool = false; // Yellow < 3
    let mut test3_2: bool = false; // Yellow == 3
    let mut test3_3: bool = false; // Yellow > 3
    // Criteria Card 4, "the Yellow number compared to 4"
    let mut test4_1: bool = false; // Yellow < 4
    let mut test4_2: bool = false; // Yellow == 4
    let mut test4_3: bool = false; // Yellow > 4
    // Criteria Card 26, "that a specific color is less than 3"
    let mut test26_1: bool = false; // Blue < 3
    let mut test26_2: bool = false; // Yellow < 3
    let mut test26_3: bool = false; // Purple < 3
    // Criteria Card 27, "that a specific color is less than 4"
    let mut test27_1: bool = false; // Blue < 4
    let mut test27_2: bool = false; // Yellow < 4
    let mut test27_3: bool = false; // Purple < 4
    // Criteria Card 28, "that a specific color is equal to 1"
    let mut test28_1: bool = false; // Blue == 1
    let mut test28_2: bool = false; // Yellow == 1
    let mut test28_3: bool = false; // Purple == 1
    // Criteria Card 29, "that a specific color is equal to 3"
    let mut test29_1: bool = false; // Blue == 3
    let mut test29_2: bool = false; // Yellow == 3
    let mut test29_3: bool = false; // Purple == 3
    // Criteria Card 30, "that a specific color is equal to 4"
    let mut test30_1: bool = false; // Blue == 4
    let mut test30_2: bool = false; // Purple == 4
    let mut test30_3: bool = false; // Yellow == 4
    // Criteria Card 31, "that a specific color is greater than 1"
    let mut test31_1: bool = false; // Blue > 1
    let mut test31_2: bool = false; // Yellow > 1
    let mut test31_3: bool = false; // Purple > 1
    // Criteria Card 32, "that a specific color is greater than 3"
    let mut test32_1: bool = false; // Blue > 3
    let mut test32_2: bool = false; // Yellow > 3
    let mut test32_3: bool = false; // Purple > 3
    // Criteria Card 39, "the number of one specific color compared to 1"
    let mut test39_1: bool = false; // Blue == 1
    let mut test39_2: bool = false; // Blue > 1
    let mut test39_3: bool = false; // Yellow == 1
    let mut test39_4: bool = false; // Yellow > 1
    let mut test39_5: bool = false; // Purple == 1
    let mut test39_6: bool = false; // Purple > 1
    // Criteria Card 40, "the number of one specific color compared to 3"
    let mut test40_1: bool = false; // Blue < 3
    let mut test40_2: bool = false; // Blue == 3
    let mut test40_3: bool = false; // Blue > 3
    let mut test40_4: bool = false; // Yellow < 3
    let mut test40_5: bool = false; // Yellow == 3
    let mut test40_6: bool = false; // Yellow > 3
    let mut test40_7: bool = false; // Purple < 3
    let mut test40_8: bool = false; // Purple == 3
    let mut test40_9: bool = false; // Purple > 3
    // Criteria Card 41, "the number of one specific color compared to 4"
    let mut test41_1: bool = false; // Blue < 4
    let mut test41_2: bool = false; // Blue == 4
    let mut test41_3: bool = false; // Blue > 4
    let mut test41_4: bool = false; // Yellow < 4
    let mut test41_5: bool = false; // Yellow == 4
    let mut test41_6: bool = false; // Yellow > 4
    let mut test41_7: bool = false; // Purple < 4
    let mut test41_8: bool = false; // Purple == 4
    let mut test41_9: bool = false; // Purple > 4

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

    // Criteria Card 5, "if Blue is even or odd"
    let mut test5_1: bool = false; // Blue is even
    let mut test5_2: bool = false; // Blue is odd
    // Criteria Card 6, "if Yellow is even or odd"
    let mut test6_1: bool = false; // Yellow is even
    let mut test6_2: bool = false; // Yellow is odd
    // Criteria Card 7, "if Purple is even or odd"
    let mut test7_1: bool = false; // Purple is even
    let mut test7_2: bool = false; // Purple is odd
    // Criteria Card 33, "that a specific color is even or odd"
    let mut test33_1: bool = false; // Blue is even
    let mut test33_2: bool = false; // Blue is odd
    let mut test33_3: bool = false; // Yellow is even
    let mut test33_4: bool = false; // Yellow is odd
    let mut test33_5: bool = false; // Purple is even
    let mut test33_6: bool = false; // Purple is odd

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

    // Criteria Card 8, "the number of 1's in the code"
    let mut test8_1: bool = false; // zero 1's
    let mut test8_2: bool = false; // one 1
    let mut test8_3: bool = false; // two 1's
    let mut test8_4: bool = false; // three 1's
    // Criteria Card 9, "the number of 3's in the code"
    let mut test9_1: bool = false; // zero 3's
    let mut test9_2: bool = false; // one 3
    let mut test9_3: bool = false; // two 3's
    let mut test9_4: bool = false; // three 3's
    // Criteria Card 10, "the number of 4's in the code"
    let mut test10_1: bool = false; // zero 4's
    let mut test10_2: bool = false; // one 4
    let mut test10_3: bool = false; // two 4's
    let mut test10_4: bool = false; // three 4's
    // Criteria Card 45, "how many 1's or how many 3's there are in the code"
    let mut test45_1: bool = false; // zero 1's
    let mut test45_2: bool = false; // zero 3's
    let mut test45_3: bool = false; // one 1
    let mut test45_4: bool = false; // one 3
    let mut test45_5: bool = false; // two 1's
    let mut test45_6: bool = false; // two 3's
    // Criteria Card 46, "how many 3's or how many 4's there are in the code"
    let mut test46_1: bool = false; // zero 3's
    let mut test46_2: bool = false; // zero 4's
    let mut test46_3: bool = false; // one 3
    let mut test46_4: bool = false; // one 4
    let mut test46_5: bool = false; // two 3's
    let mut test46_6: bool = false; // two 4's
    // Criteria Card 47, "how many 1's or how many 4's there are in the code",
    let mut test47_1: bool = false; // zero 1's
    let mut test47_2: bool = false; // zero 4's
    let mut test47_3: bool = false; // one 1
    let mut test47_4: bool = false; // one 4
    let mut test47_5: bool = false; // two 1's
    let mut test47_6: bool = false; // two 4's

    let mut num_of_1s: u8 = 0;
    let mut num_of_3s: u8 = 0;
    let mut num_of_4s: u8 = 0;
    
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

    // Criteria Card 11, "the Blue number compared to the Yellow number"
    let mut test11_1: bool = false; // Blue < Yellow
    let mut test11_2: bool = false; // Blue == Yellow
    let mut test11_3: bool = false; // Blue > Yellow
    // Criteria Card 12, "the Blue number compared to the Purple number"
    let mut test12_1: bool = false; // Blue < Purple
    let mut test12_2: bool = false; // Blue == Purple
    let mut test12_3: bool = false; // Blue > Purple
    // Criteria Card 13, "the Yellow number compared to the Purple number"
    let mut test13_1: bool = false; // Yellow < Purple
    let mut test13_2: bool = false; // Yellow == Purple
    let mut test13_3: bool = false; // Yellow > Purple
    // Criteria Card 43, "the Blue number compared to the number of another specific color"
    let mut test43_1: bool = false; // Blue < Yellow
    let mut test43_2: bool = false; // Blue < Purple
    let mut test43_3: bool = false; // Blue == Yellow
    let mut test43_4: bool = false; // Blue == Purple
    let mut test43_5: bool = false; // Blue > Yellow
    let mut test43_6: bool = false; // Blue > Purple
    // Criteria Card 44, "the Yellow number compared to the number of another specific color"
    let mut test44_1: bool = false; // Yellow < Blue
    let mut test44_2: bool = false; // Yellow < Purple
    let mut test44_3: bool = false; // Yellow == Blue
    let mut test44_4: bool = false; // Yellow == Purple
    let mut test44_5: bool = false; // Yellow > Blue
    let mut test44_6: bool = false; // Yellow > Purple
    // Criteria Card 48, "one specific color compared to another specific color"
    let mut test48_1: bool = false; // Blue < Yellow
    let mut test48_2: bool = false; // Blue == Yellow
    let mut test48_3: bool = false; // Blue > Yellow
    let mut test48_4: bool = false; // Blue < Purple
    let mut test48_5: bool = false; // Blue == Purple
    let mut test48_6: bool = false; // Blue > Purple
    let mut test48_7: bool = false; // Yellow < Purple
    let mut test48_8: bool = false; // Yellow == Purple
    let mut test48_9: bool = false; // Yellow > Purple

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

    // Criteria Card 14, "which color's number is smaller than either of the others"
    let mut test14_1: bool = false; // Blue < Yellow && Blue < Purple
    let mut test14_2: bool = false; // Yellow < Blue && Yellow < Purple
    let mut test14_3: bool = false; // Purple < Blue && Purple < Yellow

    if (blue < yellow) && (blue < purple) {
        test14_1 = true;
    }
    if (yellow < blue) && (yellow < purple) {
        test14_2 = true;
    }
    if (purple < blue) && (purple < yellow) {
        test14_3 = true;
    }

    // Criteria Card 15, "which color's number is larger than either of the others"
    let mut test15_1: bool = false; // Blue > Yellow && Blue > Purple
    let mut test15_2: bool = false; // Yellow > Blue && Yellow > Purple
    let mut test15_3: bool = false; // Purple > Blue && Purple > Yellow

    if (blue > yellow) && (blue > purple) {
        test15_1 = true;
    }
    if (yellow > blue) && (yellow > purple) {
        test15_2 = true;
    }
    if (purple > blue) && (purple > yellow) {
        test15_3 = true;
    }

    // Criteria Card 16, "the number of even numbers compared to the number of odd numbers"
    let mut test16_1: bool = false; // even > odd
    let mut test16_2: bool = false; // even < odd
    // Criteria Card 17, "how many even numbers there are in the code"
    let mut test17_1: bool = false; // zero even numbers
    let mut test17_2: bool = false; // one even number
    let mut test17_3: bool = false; // two even numbers
    let mut test17_4: bool = false; // three even numbers

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

    // Criteria Card 18, "if the sum of all the numbers is even or odd"
    let mut test18_1: bool = false; // Blue + Yellow + Purple == even
    let mut test18_2: bool = false; // Blue + Yellow + Purple == odd
    // Criteria Card 23, "the sum of all numbers compared to 6"
    let mut test23_1: bool = false; // Blue + Yellow + Purple < 6
    let mut test23_2: bool = false; // Blue + Yellow + Purple == 6
    let mut test23_3: bool = false; // Blue + Yellow + Purple < 6
    // Criteria Card 36, "the sum of all the numbers is a multiple of 3 or 4 or 5"
    let mut test36_1: bool = false; // Blue + Yellow + Purple % 3 == 0
    let mut test36_2: bool = false; // Blue + Yellow + Purple % 4 == 0
    let mut test36_3: bool = false; // Blue + Yellow + Purple % 5 == 0

    let all_color_sum = blue + yellow + purple;

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

    // Criteria Card 19, "the sum of Blue and Yellow compared to 6"
    let mut test19_1: bool = false; // Blue + Yellow < 6
    let mut test19_2: bool = false; // Blue + Yellow == 6
    let mut test19_3: bool = false; // Blue + Yellow > 6
    // Criteria Card 37, "the sum of two specific colors is equal to 4"
    let mut test37_1: bool = false; // Blue + Yellow == 4
    let mut test37_2: bool = false; // Blue + Purple == 4
    let mut test37_3: bool = false; // Yellow + Purple == 4
   // Criteria Card 38, "the sum of two specific colors is equal to 6"
    let mut test38_1: bool = false; // Blue + Yellow == 6
    let mut test38_2: bool = false; // Blue + Purple == 6
    let mut test38_3: bool = false; // Yellow + Purple == 6

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

    // Criteria Card 20, "if a number repeats itself in the code"
    let mut test20_1: bool = false; // a triple number
    let mut test20_2: bool = false; // a double number
    let mut test20_3: bool = false; // no repetition
    // Criteria Card 21, "if there is a number present exactly twice"
    let mut test21_1: bool = false; // no pairs || three of a kind
    let mut test21_2: bool = false; // a pair

    let mut num_repetitions: u8 = 0;
    
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

    // Criteria Card 22, "if the three numbers in the code are in ascending order, descending order, or no order"
    let mut test22_1: bool = false; // ascending order
    let mut test22_2: bool = false; // descending order
    let mut test22_3: bool = false; // no order

    if (blue < yellow) && (yellow < purple) {
        test22_1 = true;
    } else if (blue > yellow) && (yellow > purple) {
        test22_2 = true;
    } else {
        test22_3 = true;
    }

    // Criteria Card 24, "if there is a sequence of ascending numbers"
    let mut test24_1: bool = false; // three numbers in ascending order
    let mut test24_2: bool = false; // two numbers in ascending order
    let mut test24_3: bool = false; // no numbers in ascending order
    
    let mut test24_count: u8 = 0;

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

    // Criteria Card 25, "if there is a sequence of ascending or descending numbers"
    let mut test25_1: bool = false; // three numbers in ascending or descending order
    let mut test25_2: bool = false; // two numbers in ascending or descending order
    let mut test25_3: bool = false; // no numbers in ascending or descending order
    
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

    // Criteria Card 34, "which color has the smallest number (or is tied for the smallest number)"
    let mut test34_1: bool = false; // Blue <= Yellow && Blue <= Purple
    let mut test34_2: bool = false; // Yellow <= Blue && Yellow <= Purple
    let mut test34_3: bool = false; // Purple <= Blue && Purple <= Yellow

    if (blue <= yellow) && (blue <= purple) {
        test34_1 = true;
    }
    
    if (yellow <= blue) && (yellow <= purple) {
        test34_2 = true;
    }
    
    if (purple <= blue) && (purple <= yellow) {
        test34_3 = true;
    }

    // Criteria Card 35, "which color has the largest number (or is tied for the largest number)"
    let mut test35_1: bool = false; // Blue >= Yellow && Blue >= Purple
    let mut test35_2: bool = false; // Yellow >= Blue && Yellow >= Purple
    let mut test35_3: bool = false; // Purple >= Blue && Purple >= Yellow

    if (blue >= yellow) && (blue >= purple) {
        test35_1 = true;
    }
    
    if (yellow >= blue) && (yellow >= purple) {
        test35_2 = true;
    }
    
    if (purple >= blue) && (purple >= yellow) {
        test35_3 = true;
    }

    // Criteria Card 42, "which color is the smallest or the largest"
    let mut test42_1: bool = false; // Blue < Yellow && Blue < Purple
    let mut test42_2: bool = false; // Blue > Yellow && Blue > Purple
    let mut test42_3: bool = false; // Yellow < Blue && Yellow < Purple
    let mut test42_4: bool = false; // Yellow > Blue && Yellow > Purple
    let mut test42_5: bool = false; // Purple < Blue && Purple < Yellow
    let mut test42_6: bool = false; // Purple > Blue && Purple > Yellow

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
