pub fn generate_number_pool(length: usize, min: u8, max: u8) -> Vec<u32> {
    // For the original implementation of the "Turing Machine" Board Game, the values in the pool of valid number codes have a length of 3, with each digit having a value from 1 to 5, inclusive. This makes for one hundred twenty-five codes.
    //
    // 111, 112, 113, 114, 115,
    // 121, 122, 123, 124, 125,
    // 131, 132, 133, 134, 135,
    // 141, 142, 143, 144, 145,
    // 151, 152, 153, 154, 155,
    //
    // 211, 212, 213, 214, 215,
    // 221, 222, 223, 224, 225,
    // 231, 232, 233, 234, 235,
    // 241, 242, 243, 244, 245,
    // 251, 252, 253, 254, 255,
    //
    // 311, 312, 313, 314, 315,
    // 321, 322, 323, 324, 325,
    // 331, 332, 333, 334, 335,
    // 341, 342, 343, 344, 345,
    // 351, 352, 353, 354, 355,
    //
    // 411, 412, 413, 414, 415,
    // 421, 422, 423, 424, 425,
    // 431, 432, 433, 434, 435,
    // 441, 442, 443, 444, 445,
    // 451, 452, 453, 454, 455,
    //
    // 511, 512, 513, 514, 515,
    // 521, 522, 523, 524, 525,
    // 531, 532, 533, 534, 535,
    // 541, 542, 543, 544, 545,
    // 551, 552, 553, 554, 555

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
                    return result;
                } else {
                    strs[i] = min;
                }
            }
        }
    }
}

use rand::Rng;
pub fn rng_target_code(codes: Vec<u32>) -> u32 {
    let mut rng = rand::thread_rng();
    let target_code: u32 = codes[rng.gen_range(0..codes.len())];
    return target_code;
}


    name: String,
    game_id: String,
    guesses: Vec<u32>, // Assuming each guess is a Vec<u8> with a length of 3
    test_results: Vec<Vec<Checkbox>>, // Outer Vec for rounds, inner Vec for each test in a round
    code_digit_values: Vec<Vec<DigitValueState>>, // One Vec for each digit, inner Vec for each possible value
    notes: Vec<TestNotes>, // One TestNotes struct for each test
}