use aoc_2015::Year2015;
use aoc_core::AoCYear;
use criterion::{Criterion, criterion_group, criterion_main};
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;

fn load_input(year: u32, day: u32) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../inputs")
        .join(year.to_string())
        .join(format!("day{day:02}.txt"));

    fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "Unable to load benchmark input for year {year}, day {day:02} at {}: {error}",
            path.display()
        )
    })
}

fn bench_part(c: &mut Criterion, day: u32, part: u32, input: &str) {
    c.bench_function(&format!("day{day:02}/part{part}"), |b| {
        b.iter(|| {
            let answer = Year2015::solve(day, part, black_box(input)).unwrap_or_else(|error| {
                panic!("Year 2015 day {day:02} part {part} failed: {error}")
            });
            black_box(answer);
        });
    });
}

fn benchmark_days(c: &mut Criterion) {
    let day01 = load_input(2015, 1);
    let day02 = load_input(2015, 2);
    let day03 = load_input(2015, 3);
    let day04 = load_input(2015, 4);
    let day05 = load_input(2015, 5);
    let day06 = load_input(2015, 6);
    bench_part(c, 1, 1, &day01);
    bench_part(c, 1, 2, &day01);
    bench_part(c, 2, 1, &day02);
    bench_part(c, 2, 2, &day02);
    bench_part(c, 3, 1, &day03);
    bench_part(c, 3, 2, &day03);
    bench_part(c, 4, 1, &day04);
    bench_part(c, 4, 2, &day04);
    bench_part(c, 5, 1, &day05);
    bench_part(c, 5, 2, &day05);
    bench_part(c, 6, 1, &day06);
    bench_part(c, 6, 2, &day06);
}

criterion_group!(benches, benchmark_days);
criterion_main!(benches);
