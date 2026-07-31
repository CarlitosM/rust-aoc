use aoc_2023::Year2023;
use aoc_core::AoCYear;
use criterion::{criterion_group, criterion_main, Criterion};
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
            let answer = Year2023::solve(day, part, black_box(input)).unwrap_or_else(|error| {
                panic!("Year 2023 day {day:02} part {part} failed: {error}")
            });
            black_box(answer);
        });
    });
}

fn benchmark_days(c: &mut Criterion) {
    let day01 = load_input(2023, 1);
    bench_part(c, 1, 1, &day01);
    bench_part(c, 1, 2, &day01);
}

criterion_group!(benches, benchmark_days);
criterion_main!(benches);
