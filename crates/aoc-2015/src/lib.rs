mod days;

use anyhow::{Result, anyhow};
use aoc_core::AoCYear;

pub struct Year2015;
impl AoCYear for Year2015 {
    const YEAR: u32 = 2015;
    type Answer = String;

    fn solve(day: u32, part: u32, input: &str) -> Result<Self::Answer> {
        match (day, part) {
            (1, 1) => days::day01::part1(input),
            (1, 2) => days::day01::part2(input),
            (2, 1) => days::day02::part1(input),
            (2, 2) => days::day02::part2(input),
            (3, 1) => days::day03::part1(input),
            (3, 2) => days::day03::part2(input),
            (4, 1) => days::day04::part1(input),
            (4, 2) => days::day04::part2(input),
            (5, 1) => days::day05::part1(input),
            (5, 2) => days::day05::part2(input),
            _ => Err(anyhow!("Day {day} part {part} is not yet implemented")),
        }
    }
}
