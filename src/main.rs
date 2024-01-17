fn generate_number_pool(length: usize, min: u8, max: u8) -> Vec<u32> {
    let mut result = Vec::new();
    let mut strs = vec![min; length];

    loop {
        let number: u32 = strs.iter()
                              .map(|&n| n.to_string())
                              .collect::<String>()
                              .parse()
                              .unwrap_or(0);

        result.push(number);

        for i in (0..length).rev() {
            if strs[i] < max {
                strs[i] += 1;
                break;
            } else if i == 0 {
                return result;
            } else {
                strs[i] = min;
            }
        }
    }
}

fn main() {
    println!("~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ ~~~ \n\nWelcome to the Turing Machine CLI!\n\nThis program is a personal project based off of the board game \"Turing Machine\" designed by Fabien Gridel & Yoann Levet");
    println!("{:?}", generate_number_pool(3, 1, 5));
}
