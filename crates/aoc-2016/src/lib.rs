mod days;

use anyhow::{Result, anyhow};
use aoc_core::AoCYear;

pub struct Year2016;
impl AoCYear for Year2016 {
    const YEAR: u32 = 2016;
    type Answer = String;

    fn solve(day: u32, part: u32, input: &str) -> Result<Self::Answer> {
        match (day, part) {
            (1, 1) => days::day01::part1(input),
            (1, 2) => days::day01::part2(input),
            _ => Err(anyhow!("Day {day} part {part} is not yet implemented")),
        }
    }
}
