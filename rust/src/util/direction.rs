use crate::util::point::Point;

/// Describes the direction of the movement
/// Designed to be used with the Grid struct.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
    Stop,
}

impl Direction {
    /// Get the point for the direction
    /// # Returns
    /// * The point
    /// * If the direction is not valid, returns Point::EMPTY
    /// * If the direction is Stop, returns Point::EMPTY
    pub fn to_point(&self) -> Point {
        match self {
            Direction::Right => Point::RIGHT,
            Direction::Left => Point::LEFT,
            Direction::Up => Point::UP,
            Direction::Down => Point::DOWN,
            Direction::RightDown => Point::RIGHT_DOWN,
            Direction::RightUp => Point::RIGHT_UP,
            Direction::LeftDown => Point::LEFT_DOWN,
            Direction::LeftUp => Point::LEFT_UP,
            _ => Point::EMPTY,
        }
    }

    /// Check if the direction is diagonal
    /// # Returns
    /// * True if the direction is diagonal
    /// * False if the direction is not diagonal
    pub fn is_diagonal(&self) -> bool {
        match self {
            Direction::RightDown | Direction::RightUp | Direction::LeftDown | Direction::LeftUp => {
                true
            }
            _ => false,
        }
    }
}
