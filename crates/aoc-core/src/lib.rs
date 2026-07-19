pub mod runner;
pub use runner::AoCYear;

use anyhow::{anyhow, Context, Result};
use std::fs;

/// Resolves the Advent of Code session cookie.
///
/// Priority:
/// 1. `cli_session` if provided and non-empty.
/// 2. `AOC_SESSION` environment variable if non-empty.
/// 3. `.session` file in the current working directory if non-empty.
///
/// # Errors
///
/// Returns an error if no non-empty session cookie can be found via any of the prioritized methods.
pub fn resolve_session(cli_session: Option<String>) -> Result<String> {
    if let Some(cookie) = cli_session {
        let trimmed = cookie.trim().to_string();
        if !trimmed.is_empty() {
            return Ok(trimmed);
        }
    }

    if let Ok(cookie) = std::env::var("AOC_SESSION") {
        let trimmed = cookie.trim().to_string();
        if !trimmed.is_empty() {
            return Ok(trimmed);
        }
    }

    if let Ok(content) = fs::read_to_string(".session") {
        let trimmed = content.trim().to_string();
        if !trimmed.is_empty() {
            return Ok(trimmed);
        }
    }

    Err(anyhow!(
        "No Advent of Code session cookie found.\n\
        Please provide it using one of the following methods:\n\
        1. Pass the `--session` (or `-s`) flag.\n\
        2. Set the `AOC_SESSION` environment variable.\n\
        3. Create a `.session` file in the workspace root containing your cookie."
    ))
}

/// Represents the part / level of the daily challenge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Part1,
    Part2,
}

impl Level {
    /// Returns the string representation of the level as expected by the AOC form ("1" or "2").
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Part1 => "1",
            Self::Part2 => "2",
        }
    }
}

/// Represents the submission data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Submission {
    pub year: u32,
    pub day: u32,
    pub level: Level,
    pub answer: String,
}

/// Represents the parsed result of the answer submission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmissionResult {
    /// The submitted answer is correct!
    Correct,
    /// The submitted answer is incorrect.
    Incorrect {
        /// A description of the incorrect state (e.g., "too high", "too low", or "incorrect").
        description: String,
    },
    /// Rate limit hit, requiring a wait before submitting again.
    WaitTime {
        /// The parsed remaining wait time (e.g. "1m 20s").
        remaining: String,
    },
    /// The level was already completed previously.
    AlreadyCompleted,
    /// An unknown response was returned from the server.
    Unknown(String),
}

/// Submits an answer to Advent of Code.
///
/// # Errors
///
/// Returns an error if:
/// - The HTTP client failed to build.
/// - The request failed to send.
/// - The server returned a non-success HTTP status code.
pub fn submit_answer(
    session_cookie: &str,
    submission: &Submission,
) -> Result<SubmissionResult> {
    let url = format!(
        "https://adventofcode.com/{}/day/{}/answer",
        submission.year, submission.day
    );
    let user_agent = "github.com/CarlitosM/rust-aoc by carlitos@example.com (via aoc-fetch CLI)";

    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()
        .context("Failed to build HTTP client")?;

    let params = [
        ("level", submission.level.as_str()),
        ("answer", &submission.answer),
    ];

    let response = client
        .post(&url)
        .header("Cookie", format!("session={session_cookie}"))
        .form(&params)
        .send()
        .context("Failed to send submission to Advent of Code")?;

    let status = response.status();
    if !status.is_success() {
        let body_err = response.text().unwrap_or_default();
        return Err(anyhow!(
            "Failed to submit answer. Status code: {status}.\nResponse: {}",
            body_err.trim()
        ));
    }

    let body = response.text().context("Failed to read response body")?;
    Ok(parse_submission_response(&body))
}

