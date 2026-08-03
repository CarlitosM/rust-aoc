use anyhow::Result;

#[allow(clippy::unnecessary_wraps)]
pub fn part1(input: &str) -> Result<String> {
    let num = find_num(input.trim(), 5);
    Ok(num.to_string())
}

#[allow(clippy::unnecessary_wraps)]
pub fn part2(input: &str) -> Result<String> {
    let num = find_num(input.trim(), 6);
    Ok(num.to_string())
}

/// Finds the lowest non-negative integer that, when appended to the input,
/// produces an MD5 hex digest starting with the given prefix.
fn find_num(input: &str, num_zeros: usize) -> usize {
    for num in 0.. {
        let digest = md5::compute(format!("{input}{num}"));

        if has_leading_hex_zeros(&digest.0, num_zeros) {
            return num;
        }
    }

    unreachable!()
}

fn has_leading_hex_zeros(digest: &[u8; 16], num_zeros: usize) -> bool {
    let full_bytes = num_zeros / 2;

    if digest[..full_bytes].iter().any(|byte| *byte != 0) {
        return false;
    }

    if num_zeros % 2 == 1 {
        return digest[full_bytes] & 0xF0 == 0;
    }

    true
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    /// Tests `find_num` using the example input keys and expected results.
    #[test]
    fn test_find_num() {
        assert_eq!(find_num("abcdef", 5), 609043);
        assert_eq!(find_num("pqrstuv", 5), 1048970);
    }
}

#[cfg(test)]
mod tests_part2 {}
