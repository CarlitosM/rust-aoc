use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let result = run_grid::<LightState>(input)?;
    Ok(result.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let result = run_grid::<usize>(input)?;
    Ok(result.to_string())
}

const LIGHT_GRID_COLS: usize = 1000;
const LIGHT_GRID_ROWS: usize = 1000;
const LIGHT_GRID_SIZE: usize = LIGHT_GRID_COLS * LIGHT_GRID_ROWS;

const LIGHT_ON: &str = "turn on";
const LIGHT_OFF: &str = "turn off";
const LIGHT_TOGGLE: &str = "toggle";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightAction {
    TurnOn,
    TurnOff,
    Toggle,
}

pub trait LightCell: Copy + Default {
    fn apply(&mut self, action: LightAction);
    fn brightness(&self) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightState {
    #[default]
    Off,
    On,
}

impl LightCell for LightState {
    fn apply(&mut self, action: LightAction) {
        *self = match action {
            LightAction::TurnOn => Self::On,
            LightAction::TurnOff => Self::Off,
            LightAction::Toggle => match *self {
                Self::On => Self::Off,
                Self::Off => Self::On,
            },
        };
    }

    fn brightness(&self) -> usize {
        match *self {
            Self::On => 1,
            Self::Off => 0,
        }
    }
}

impl LightCell for usize {
    fn apply(&mut self, action: LightAction) {
        match action {
            LightAction::TurnOn => *self += 1,
            LightAction::TurnOff => *self = self.saturating_sub(1),
            LightAction::Toggle => *self += 2,
        }
    }

    fn brightness(&self) -> usize {
        *self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub col: usize,
    pub row: usize,
}

impl TryFrom<&str> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (col, row) = value
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Invalid cell: {value}"))?;

