mod days;

use aoc_core::AoCYear;
use anyhow::{anyhow, Result};

pub struct Year2023;

impl AoCYear for Year2023 {
    const YEAR: u32 = 2023;
    type Answer = String;

    fn solve(day: u32, part: u32, input: &str) -> Result<Self::Answer> {
        match (day, part) {
            (1, 1) => days::day01::part1(input),
            (1, 2) => days::day01::part2(input),
            // (2, 1) => days::day02::part1(input),
            // (2, 2) => days::day02::part2(input),
            _ => Err(anyhow!("Day {day} part {part} is not yet implemented")),
        }
    }
}
