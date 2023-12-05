pub struct Day {
    pub day_number: u8,
    pub name: &'static str,
    pub part_one_fn: fn(&str) -> String,
    pub part_two_fn: fn(&str) -> String,
}
