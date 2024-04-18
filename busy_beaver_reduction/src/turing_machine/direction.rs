#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub enum Direction {
    LEFT,
    RIGHT,
}

impl Direction {
    /// Gets the value (`u8`) associated to each direction:
    /// - `LEFT` = 0
    /// - `RIGHT` = 0
    pub fn value(&self) -> u8 {
        match *self {
            Direction::LEFT => 0,
            Direction::RIGHT => 1,
        }
    }

    /// Transforms the value given (`u8`) to a Direction:
    /// - `0` = LEFT
    /// - `1` = RIGHT
    /// - `_` = LEFT, by default
    pub fn transform(direction: u8) -> Self {
        // for any u8 other than 0 or 1, return LEFT,
        // but this match will not be reached
        match direction {
            0 => Direction::LEFT,
            1 => Direction::RIGHT,
            _ => Direction::LEFT,
        }
    }
}
