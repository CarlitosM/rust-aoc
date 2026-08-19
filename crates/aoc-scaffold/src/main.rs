//! Command-line scaffolding for Advent of Code year crates and day modules.
//!
//! The tool creates a new `aoc-{year}` crate when needed, adds the standard
//! runner wiring, and appends day modules under that year's `src/days` tree.

use anyhow::{Context, Result, bail};
use clap::Parser;

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Command-line arguments accepted by `aoc-scaffold`.
#[derive(Parser, Debug)]
#[command(
    name = "aoc-scaffold",
    author = "CarlitosM",
    version = "0.1.0",
    about = "Scaffold for Advent of Code",
    long_about = "Scaffold for Advent of Code by year or day"
)]
struct Cli {
    /// The AOC year to scaffold
    #[arg(short, long)]
    year: u32,

    /// The AOC day to scaffold (1-25), optional
    #[arg(
        short,
        long,
        value_parser = clap::value_parser!(u32).range(1..=25)
    )]
    day: Option<u32>,
}

/// Source templates written into generated Advent of Code crates.
struct Templates;
impl Templates {
    /// Builds the binary runner entry point for a generated year crate.
    fn run(year: u32) -> String {
        format!(
            r"use aoc_{year}::Year{year};

fn main() -> anyhow::Result<()> {{
    aoc_core::runner::run::<Year{year}>()
}}
"
        )
    }

    /// Builds the `lib.rs` implementation that dispatches year/day solutions.
    fn lib(year: u32) -> String {
        format!(
            r#"mod days;

use anyhow::{{Result, anyhow}};
use aoc_core::AoCYear;

pub struct Year{year};
impl AoCYear for Year{year} {{
    const YEAR: u32 = {year};
    type Answer = String;

    fn solve(day: u32, part: u32, input: &str) -> Result<Self::Answer> {{
        match (day, part) {{
            (1, 1) => days::day01::part1(input),
            (1, 2) => days::day01::part2(input),
            _ => Err(anyhow!("Day {{day}} part {{part}} is not yet implemented")),
        }}
    }}
}}
"#
        )
    }

