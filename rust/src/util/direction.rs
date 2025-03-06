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

pub const ORTHOGONAL: [Direction; 4] = [
    Direction::Right,
    Direction::Down,
    Direction::Left,
    Direction::Up,
];

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

    pub fn parse(c: char) -> Option<Self> {
        match c {
            'R' => Some(Direction::Right),
            'L' => Some(Direction::Left),
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Stop => Direction::Stop,
            _ => panic!("Invalid direction {:?}", self),
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Stop => Direction::Stop,
            _ => panic!("Invalid direction {:?}", self),
        }
    }
}
