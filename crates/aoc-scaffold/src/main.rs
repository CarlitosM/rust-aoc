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
        value_parser = clap::value_parser!(u32).range(2..=25)
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

pub fn part1(input: &str) -> Result<String> {
    let _ = input;
    Ok("TODO".to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let _ = input;
    Ok("TODO".to_string())
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn placeholder_part1() -> Result<()> {
        assert_eq!(part1("TODO")?, "TODO");
        Ok(())
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn placeholder_part2() -> Result<()> {
        assert_eq!(part2("TODO")?, "TODO");
        Ok(())
    }
}
"#
        .to_string()
    }

    /// Builds the module declaration appended to `src/days/mod.rs`.
    fn append_mod(day: u32) -> String {
        format!("pub mod day{day:02};\n")
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
}

impl ScaffoldDirs {
    /// Resolves all paths needed to scaffold the requested Advent of Code year.
    fn new(year: u32) -> Result<Self> {
        let year = ScaffoldDirs::aoc_year_dir(year)?;
        let src = year.join("src");
        let days = src.join("days");
        let bin = src.join("bin");

        Ok(Self {
            year,
            days,
            src,
            bin,
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

    fs::create_dir_all(&aoc_dirs.bin)
        .with_context(|| format!("Failed to create bin directory: {}", aoc_dirs.bin.display()))?;
    fs::create_dir_all(&aoc_dirs.days).with_context(|| {
        format!(
            "Failed to create days directory: {}",
            aoc_dirs.days.display()
        )
    })?;

    let run_path = aoc_dirs.bin.join("run.rs");
    fs::write(&run_path, Templates::run(year))
        .with_context(|| format!("Failed to write run template to {}", run_path.display()))?;

    let lib_path = aoc_dirs.src.join("lib.rs");
    fs::write(&lib_path, Templates::lib(year))
        .with_context(|| format!("Failed to write lib template to {}", lib_path.display()))?;

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
