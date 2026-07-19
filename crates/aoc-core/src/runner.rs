use crate::{resolve_session, submit_answer, Level, Submission, SubmissionResult};
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::fmt::Display;
use std::fs;
use std::io::{self, Write};

/// Trait that each year's solution crate implements.
///
/// The single `solve` method dispatches to the appropriate day/part logic
/// via a match expression in the implementing type.
pub trait AoCYear {
    /// The puzzle year (e.g. 2023).
    const YEAR: u32;

    /// The answer type. Must implement `Display` so the runner can print
    /// and submit it as a string.
    type Answer: Display;

    /// Solve the given day and part using the provided puzzle input.
    ///
    /// # Errors
    ///
    /// Returns an error if the day/part combination is unsupported or if the
    /// puzzle input cannot be parsed.
    fn solve(day: u32, part: u32, input: &str) -> Result<Self::Answer>;
}

/// CLI arguments for the AOC runner.
#[derive(Parser, Debug)]
#[command(
    name = "aoc-runner",
    about = "Runs Advent of Code solutions and optionally submits answers"
)]
struct Args {
    /// The day of the puzzle (1 to 25)
    day: u32,

    /// The part of the puzzle (1 or 2)
    part: u32,

    /// Submit the answer to Advent of Code after computing it
    #[arg(long)]
    submit: bool,
}

/// Entry point for year-crate binaries.
///
/// Parses CLI args, loads input, runs the solution, and optionally submits.
///
/// # Errors
///
/// Returns an error if the day or part arguments are out of range, if the
/// puzzle input file cannot be read, if the solution itself fails, or if
/// answer submission encounters a network or session error.
///
/// # Usage
///
/// ```text
/// cargo run --bin run -- <day> <part> [--submit]
/// ```
pub fn run<Y: AoCYear>() -> Result<()> {
    let args = Args::parse();

    // Validate
    if !(1..=25).contains(&args.day) {
        return Err(anyhow!("Day must be between 1 and 25. Got: {}", args.day));
    }
    if args.part != 1 && args.part != 2 {
        return Err(anyhow!("Part must be 1 or 2. Got: {}", args.part));
    }

    // Load input
    let input = load_input(Y::YEAR, args.day)?;

    // Run the solution
    println!(
        "--- Year {}, Day {:02}, Part {} ---",
        Y::YEAR, args.day, args.part
    );
    let answer = Y::solve(args.day, args.part, &input)?;
    println!("Answer: {answer}");

    // Submit if requested
    if args.submit {
        let level = if args.part == 1 {
            Level::Part1
        } else {
            Level::Part2
        };
        let submission = Submission {
            year: Y::YEAR,
            day: args.day,
            level,
            answer: answer.to_string(),
        };
        submit_with_confirmation(&submission)?;
    }

    Ok(())
}

/// Loads the puzzle input from `inputs/{year}/day{day:02}.txt`.
///
/// The path is resolved relative to the current working directory,
/// which should be the workspace root when using `cargo run`.
fn load_input(year: u32, day: u32) -> Result<String> {
    let path = format!("inputs/{year}/day{day:02}.txt");
    fs::read_to_string(&path).with_context(|| {
        format!(
            "Failed to read input file '{path}'.\n\
             Make sure to fetch the input first with: \
             cargo run -p aoc-fetch -- --year {year} --day {day}"
        )
    })
}

/// Submits an answer with interactive y/n confirmation.
fn submit_with_confirmation(submission: &Submission) -> Result<()> {
    print!(
        "Submit '{}' as answer for Year {}, Day {}, Part {}? [y/N] ",
        submission.answer, submission.year, submission.day, submission.level.as_str()
    );
    io::stdout().flush()?;

    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    if response.trim().eq_ignore_ascii_case("y") {
        let session = resolve_session(None)?;
        let result = submit_answer(&session, submission)?;
        print_submission_result(&result);
    } else {
        println!("Submission cancelled.");
    }

    Ok(())
}

/// Pretty-prints the submission result.
fn print_submission_result(result: &SubmissionResult) {
    match result {
        SubmissionResult::Correct => {
            println!("🎄 Correct! You've earned a gold star! ⭐");
        }
        SubmissionResult::Incorrect { description } => {
            println!("❌ Incorrect — {description}.");
        }
        SubmissionResult::WaitTime { remaining } => {
            println!("⏳ Rate limited — please wait {remaining} before submitting again.");
        }
        SubmissionResult::AlreadyCompleted => {
            println!("✅ Already completed! You've already solved this part.");
        }
        SubmissionResult::Unknown(raw) => {
            println!("❓ Unknown response from AOC:\n{raw}");
        }
    }
}