    /// Builds the default day module with placeholder solutions and tests.
    fn day() -> String {
        r#"use anyhow::Result;

#[allow(clippy::unnecessary_wraps)]
pub fn part1(input: &str) -> Result<String> {
    let _ = input;
    Ok("TODO".to_string())
}

#[allow(clippy::unnecessary_wraps)]
pub fn part2(input: &str) -> Result<String> {
    let _ = input;
    Ok("TODO".to_string())
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test_aoc_test_inputs() {
        assert!(false);
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test_aoc_test_inputs() {
        assert!(false);
    }
}
"#
        .to_string()
    }

    /// Builds the module declaration appended to `src/days/mod.rs`.
    fn append_mod(day: u32) -> String {
        format!("pub mod day{day:02};\n")
    }

    /// Builds the Criterion benchmark harness for a generated year crate.
    fn bench(year: u32, days: &[u32]) -> String {
        let input_bindings = days
            .iter()
            .map(|day| format!("    let day{day:02} = load_input({year}, {day});"))
            .collect::<Vec<_>>()
            .join("\n");
        let benchmark_calls = days
            .iter()
            .flat_map(|day| {
                [
                    format!("    bench_part(c, {day}, 1, &day{day:02});"),
                    format!("    bench_part(c, {day}, 2, &day{day:02});"),
                ]
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"use aoc_core::AoCYear;
use aoc_{year}::Year{year};
use criterion::{{Criterion, criterion_group, criterion_main}};
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;

fn load_input(year: u32, day: u32) -> String {{
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../inputs")
        .join(year.to_string())
        .join(format!("day{{day:02}}.txt"));

    fs::read_to_string(&path).unwrap_or_else(|error| {{
        panic!(
            "Unable to load benchmark input for year {{year}}, day {{day:02}} at {{}}: {{error}}",
            path.display()
        )
    }})
}}

fn bench_part(c: &mut Criterion, day: u32, part: u32, input: &str) {{
    c.bench_function(&format!("day{{day:02}}/part{{part}}"), |b| {{
        b.iter(|| {{
            let answer = Year{year}::solve(day, part, black_box(input)).unwrap_or_else(|error| {{
                panic!("Year {year} day {{day:02}} part {{part}} failed: {{error}}")
            }});
            black_box(answer);
        }});
    }});
}}

fn benchmark_days(c: &mut Criterion) {{
{input_bindings}
{benchmark_calls}
}}

criterion_group!(benches, benchmark_days);
criterion_main!(benches);
"#
        )
    }
}

/// Filesystem paths used while scaffolding a generated year crate.
struct ScaffoldDirs {
    /// Root directory for the generated `aoc-{year}` crate.
    year: PathBuf,
    /// Directory containing generated day modules.
    days: PathBuf,
    /// Source directory for the generated crate.
    src: PathBuf,
    /// Binary directory containing the generated runner.
    bin: PathBuf,
    /// Directory containing generated Criterion benchmark harnesses.
    benches: PathBuf,
}

impl ScaffoldDirs {
    /// Resolves all paths needed to scaffold the requested Advent of Code year.
    fn new(year: u32) -> Result<Self> {
        let year = ScaffoldDirs::aoc_year_dir(year)?;
        let src = year.join("src");
        let days = src.join("days");
        let bin = src.join("bin");
        let benches = year.join("benches");

        Ok(Self {
            year,
            days,
            src,
            bin,
            benches,
        })
    }

    /// Returns the workspace root based on Cargo's manifest directory.
    fn repo_root() -> Result<PathBuf> {
        let manifest_dir = option_env!("CARGO_MANIFEST_DIR")
            .ok_or(anyhow::anyhow!("CARGO_MANIFEST_DIR not set"))?;

        Ok(PathBuf::from(manifest_dir).join("..").join(".."))
    }

    /// Returns the crate directory for a generated Advent of Code year.
    fn aoc_year_dir(year: u32) -> Result<PathBuf> {
        Ok(ScaffoldDirs::repo_root()?
            .join("crates")
            .join(format!("aoc-{year}")))
    }
}

/// Parses CLI arguments and scaffolds the requested year or day.
fn main() -> Result<()> {
    let args = Cli::parse();

    let aoc_dirs = ScaffoldDirs::new(args.year)?;

    if args.day.is_none() {
        scaffold_year(&aoc_dirs, args.year)?;
    }

    scaffold_day(&aoc_dirs, args.day.unwrap_or(1))?;

    run_cargo(
        Command::new("cargo").arg("fmt").current_dir(&aoc_dirs.year),
        "format code",
    )?;

    Ok(())
}

/// Creates a new year crate and writes its initial runner and library files.
fn scaffold_year(aoc_dirs: &ScaffoldDirs, year: u32) -> Result<()> {
    let crates_dir = aoc_dirs.year.parent().with_context(|| {
        format!(
            "Could not find parent directory for {}",
            aoc_dirs.year.display()
        )
    })?;
    let crate_name = format!("aoc-{year}");

    run_cargo(
        Command::new("cargo")
            .arg("new")
            .arg(&crate_name)
            .arg("--lib")
            .current_dir(crates_dir),
        &format!("create new crate {crate_name}"),
    )?;

    run_cargo(
        Command::new("cargo")
            .arg("add")
            .arg("anyhow")
            .current_dir(&aoc_dirs.year),
        &format!("add anyhow dependency to {crate_name}"),
    )?;

    run_cargo(
        Command::new("cargo")
            .arg("add")
            .arg("aoc-core")
            .arg("--path")
            .arg("../aoc-core")
            .current_dir(&aoc_dirs.year),
        &format!("add aoc-core dependency to {crate_name}"),
    )?;

    run_cargo(
        Command::new("cargo")
            .arg("add")
            .arg("criterion")
            .arg("--dev")
            .arg("--features")
            .arg("html_reports")
            .current_dir(&aoc_dirs.year),
        &format!("add Criterion dev-dependency to {crate_name}"),
    )?;

    configure_bench_cargo(aoc_dirs)?;

    fs::create_dir_all(&aoc_dirs.bin)
        .with_context(|| format!("Failed to create bin directory: {}", aoc_dirs.bin.display()))?;
    fs::create_dir_all(&aoc_dirs.days).with_context(|| {
        format!(
            "Failed to create days directory: {}",
            aoc_dirs.days.display()
        )
    })?;
    fs::create_dir_all(&aoc_dirs.benches).with_context(|| {
        format!(
            "Failed to create benchmark directory: {}",
            aoc_dirs.benches.display()
        )
    })?;

    let run_path = aoc_dirs.bin.join("run.rs");
    fs::write(&run_path, Templates::run(year))
        .with_context(|| format!("Failed to write run template to {}", run_path.display()))?;

    let lib_path = aoc_dirs.src.join("lib.rs");
    fs::write(&lib_path, Templates::lib(year))
        .with_context(|| format!("Failed to write lib template to {}", lib_path.display()))?;

    let bench_path = aoc_dirs.benches.join("days.rs");
    fs::write(&bench_path, Templates::bench(year, &[1])).with_context(|| {
        format!(
            "Failed to write benchmark template to {}",
            bench_path.display()
        )
    })?;

    Ok(())
}

/// Adds the Cargo configuration required for a Criterion benchmark target.
fn configure_bench_cargo(aoc_dirs: &ScaffoldDirs) -> Result<()> {
    let cargo_path = aoc_dirs.year.join("Cargo.toml");
    let mut cargo_toml = fs::read_to_string(&cargo_path)
        .with_context(|| format!("Failed to read {}", cargo_path.display()))?;

    if !cargo_toml.contains("[[bench]]\nname = \"days\"") {
        cargo_toml.push_str(
            r#"

[[bench]]
name = "days"
harness = false
"#,
        );
        fs::write(&cargo_path, cargo_toml)
            .with_context(|| format!("Failed to update {}", cargo_path.display()))?;
    }

    Ok(())
}

/// Writes a day module and ensures it is exported from the generated day tree.
fn scaffold_day(aoc_dirs: &ScaffoldDirs, day: u32) -> Result<()> {
    let day_file_path = aoc_dirs.days.join(format!("day{day:02}.rs"));

    fs::write(&day_file_path, Templates::day()).with_context(|| {
        format!(
            "Failed to write day template to {}",
            day_file_path.display()
        )
    })?;

    let mod_file_path = aoc_dirs.days.join("mod.rs");
    let mod_line = Templates::append_mod(day);

    let existing_content = if mod_file_path.exists() {
        fs::read_to_string(&mod_file_path)
            .with_context(|| format!("Failed to read {}", mod_file_path.display()))?
    } else {
        String::new()
    };

    if !existing_content.contains(mod_line.trim()) {
        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&mod_file_path)
            .with_context(|| format!("Failed to open or create {}", mod_file_path.display()))?;

        file.write_all(mod_line.as_bytes())
            .with_context(|| format!("Failed to append to {}", mod_file_path.display()))?;
    }

    scaffold_bench(aoc_dirs, day)
}

/// Rebuilds the explicit benchmark registrations from the exported day modules.
fn scaffold_bench(aoc_dirs: &ScaffoldDirs, _day: u32) -> Result<()> {
    let mod_file_path = aoc_dirs.days.join("mod.rs");
    let mod_content = fs::read_to_string(&mod_file_path)
        .with_context(|| format!("Failed to read {}", mod_file_path.display()))?;
    let mut days = mod_content
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub mod day"))
        .filter_map(|day| day.trim_end_matches(';').parse::<u32>().ok())
        .collect::<Vec<_>>();
    days.sort_unstable();
    days.dedup();

    fs::create_dir_all(&aoc_dirs.benches).with_context(|| {
        format!(
            "Failed to create benchmark directory: {}",
            aoc_dirs.benches.display()
        )
    })?;

    let year = aoc_dirs
        .year
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_prefix("aoc-"))
        .and_then(|year| year.parse::<u32>().ok())
        .with_context(|| format!("Could not determine year from {}", aoc_dirs.year.display()))?;
    let bench_path = aoc_dirs.benches.join("days.rs");
    fs::write(&bench_path, Templates::bench(year, &days)).with_context(|| {
        format!(
            "Failed to write benchmark template to {}",
            bench_path.display()
        )
    })?;

    Ok(())
}

/// Runs a Cargo command and annotates failures with the requested action.
fn run_cargo(command: &mut Command, description: &str) -> Result<()> {
    let status = command
        .status()
        .with_context(|| format!("Failed to run cargo command to {description}"))?;

    if !status.success() {
        bail!("Cargo command failed to {description}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Templates;

    #[test]
    fn benchmark_template_registers_each_day_and_part() {
        let template = Templates::bench(2099, &[1, 4]);

        assert!(template.contains("let day01 = load_input(2099, 1);"));
        assert!(template.contains("let day04 = load_input(2099, 4);"));
        assert!(template.contains("bench_part(c, 1, 1, &day01);"));
        assert!(template.contains("bench_part(c, 1, 2, &day01);"));
        assert!(template.contains("bench_part(c, 4, 1, &day04);"));
        assert!(template.contains("bench_part(c, 4, 2, &day04);"));
        assert!(template.contains("day{day:02}/part{part}"));
    }
}
