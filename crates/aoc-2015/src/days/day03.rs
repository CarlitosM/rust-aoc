use std::collections::HashSet;

use anyhow::{Ok, Result};

pub fn part1(input: &str) -> Result<String> {
    let result = parse_and_track(input)?;
    Ok(result.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let result = parse_dual_track(input)?;
    Ok(result.to_string())
}

/// Represents a cardinal movement direction on a 2D grid.
#[derive(Debug)]
enum CardinalDir {
    North,
    West,
    South,
    East,
}

impl TryFrom<char> for CardinalDir {
    type Error = anyhow::Error;

    /// Converts a direction character (`^`, `<`, `v`, `>`) into a [`CardinalDir`].
    fn try_from(item: char) -> Result<Self> {
        match item {
            '^' => Ok(CardinalDir::North),
            '<' => Ok(CardinalDir::West),
            'v' => Ok(CardinalDir::South),
            '>' => Ok(CardinalDir::East),
            c => Err(anyhow::anyhow!("invalid direction: {c}")),
        }
    }
}

/// Represents a 2D coordinate position (x, y) on a Cartesian grid.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct CartesianPoint(i32, i32);

impl CartesianPoint {
    /// Creates a new point at the origin `(0, 0)`.
    fn new() -> Self {
        CartesianPoint(0, 0)
    }

    /// Computes a new point by moving one step from `from` in the given `dir`.
    fn move_relative(from: &CartesianPoint, dir: &CardinalDir) -> CartesianPoint {
        let x = from.0;
        let y = from.1;

        match dir {
            CardinalDir::North => CartesianPoint(x, y + 1),
            CardinalDir::West => CartesianPoint(x - 1, y),
            CardinalDir::South => CartesianPoint(x, y - 1),
            CardinalDir::East => CartesianPoint(x + 1, y),
        }
    }
}

/// Tracks the current position and all visited unique positions for a single mover.
struct Tracker {
    current: CartesianPoint,
    unique_points: HashSet<CartesianPoint>,
}

impl Tracker {
    /// Initializes a tracker starting at the origin `(0, 0)` with the starting position recorded.
    fn start() -> Self {
        let current = CartesianPoint::new();
        let mut unique_points = HashSet::new();
        unique_points.insert(current.clone());

        Tracker {
            current,
            unique_points,
        }
    }

    /// Moves the tracker one step in the specified direction and records the new position.
    fn track(&mut self, dir: &CardinalDir) {
        let new_point = CartesianPoint::move_relative(&self.current, dir);
        self.current = new_point.clone();
        self.unique_points.insert(new_point);
    }
}

/// Parses input directions and tracks unique positions visited by a single mover.
fn parse_and_track(input: &str) -> Result<usize> {
    let tracks = input
        .chars()
        .try_fold(Tracker::start(), |mut t, input_dir| {
            let dir = CardinalDir::try_from(input_dir)?;
            t.track(&dir);
            Ok(t)
        })?;

    Ok(tracks.unique_points.len())
}

/// Indicates which tracker moved last in dual tracking mode.
enum LastTracked {
    A,
    B,
}

/// Tracks positions visited by two movers taking alternating turns.
struct DualTracker {
    tracker_a: Tracker,
    tracker_b: Tracker,
    last: LastTracked,
}

impl DualTracker {
    /// Initializes a dual tracker with two independent trackers starting at the origin.
    fn start() -> Self {
        DualTracker {
            tracker_a: Tracker::start(),
            tracker_b: Tracker::start(),
            last: LastTracked::B,
        }
    }

    /// Moves the next tracker in sequence one step in the specified direction.
    fn track(&mut self, dir: &CardinalDir) {
        match self.last {
            LastTracked::A => {
                self.tracker_b.track(dir);
                self.last = LastTracked::B;
            }
            LastTracked::B => {
                self.tracker_a.track(dir);
                self.last = LastTracked::A;
            }
        }
    }

    /// Returns the total count of unique points visited by either tracker.
    fn total_unique(&self) -> usize {
        self.tracker_a
            .unique_points
            .union(&self.tracker_b.unique_points)
            .count()
    }
}

/// Parses input directions and tracks unique positions visited by two alternating movers.
fn parse_dual_track(input: &str) -> Result<usize> {
    let tracks = input
        .chars()
        .try_fold(DualTracker::start(), |mut tracker, input_dir| {
            let dir = CardinalDir::try_from(input_dir)?;
            tracker.track(&dir);
            Ok(tracker)
        })?;

    Ok(tracks.total_unique())
}

#[cfg(test)]
mod tests_part1 {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_dir_from_char() {
        assert_matches!(CardinalDir::try_from('^').unwrap(), CardinalDir::North);
        assert_matches!(CardinalDir::try_from('<').unwrap(), CardinalDir::West);
        assert_matches!(CardinalDir::try_from('v').unwrap(), CardinalDir::South);
        assert_matches!(CardinalDir::try_from('>').unwrap(), CardinalDir::East);
    }

    #[test]
    fn test_err_from_char() {
        assert!(CardinalDir::try_from('x').is_err());
        assert!(CardinalDir::try_from('1').is_err());
        assert!(CardinalDir::try_from(' ').is_err());
    }

    #[test]
    fn test_move_point() {
        assert_matches!(
            CartesianPoint::move_relative(&CartesianPoint(0, 0), &CardinalDir::North),
            CartesianPoint(0, 1)
        );

        assert_matches!(
            CartesianPoint::move_relative(&CartesianPoint(0, 0), &CardinalDir::West),
            CartesianPoint(-1, 0)
        );

        assert_matches!(
            CartesianPoint::move_relative(&CartesianPoint(0, 0), &CardinalDir::South),
            CartesianPoint(0, -1)
        );

        assert_matches!(
            CartesianPoint::move_relative(&CartesianPoint(0, 0), &CardinalDir::East),
            CartesianPoint(1, 0)
        );
    }

    #[test]
    fn test_tracker_start() {
        let tracker = Tracker::start();

        assert_matches!(tracker.current, CartesianPoint(0, 0));
        assert_eq!(tracker.unique_points.len(), 1);
    }

    #[test]
    fn test_tracker_track() {
        let mut tracker = Tracker::start();

        tracker.track(&CardinalDir::North);
        assert_matches!(tracker.current, CartesianPoint(0, 1));
        assert_eq!(tracker.unique_points.len(), 2);

        tracker.track(&CardinalDir::West);
        assert_matches!(tracker.current, CartesianPoint(-1, 1));
        assert_eq!(tracker.unique_points.len(), 3);

        tracker.track(&CardinalDir::South);
        assert_matches!(tracker.current, CartesianPoint(-1, 0));
        assert_eq!(tracker.unique_points.len(), 4);

        tracker.track(&CardinalDir::East);
        assert_matches!(tracker.current, CartesianPoint(0, 0));
        assert_eq!(tracker.unique_points.len(), 4);

        tracker.track(&CardinalDir::North);
        assert_matches!(tracker.current, CartesianPoint(0, 1));
        assert_eq!(tracker.unique_points.len(), 4);

        tracker.track(&CardinalDir::North);
        assert_matches!(tracker.current, CartesianPoint(0, 2));
        assert_eq!(tracker.unique_points.len(), 5);
    }

    #[test]
    fn test_aoc_test_input() {
        assert_eq!(parse_and_track(">").unwrap(), 2);
        assert_eq!(parse_and_track("^>v<").unwrap(), 4);
        assert_eq!(parse_and_track("^v^v^v^v^v").unwrap(), 2);
    }
}

#[cfg(test)]
mod tests_part2 {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_dual_tracker() {
        let mut tracker = DualTracker::start();

        tracker.track(&CardinalDir::North);

        assert_matches!(tracker.tracker_a.current, CartesianPoint(0, 1));
        assert_eq!(tracker.tracker_a.unique_points.len(), 2);

        assert_matches!(tracker.tracker_b.current, CartesianPoint(0, 0));
        assert_eq!(tracker.tracker_b.unique_points.len(), 1);

        assert_eq!(tracker.total_unique(), 2);

        tracker.track(&CardinalDir::South);

        assert_matches!(tracker.tracker_a.current, CartesianPoint(0, 1));
        assert_eq!(tracker.tracker_a.unique_points.len(), 2);

        assert_matches!(tracker.tracker_b.current, CartesianPoint(0, -1));
        assert_eq!(tracker.tracker_b.unique_points.len(), 2);

        assert_eq!(tracker.total_unique(), 3);

        tracker.track(&CardinalDir::East);

        assert_matches!(tracker.tracker_a.current, CartesianPoint(1, 1));
        assert_eq!(tracker.tracker_a.unique_points.len(), 3);

        assert_matches!(tracker.tracker_b.current, CartesianPoint(0, -1));
        assert_eq!(tracker.tracker_b.unique_points.len(), 2);

        assert_eq!(tracker.total_unique(), 4);

        tracker.track(&CardinalDir::West);

        assert_matches!(tracker.tracker_a.current, CartesianPoint(1, 1));
        assert_eq!(tracker.tracker_a.unique_points.len(), 3);

        assert_matches!(tracker.tracker_b.current, CartesianPoint(-1, -1));
        assert_eq!(tracker.tracker_b.unique_points.len(), 3);

        assert_eq!(tracker.total_unique(), 5);

        tracker.track(&CardinalDir::West);

        assert_matches!(tracker.tracker_a.current, CartesianPoint(0, 1));
        assert_eq!(tracker.tracker_a.unique_points.len(), 3);

        assert_matches!(tracker.tracker_b.current, CartesianPoint(-1, -1));
        assert_eq!(tracker.tracker_b.unique_points.len(), 3);

        assert_eq!(tracker.total_unique(), 5);

        tracker.track(&CardinalDir::East);

        assert_matches!(tracker.tracker_a.current, CartesianPoint(0, 1));
        assert_eq!(tracker.tracker_a.unique_points.len(), 3);

        assert_matches!(tracker.tracker_b.current, CartesianPoint(0, -1));
        assert_eq!(tracker.tracker_b.unique_points.len(), 3);

        assert_eq!(tracker.total_unique(), 5);
    }

    #[test]
    fn test_aoc_test_input() {
        assert_eq!(parse_dual_track("^v").unwrap(), 3);
        assert_eq!(parse_dual_track("^>v<").unwrap(), 3);
        assert_eq!(parse_dual_track("^v^v^v^v^v").unwrap(), 11);
    }
}
