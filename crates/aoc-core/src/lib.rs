use anyhow::{Result, anyhow};
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
}
