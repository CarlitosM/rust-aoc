use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let result = parse_input_sum(input, calculate_wrapping_paper)?;
    Ok(result.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let result = parse_input_sum(input, calculate_total_ribbon)?;
    Ok(result.to_string())
}

/// Parses present dimensions from each line and sums the selected calculation.
fn parse_input_sum(input: &str, calc_fn: fn(u32, u32, u32) -> u32) -> Result<u32> {
    input.lines().try_fold(0, |acc, line| {
        let mut dims = line.split('x');
        let l = dims
            .next()
            .ok_or(anyhow::anyhow!("expected l"))?
            .parse::<u32>()?;
        let w = dims
            .next()
            .ok_or(anyhow::anyhow!("expected w"))?
            .parse::<u32>()?;
        let h = dims
            .next()
            .ok_or(anyhow::anyhow!("expected h"))?
            .parse::<u32>()?;
        Ok(acc + calc_fn(l, w, h))
    })
}

/// Returns the three dimensions sorted from smallest to largest.
fn sort_dims(l: u32, w: u32, h: u32) -> [u32; 3] {
    let mut sorted = [l, w, h];
    sorted.sort_unstable();
    sorted
}

/// Calculates the total wrapping paper needed for one present.
fn calculate_wrapping_paper(l: u32, w: u32, h: u32) -> u32 {
    let extra_paper = calculate_extra_paper(l, w, h);
    let area = calculate_area(l, w, h);
    area + extra_paper
}

/// Calculates the slack paper from the area of the smallest side.
fn calculate_extra_paper(l: u32, w: u32, h: u32) -> u32 {
    let sorted = sort_dims(l, w, h);
    sorted[0] * sorted[1]
}

/// Calculates the surface area of one present.
fn calculate_area(l: u32, w: u32, h: u32) -> u32 {
    2 * (l * w + w * h + h * l)
}

/// Calculates the ribbon needed to wrap around the smallest perimeter.
fn calculate_ribbon(l: u32, w: u32, h: u32) -> u32 {
    let sorted = sort_dims(l, w, h);
    2 * (sorted[0] + sorted[1])
}

/// Calculates the bow ribbon from the present's volume.
fn calculate_bow(l: u32, w: u32, h: u32) -> u32 {
    l * w * h
}

/// Calculates the total ribbon needed for one present.
fn calculate_total_ribbon(l: u32, w: u32, h: u32) -> u32 {
    calculate_ribbon(l, w, h) + calculate_bow(l, w, h)
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn calculate_required_wrapping_paper() {
        assert_eq!(calculate_wrapping_paper(2, 3, 4), 58);
        assert_eq!(calculate_wrapping_paper(1, 1, 10), 43);
        assert_eq!(calculate_wrapping_paper(20, 3, 11), 659);
        assert_eq!(calculate_wrapping_paper(17, 7, 7), 623);
        assert_eq!(calculate_wrapping_paper(1, 26, 1), 107);
        assert_eq!(calculate_wrapping_paper(4, 28, 28), 2128);
    }

    #[test]
    fn calculate_total_wrapping_paper() {
        assert_eq!(
            parse_input_sum(
                "2x3x4\n1x1x10\n20x3x11\n17x7x7\n1x26x1\n4x28x28",
                calculate_wrapping_paper,
            )
            .unwrap(),
            3618
        );
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test_calculate_ribbon() {
        assert_eq!(calculate_ribbon(2, 3, 4), 10);
        assert_eq!(calculate_ribbon(1, 1, 10), 4);
    }

    #[test]
    fn test_calculate_bow() {
        assert_eq!(calculate_bow(2, 3, 4), 24);
        assert_eq!(calculate_bow(1, 1, 10), 10);
    }

    #[test]
    fn test_calculate_total_ribbon() {
        assert_eq!(calculate_total_ribbon(2, 3, 4), 34);
        assert_eq!(calculate_total_ribbon(1, 1, 10), 14);
    }

    #[test]
    fn test_parse_input_sum_total_ribbon() {
        assert_eq!(
            parse_input_sum("2x3x4\n1x1x10", calculate_total_ribbon).unwrap(),
            48
        );
    }
}
