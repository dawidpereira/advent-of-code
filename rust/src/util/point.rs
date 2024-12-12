/// Represents a point in 2D space, designed for use with grid structures.
///
/// The `Point` struct encapsulates a coordinate in two-dimensional space with integer precision.
/// It provides multiple utility methods for vector operations such as addition, subtraction,
/// and directional checks.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Adds another `Point` to this `Point`.
    ///
    /// Performs component-wise addition with the given point and returns a new `Point`.
    ///
    /// # Arguments
    ///
    /// * `other` - The point to add to this one.
    ///
    /// # Returns
    ///
    /// A new `Point` that is the component-wise addition of this and the other point.
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Adds the x-coordinate of another `Point` to this `Point`.
    ///
    /// Only the x-coordinate is modified. The y-coordinate of this `Point` remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `other` - The point whose x-coordinate will be added.
    ///
    /// # Returns
    ///
    /// A new `Point` with the resulting x-coordinate.
    pub fn add_x(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y,
        }
    }

    /// Adds the y-coordinate of another `Point` to this `Point`.
    ///
    /// Only the y-coordinate is modified. The x-coordinate of this `Point` remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `other` - The point whose y-coordinate will be added.
    ///
    /// # Returns
    ///
    /// A new `Point` with the resulting y-coordinate.
    pub fn add_y(&self, other: &Self) -> Self {
        Self {
            x: self.x,
            y: self.y + other.y,
        }
    }

    /// Subtracts another `Point` from this `Point`.
    ///
    /// Performs component-wise subtraction with the given point and returns a new `Point`.
    ///
    /// # Arguments
    ///
    /// * `other` - The point to subtract from this one.
    ///
    /// # Returns
    ///
    /// A new `Point` that is the component-wise subtraction of this and the other point.
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Checks if a point is diagonal.
    ///
    /// A point is considered diagonal if both its x and y coordinates are non-zero.
    ///
    /// # Returns
    ///
    /// `true` if the point is diagonal; otherwise, `false`.
    pub fn is_diagonal(&self) -> bool {
        self.x != 0 && self.y != 0
    }

    pub const EMPTY: Self = Self { x: 0, y: 0 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const UP: Self = Self { x: 0, y: -1 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const RIGHT_DOWN: Self = Self { x: 1, y: 1 };
    pub const RIGHT_UP: Self = Self { x: 1, y: -1 };
    pub const LEFT_DOWN: Self = Self { x: -1, y: 1 };
    pub const LEFT_UP: Self = Self { x: -1, y: -1 };
}
