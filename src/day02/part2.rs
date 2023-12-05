pub fn solve_part_two(input: &String) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue }
        let list_start = line.find(":").expect("Ill formated input!");

        let cube_sets = &line[list_start + 1..];

        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for set in cube_sets.split(';') {
            let set = set.trim();

            let cubes: Vec<(&str, &str)> = set
                .split(',')
                .map(|c| c.trim().split(' ').collect())
                .map(|ca: Vec<&str>| (ca[0].trim(), ca[1].trim()))
                .collect();

            for (count, color) in cubes {
                let count = count.parse::<i32>().unwrap();

                match color {
                    "red" => if count > min_red { min_red = count; }
                    "green" => if count > min_green { min_green = count; }
                    "blue" => if count > min_blue { min_blue = count; }
                    _ => panic!()
                };
            }

        }
        
        let pow = min_red * min_green * min_blue;
        sum += pow;
    }

    return sum.to_string();
}
