use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let result = parse_and_calc_floor(input)?;
    Ok(format!("{result}"))
}

pub fn part2(input: &str) -> Result<String> {
    let result = parse_find_basement_pos(input)?;
    Ok(format!("{result}"))
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
}

fn floor_to_dir(c: char) -> Result<Dir> {
    match c {
        '(' => Ok(Dir::Up),
        ')' => Ok(Dir::Down),
        _ => Err(anyhow::anyhow!("invalid char: {c}")),
    }
}

fn parse_and_calc_floor(input: &str) -> Result<i32> {
    input.chars().try_fold(0, |acc, c| {
        let dir = floor_to_dir(c)?;
        match dir {
            Dir::Up => Ok(acc + 1),
            Dir::Down => Ok(acc - 1),
        }
    })
}

const BASEMENT: i32 = -1;

fn parse_find_basement_pos(input: &str) -> Result<usize> {
    let mut floor = 0;

    for (i, c) in input.chars().enumerate() {
        let dir = floor_to_dir(c)?;

        match dir {
            Dir::Up => floor += 1,
            Dir::Down => floor -= 1,
        }

        if floor == BASEMENT {
            return Ok(i + 1);
        }
    }

    Err(anyhow::anyhow!("basement not found"))
}

#[cfg(test)]
mod tests_part1 {
    use super::*;
    use std::assert_matches;

    #[test]
    fn test_floor_to_dir_should_be_up() {
        assert_matches!(floor_to_dir('('), Ok(Dir::Up));
    }

    #[test]
    fn test_floor_to_dir_should_be_down() {
        assert_matches!(floor_to_dir(')'), Ok(Dir::Down));
    }

    #[test]
    fn test_floor_to_dir_should_err() {
        assert!(floor_to_dir('?').is_err());
    }

    #[test]
    fn test_parse_and_calc_floor_should_work() {
        assert_eq!(parse_and_calc_floor("(())").unwrap(), 0);
        assert_eq!(parse_and_calc_floor("()()").unwrap(), 0);
        assert_eq!(parse_and_calc_floor("(((").unwrap(), 3);
        assert_eq!(parse_and_calc_floor("(()(()(").unwrap(), 3);
        assert_eq!(parse_and_calc_floor("))(((((").unwrap(), 3);
        assert_eq!(parse_and_calc_floor("())").unwrap(), -1);
        assert_eq!(parse_and_calc_floor("))(").unwrap(), -1);
        assert_eq!(parse_and_calc_floor(")))").unwrap(), -3);
        assert_eq!(parse_and_calc_floor(")())())").unwrap(), -3);
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test_parse_find_basement_pos_should_work() {
        assert_eq!(parse_find_basement_pos(")").unwrap(), 1);
        assert_eq!(parse_find_basement_pos("()())").unwrap(), 5);
    }

    #[test]
    fn test_parse_find_basement_pos_should_err() {
        assert!(parse_find_basement_pos("").is_err());
        assert!(parse_find_basement_pos("(").is_err());
    }
}
