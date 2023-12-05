pub mod common;
pub mod day01;
pub mod day02;

pub fn banana() {}

fn main() {
    let days = vec![day01::day_data(), day02::day_data()];

    println!(
        "| {}๑(◕‿◕)๑ Advent of Code  2023{} |",
        " ".repeat(12),
        " ".repeat(12),
    );

    println!(
        "| {:<21} | {:<7} | {:<7} | {:<8} |",
        "Day", "Part 1", "Part 2", "Elapsed"
    );

    for day in days {
        let input_path = format!("inputs/day{}.txt", day.day_number);
        let input = std::fs::read_to_string(input_path.clone())
            .unwrap_or_else(|_| panic!("Failed to open file {}", input_path));

        use std::time::Instant;
        let now = Instant::now();

        let output_1 = (day.part_one_fn)(&input);
        let output_2 = (day.part_two_fn)(&input);

        let elapsed = now.elapsed();

        println!(
            "| {:<21} | {:<7} | {:<7} | {:<5} μs |",
            day.name,
            output_1,
            output_2,
            elapsed.as_micros()
        )
    }
}
