pub fn solve_part_two(input: &String) -> String {
    let number_literals = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];

    let mut sum = 0;
    let mut digits: Vec<u32> = Vec::new();

    for line in input.lines() {
        for i in 0..line.len() {
            let slice = &line[i..];
            if slice.len() == 0 { break }

            let first_char = slice.chars().nth(0).unwrap();
            if first_char.is_numeric() {
                digits.push(first_char.to_digit(10).unwrap());
            }

            for (number, literal) in number_literals.iter().enumerate() {
                if slice.starts_with(literal) {
                    digits.push((number + 1) as u32);
                }
            }
        }
        
        sum += digits[0] * 10 + digits[digits.len() - 1];

        digits.clear();
    }

    return sum.to_string();
}
