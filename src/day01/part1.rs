pub fn solve_part_one(input: &String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let digits: Vec<u32> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        sum += digits[0] * 10 + digits[digits.len() - 1];
    }

    return sum.to_string();
}