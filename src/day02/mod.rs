mod part1;
mod part2;

use crate::common::Day;

pub fn day_data() -> Day {
    Day {
        day_number: 2,
        name: "Day 2: Cube Conundrum",
        part_one_fn: part1::solve_part_one,
        part_two_fn: part2::solve_part_two,
    }
}