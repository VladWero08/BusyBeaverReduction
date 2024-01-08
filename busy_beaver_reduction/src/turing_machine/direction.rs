#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn value(&self) -> u8 {
        match *self {
            Direction::LEFT => 0,
            Direction::RIGHT => 1,
        }
    }

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
