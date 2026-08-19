use anyhow::Result;

#[allow(clippy::unnecessary_wraps)]
pub fn part1(input: &str) -> Result<String> {
    let result = parse_and_count_nice_strings(input, is_nice_string);
    Ok(result.to_string())
}

#[allow(clippy::unnecessary_wraps)]
pub fn part2(input: &str) -> Result<String> {
    let result = parse_and_count_nice_strings(input, is_actually_nice_string);
    Ok(result.to_string())
}

const NAUGHTY_STRINGS: &[&str] = &["ab", "cd", "pq", "xy"];
const VOWELS: &str = "aeiou";

fn is_long_enough(s: &str, len: usize) -> bool {
    s.len() >= len
}

fn no_naughty_strings(s: &str) -> bool {
    !NAUGHTY_STRINGS.iter().any(|naughty| s.contains(naughty))
}

fn rules_say_nice(s: &str) -> bool {
    let mut chars_iter = s.chars();
    let mut prev_char = chars_iter.next().unwrap_or(' ');
    let mut vowel_count: u32 = VOWELS.contains(prev_char).into();
    let mut double_letter = false;

    for c in chars_iter {
        if VOWELS.contains(c) {
            vowel_count += 1;
        }

        if prev_char == c {
            double_letter = true;
        }

        prev_char = c;
    }

    vowel_count >= 3 && double_letter
}

fn has_pair_twice(s: &str) -> bool {
    let bytes = s.as_bytes();
    (0..bytes.len().saturating_sub(1)).any(|i| {
        let pair = &bytes[i..i + 2];
        bytes[i + 2..].windows(2).any(|w| w == pair)
    })
}

fn has_repeat_with_gap(s: &str) -> bool {
    s.as_bytes().windows(3).any(|w| w[0] == w[2])
}

fn is_nice_string(s: &str) -> bool {
    is_long_enough(s, 3) && no_naughty_strings(s) && rules_say_nice(s)
}

fn is_actually_nice_string(s: &str) -> bool {
    has_pair_twice(s) && has_repeat_with_gap(s)
}

fn parse_and_count_nice_strings(input: &str, pred_fn: fn(&str) -> bool) -> usize {
    input.lines().filter(|s| pred_fn(s)).count()
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test_aoc_test_inputs() {
        assert!(is_nice_string("ugknbfddgicrmopn"));
        assert!(is_nice_string("aaa"));
        assert!(!is_nice_string("jchzalrnumimnmhp"));
        assert!(!is_nice_string("haegwjzuvuyypxyu"));
        assert!(!is_nice_string("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_parse_and_count_nice_strings() {
        assert_eq!(
            parse_and_count_nice_strings(
                "ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb",
                is_nice_string
            ),
            2
        );
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test_aoc_test_inputs() {
        assert!(is_actually_nice_string("qjhvhtzxzqqjkmpb"));
        assert!(is_actually_nice_string("xxyxx"));
        assert!(!is_actually_nice_string("uurcxstgmygtbstg"));
        assert!(!is_actually_nice_string("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_edge_cases() {
        assert!(!is_actually_nice_string("aaa"));
        assert!(is_actually_nice_string("aaaa"));
        assert!(is_actually_nice_string("xyxy"));
        assert!(!is_actually_nice_string("urrvucyrzzzooxhx"));
        assert!(is_actually_nice_string("bbbbtb"));
    }

    #[test]
    fn test_parse_and_count_actually_nice_strings() {
        assert_eq!(
            parse_and_count_nice_strings(
                "qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy",
                is_actually_nice_string
            ),
            2
        );
    }
}
