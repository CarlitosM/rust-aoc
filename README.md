This repository contains my solutions to the Advent of Code (AoC) challenges using Rust. Using this as part of my learning journey in Rust.

## Benchmarking

The year crates have Criterion benchmarks for every scaffolded day and both puzzle parts. Benchmarks load the real inputs from `inputs/{year}/day{NN}.txt`, so a missing input is reported as a benchmark error rather than silently replaced with a fixture.

Run all benchmarks for a year:

```bash
cargo bench -p aoc-2015 --bench days
```

Run one day or part using Criterion's filter:

```bash
cargo bench -p aoc-2015 --bench days -- day03/part1
```

Criterion stores reports under the selected year crate's `target/criterion`. Use the benchmark executable with [`samply`](https://github.com/mstange/samply) when investigating CPU hot spots:

```bash
samply record cargo bench -p aoc-2015 --bench days -- day03/part1
```

The benchmark target is optimized by Cargo. For useful profiles, keep debug symbols available in the toolchain/build configuration and profile one day or part at a time. On Linux, `samply` may require permission to access performance counters.

When `aoc-scaffold` creates a year, it adds Criterion and creates `benches/days.rs`. When it creates another day, it regenerates the explicit benchmark registrations so benchmark names remain stable.
