pub fn solve_part_one(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let list_start = line.find(':').expect("Ill formated input!");

        let game_id = &line["Game ".len()..list_start];
        let cube_sets = &line[list_start + 1..];

        let mut possible = true;
        'setloop: for set in cube_sets.split(';') {
            let set = set.trim();

            let cubes: Vec<(&str, &str)> = set
                .split(',')
                .map(|c| c.trim().split(' ').collect())
                .map(|ca: Vec<&str>| (ca[0].trim(), ca[1].trim()))
                .collect();

            for (count, color) in cubes {
                let count = count.parse::<i32>().unwrap();
                let max_count = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("Invalid color input! Excepted colors to be 'red', 'green' or 'blue'"),
                };

                if count > max_count {
                    possible = false;
                    break 'setloop;
                }
            }
        }

        if possible {
            sum += game_id.parse::<i32>().unwrap();
        }
    }

    sum.to_string()
}