/// Parses the response HTML from Advent of Code to extract the submission result.
#[must_use]
pub fn parse_submission_response(html: &str) -> SubmissionResult {
    let text = extract_article_text(html);

    if text.contains("That's the right answer") {
        return SubmissionResult::Correct;
    }

    if text.contains("too low") {
        return SubmissionResult::Incorrect {
            description: "too low".to_string(),
        };
    }

    if text.contains("too high") {
        return SubmissionResult::Incorrect {
            description: "too high".to_string(),
        };
    }

    if text.contains("not the right answer") {
        return SubmissionResult::Incorrect {
            description: "incorrect".to_string(),
        };
    }

    if text.contains("You gave an answer too recently") {
        let remaining = extract_wait_time(&text);
        return SubmissionResult::WaitTime { remaining };
    }

    if text.contains("solving the right level")
        || text.contains("already solved")
        || text.contains("Already completed")
    {
        return SubmissionResult::AlreadyCompleted;
    }

    SubmissionResult::Unknown(text)
}

fn extract_wait_time(text: &str) -> String {
    let Some(start_idx) = text.find("you have ") else {
        return "unknown duration".to_string();
    };
    let after_start = &text[start_idx + "you have ".len()..];
    let Some(end_idx) = after_start.find(" left to wait") else {
        return "unknown duration".to_string();
    };
    after_start[..end_idx].trim().to_string()
}

fn extract_article_text(html: &str) -> String {
    let Some(start_idx) = html.find("<article>") else {
        return strip_html_tags(html);
    };
    let after_start = &html[start_idx + "<article>".len()..];
    let Some(end_idx) = after_start.find("</article>") else {
        return strip_html_tags(html);
    };
    strip_html_tags(&after_start[..end_idx])
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }
    result
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_session_from_cli() {
        let res = resolve_session(Some("cli-token".to_string()));
        assert_eq!(res.unwrap(), "cli-token");
    }

    #[test]
    fn test_resolve_session_from_env() {
        unsafe {
            std::env::set_var("AOC_SESSION", "env-token");
        }
        let res = resolve_session(None);
        unsafe {
            std::env::remove_var("AOC_SESSION");
        }
        assert_eq!(res.unwrap(), "env-token");
    }

    #[test]
    fn test_resolve_session_precedence() {
        unsafe {
            std::env::set_var("AOC_SESSION", "env-token");
        }
        let res = resolve_session(Some("cli-token".to_string()));
        unsafe {
            std::env::remove_var("AOC_SESSION");
        }
        assert_eq!(res.unwrap(), "cli-token");
    }

    #[test]
    fn test_parse_correct() {
        let html = "<article><p>That's the right answer! You've won one gold star...</p></article>";
        assert_eq!(parse_submission_response(html), SubmissionResult::Correct);
    }

    #[test]
    fn test_parse_too_low() {
        let html = "<article><p>That's not the right answer; your answer is too low. If you're stuck...</p></article>";
        assert_eq!(
            parse_submission_response(html),
            SubmissionResult::Incorrect { description: "too low".to_string() }
        );
    }

    #[test]
    fn test_parse_too_high() {
        let html = "<article><p>That's not the right answer; your answer is too high. If you're stuck...</p></article>";
        assert_eq!(
            parse_submission_response(html),
            SubmissionResult::Incorrect { description: "too high".to_string() }
        );
    }

    #[test]
    fn test_parse_generic_incorrect() {
        let html = "<article><p>That's not the right answer. If you're stuck...</p></article>";
        assert_eq!(
            parse_submission_response(html),
            SubmissionResult::Incorrect { description: "incorrect".to_string() }
        );
    }

    #[test]
    fn test_parse_wait_time() {
        let html = "<article><p>You gave an answer too recently; you have 1m 20s left to wait. Please wait...</p></article>";
        assert_eq!(
            parse_submission_response(html),
            SubmissionResult::WaitTime { remaining: "1m 20s".to_string() }
        );
    }

    #[test]
    fn test_parse_already_completed() {
        let html = "<article><p>You don't seem to be solving the right level. Can you go back...</p></article>";
        assert_eq!(parse_submission_response(html), SubmissionResult::AlreadyCompleted);
    }

    #[test]
    fn test_parse_unknown() {
        let html = "<article><p>Some unexpected response from the server</p></article>";
        assert_eq!(
            parse_submission_response(html),
            SubmissionResult::Unknown("Some unexpected response from the server".to_string())
        );
    }
}
