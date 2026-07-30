use anyhow::Result;

#[allow(clippy::unnecessary_wraps)]
pub fn part1(input: &str) -> Result<String> {
    let num = find_num(input.trim(), "00000");
    Ok(num.to_string())
}

#[allow(clippy::unnecessary_wraps)]
pub fn part2(input: &str) -> Result<String> {
    let num = find_num(input.trim(), "000000");
    Ok(num.to_string())
}

/// Finds the lowest non-negative integer that, when appended to the input,
/// produces an MD5 hex digest starting with the given prefix.
fn find_num(input: &str, starts_with: &str) -> usize {
    let mut result = String::new();
    let mut num: isize = -1;

    while !result.starts_with(starts_with) {
        num += 1;
        let digest = md5::compute(format!("{input}{num}"));
        result = format!("{digest:x}");
    }

    num.cast_unsigned()
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    /// Tests `find_num` using the example input keys and expected results.
    #[test]
    fn test_find_num() {
        assert_eq!(find_num("abcdef", "00000"), 609043);
        assert_eq!(find_num("pqrstuv", "00000"), 1048970);
    }
}

#[cfg(test)]
mod tests_part2 {}
