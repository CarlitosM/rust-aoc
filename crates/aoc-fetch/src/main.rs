use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::fs;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(
    name = "aoc-fetch",
    author = "CarlitosM",
    version = "0.1.0",
    about = "Fetches puzzle inputs for Advent of Code",
    long_about = "A CLI tool to fetch daily puzzle inputs from adventofcode.com and save them to the project directory."
)]
struct Args {
    /// The year of the Advent of Code challenge (e.g., 2023)
    #[arg(short, long)]
    year: u32,

    /// The day of the puzzle (1 to 25)
    #[arg(short, long)]
    day: u32,

    /// Advent of Code session cookie.
    /// If not provided, the tool will try the `AOC_SESSION` environment variable,
    /// followed by a `.session` file in the workspace root.
    #[arg(short, long)]
    session: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Validate Day
    if args.day < 1 || args.day > 25 {
        return Err(anyhow!("Day must be between 1 and 25 inclusive. Got: {}", args.day));
    }

    // 2. Resolve session cookie
    let session_cookie = aoc_core::resolve_session(args.session)?;

    // 3. Fetch the input
    println!("Fetching input for Year {}, Day {}...", args.year, args.day);
    let input_data = fetch_input(args.year, args.day, &session_cookie)?;

    // 4. Save the input to inputs/<year>/day<padded_day>.txt
    save_input(args.year, args.day, &input_data)?;

    println!("Success! Input saved.");
    Ok(())
}

fn fetch_input(year: u32, day: u32, session_cookie: &str) -> Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    // Advent of Code requests a polite User-Agent specifying codebase URL and contact/maintainer details
    let user_agent = "github.com/CarlitosM/rust-aoc by carlitos@example.com (via aoc-fetch CLI)";

    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .get(&url)
        .header("Cookie", format!("session={session_cookie}"))
        .send()
        .context("Failed to send request to Advent of Code")?;

    let status = response.status();
    if !status.is_success() {
        let body_err = response.text().unwrap_or_default();
        return Err(anyhow!(
            "Failed to download input. Status code: {status}.\nResponse: {}",
            body_err.trim()
        ));
    }

    let body = response.text().context("Failed to read response body")?;
    Ok(body)
}

fn save_input(year: u32, day: u32, data: &str) -> Result<()> {
    let dir_path = format!("inputs/{year}");
    fs::create_dir_all(&dir_path)
        .with_context(|| format!("Failed to create directory structure '{dir_path}'"))?;

    let file_path = format!("{dir_path}/day{day:02}.txt");
    let mut file = fs::File::create(&file_path)
        .with_context(|| format!("Failed to create file '{file_path}'"))?;

    file.write_all(data.as_bytes())
        .with_context(|| format!("Failed to write data to '{file_path}'"))?;

    Ok(())
}
