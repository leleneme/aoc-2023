pub fn solve_part_one(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let digits: Vec<u32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        sum += digits[0] * 10 + digits[digits.len() - 1];
    }

    sum.to_string()
}