        Ok(Self {
            col: col.parse()?,
            row: row.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangeUpdate {
    pub start: Cell,
    pub end: Cell,
    pub action: LightAction,
}

impl TryFrom<&str> for RangeUpdate {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (idx, action) = if let Some(idx) = line.find(LIGHT_ON) {
            (idx + LIGHT_ON.len(), LightAction::TurnOn)
        } else if let Some(idx) = line.find(LIGHT_OFF) {
            (idx + LIGHT_OFF.len(), LightAction::TurnOff)
        } else if let Some(idx) = line.find(LIGHT_TOGGLE) {
            (idx + LIGHT_TOGGLE.len(), LightAction::Toggle)
        } else {
            return Err(anyhow::anyhow!("Invalid light switch: {line}"));
        };

        let (start, end) = line[idx..]
            .trim()
            .split_once(" through ")
            .ok_or_else(|| anyhow::anyhow!("Could not get start and end cell: {line}"))?;

        Ok(Self {
            start: Cell::try_from(start)?,
            end: Cell::try_from(end)?,
            action,
        })
    }
}

pub struct LightGrid<T: LightCell> {
    grid: Box<[T; LIGHT_GRID_SIZE]>,
}

impl<T: LightCell> Default for LightGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: LightCell> LightGrid<T> {
    #[must_use]
    pub fn new() -> Self {
        let boxed = vec![T::default(); LIGHT_GRID_SIZE].into_boxed_slice();
        let grid = boxed
            .try_into()
            .unwrap_or_else(|_| panic!("Vector length mismatch"));
        Self { grid }
    }

    pub fn update_from_range(&mut self, update: &RangeUpdate) {
        let min_row = update.start.row.min(update.end.row);
        let max_row = update.start.row.max(update.end.row);
        let min_col = update.start.col.min(update.end.col);
        let max_col = update.start.col.max(update.end.col);

        for row in min_row..=max_row {
            let row_offset = row * LIGHT_GRID_COLS;
            for col in min_col..=max_col {
                self.grid[row_offset + col].apply(update.action);
            }
        }
    }

    #[must_use]
    pub fn total_brightness(&self) -> usize {
        self.grid.iter().map(LightCell::brightness).sum()
    }
}

fn run_grid<T: LightCell>(input: &str) -> Result<usize> {
    let mut grid = LightGrid::<T>::new();
    for line in input.lines() {
        let update = RangeUpdate::try_from(line)?;
        grid.update_from_range(&update);
    }
    Ok(grid.total_brightness())
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test_aoc_test_inputs_1() {
        let mut grid = LightGrid::<LightState>::new();
        grid.update_from_range(&RangeUpdate {
            start: Cell { row: 0, col: 0 },
            end: Cell { row: 999, col: 999 },
            action: LightAction::TurnOn,
        });
        assert_eq!(grid.total_brightness(), 1000 * 1000);
    }

    #[test]
    fn test_aoc_test_inputs_2() {
        let mut grid = LightGrid::<LightState>::new();
        grid.update_from_range(&RangeUpdate {
            start: Cell { row: 0, col: 0 },
            end: Cell { row: 0, col: 999 },
            action: LightAction::Toggle,
        });
        assert_eq!(grid.total_brightness(), 1000);
    }

    #[test]
    fn test_aoc_test_inputs_3() {
        let mut grid = LightGrid::<LightState>::new();
        grid.update_from_range(&RangeUpdate {
            start: Cell { row: 0, col: 0 },
            end: Cell { row: 999, col: 999 },
            action: LightAction::TurnOn,
        });
        grid.update_from_range(&RangeUpdate {
            start: Cell { row: 0, col: 0 },
            end: Cell { row: 0, col: 999 },
            action: LightAction::Toggle,
        });
        assert_eq!(grid.total_brightness(), (1000 * 1000) - 1000);
    }

    #[test]
    fn test_aoc_test_inputs_4() {
        let mut grid = LightGrid::<LightState>::new();
        grid.update_from_range(&RangeUpdate {
            start: Cell { row: 0, col: 0 },
            end: Cell { row: 999, col: 999 },
            action: LightAction::TurnOn,
        });
        grid.update_from_range(&RangeUpdate {
            start: Cell { row: 499, col: 499 },
            end: Cell { row: 500, col: 500 },
            action: LightAction::TurnOff,
        });
        assert_eq!(grid.total_brightness(), 1000 * 1000 - 4);
    }

    #[test]
    fn test_aoc_test_inputs_parse_1() -> Result<()> {
        let parsed = RangeUpdate::try_from("turn on 0,0 through 999,999")?;
        assert_eq!(parsed.action, LightAction::TurnOn);
        assert_eq!(parsed.start, Cell { col: 0, row: 0 });
        assert_eq!(parsed.end, Cell { col: 999, row: 999 });
        Ok(())
    }

    #[test]
    fn test_aoc_test_inputs_parse_2() -> Result<()> {
        let parsed = RangeUpdate::try_from("toggle 0,0 through 999,0")?;
        assert_eq!(parsed.action, LightAction::Toggle);
        assert_eq!(parsed.start, Cell { col: 0, row: 0 });
        assert_eq!(parsed.end, Cell { col: 999, row: 0 });
        Ok(())
    }

    #[test]
    fn test_aoc_test_inputs_parse_3() -> Result<()> {
        let parsed = RangeUpdate::try_from("turn off 499,499 through 500,500")?;
        assert_eq!(parsed.action, LightAction::TurnOff);
        assert_eq!(parsed.start, Cell { col: 499, row: 499 });
        assert_eq!(parsed.end, Cell { col: 500, row: 500 });
        Ok(())
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test_part2_turn_on_one_light() {
        let mut grid = LightGrid::<usize>::new();
        grid.update_from_range(&RangeUpdate {
            start: Cell { col: 0, row: 0 },
            end: Cell { col: 0, row: 0 },
            action: LightAction::TurnOn,
        });
        assert_eq!(grid.total_brightness(), 1);
    }

    #[test]
    fn test_part2_toggle_all_lights() {
        let mut grid = LightGrid::<usize>::new();
        grid.update_from_range(&RangeUpdate {
            start: Cell { col: 0, row: 0 },
            end: Cell { col: 999, row: 999 },
            action: LightAction::Toggle,
        });
        assert_eq!(grid.total_brightness(), 2_000_000);
    }
}
