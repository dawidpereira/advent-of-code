use crate::util::conversions::FromChar;
use crate::util::direction::Direction;
use crate::util::point::Point;
use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

/// A generic grid structure for managing data organized in a 2D grid format.
///
/// The `Grid<T>` struct represents a two-dimensional grid of items of type `T`. It provides
/// essential functionalities for grid-based operations such as calculating starting
/// points and checking boundary limits for grid traversal.
///
/// # Type Parameters
/// - `T`: The type of elements contained within the grid.
///
/// # Derivable Traits
/// - `Clone`: Allows creating a duplicate of the grid.
/// - `PartialEq`, `Eq`: Supports equality comparisons between grids.
/// - `Hash`: Allows the grid to be used in hashed collections like `HashMap`.
/// - `Debug`: Enables formatting for debugging.
///
/// # Fields
/// - `width`: The number of columns in the grid. This defines the horizontal size.
/// - `height`: The number of rows in the grid, dynamically determined by the number of elements.
/// - `data`: A vector containing the grid's elements, managed in a single contiguous memory block.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub data: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Default + Clone + Debug + PartialEq,
    T: FromStr + FromChar,
    <T as FromStr>::Err: Debug,
    <T as FromChar>::Err: Debug,
{
    pub fn new(data: Vec<Vec<T>>, width: i32) -> Self {
        Self {
            width,
            height: data.len() as i32,
            data,
        }
    }

    /// Determines the starting point for a search based on the provided direction.
    ///
    /// This function calculates an initial point within a grid, determined by the
    /// specified direction for the search operation. The starting point reflects
    /// common patterns needed to navigate typical data structures or graphical grids
    /// that employ directional traversal.
    ///
    /// # Directional Starting Points
    /// - **Right**, **Down**, **RightDown**: Start from the **top-left** corner at `(0, 0)`.
    /// - **Left**, **Up**, **LeftUp**: Start from the **bottom-right** corner, computed as `(width - 1, height - 1)`.
    /// - **RightUp**: Start from the **bottom-left** corner at `(0, height - 1)`.
    /// - **LeftDown**: Start from the **top-right** corner, computed as `(width - 1, 0)`.
    ///
    /// # Arguments
    /// * `direction` - A reference to a `Direction` enum, indicating the intended search direction.
    ///
    /// # Returns
    /// * A `Point` representing the calculated starting position in the form of `(x, y)`.
    pub fn get_starting_point(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Right | Direction::Down | Direction::RightDown => Point::new(0, 0),
            Direction::Left | Direction::Up | Direction::LeftUp => {
                Point::new(self.width - 1, self.height - 1)
            }
            Direction::RightUp => Point::new(0, self.height - 1),
            Direction::LeftDown => Point::new(self.width - 1, 0),
            _ => Point::new(0, 0), // Default case
        }
    }

    /// Determines if the current point exceeds the bounds of the grid based on the direction and chunk size.
    ///
    /// This function checks whether a given `Point` in the grid surpasses the allowable boundaries when extending
    /// a certain distance (`chunk_size`) in the specified `Direction`. This is useful for validating movements,
    /// ensuring that operations do not attempt to access grid locations that do not exist.
    ///
    /// # Boundary Conditions
    /// Directions describe how to extend the current point, and this function calculates:
    /// - **Right**: Extending beyond the right edge, where `current_point.x + chunk_size >= self.width`.
    /// - **Left**: Extending beyond the left edge, where `current_point.x - chunk_size < 0`.
    /// - **Up**: Extending beyond the top edge, where `current_point.y - chunk_size < 0`.
    /// - **Down**: Extending beyond the bottom edge, where `current_point.y + chunk_size >= self.height`.
    /// - **RightUp**: Exceeding either the right edge or the top edge.
    /// - **RightDown**: Exceeding either the right edge or the bottom edge.
    /// - **LeftUp**: Exceeding either the left edge or the top edge.
    /// - **LeftDown**: Exceeding either the left edge or the bottom edge.
    ///
    /// # Arguments
    /// * `current_point` - A reference to a `Point` representing the current position in the grid.
    /// * `direction` - A reference to a `Direction` enum, indicating the direction of movement from the current point.
    /// * `chunk_size` - An integer representing how far to extend from the current point in all specified directions.
    ///
    /// # Returns
    /// * `true` if extending from `current_point` in the specified `direction` by `chunk_size` results in exceeding the grid's boundaries.
    /// * `false` otherwise.
    ///
    /// # Panics
    /// Panics if given an invalid direction not covered by the defined `Direction` variants.
    pub fn exced_bounds(
        &self,
        current_point: &Point,
        direction: &Direction,
        chunk_size: i32,
    ) -> bool {
        let offset = chunk_size;
        match direction {
            Direction::Right => current_point.x + offset >= self.width,
            Direction::Left => current_point.x - offset < 0,
            Direction::Up => current_point.y - offset < 0,
            Direction::Down => current_point.y + offset >= self.height,
            Direction::RightUp => {
                current_point.x + offset >= self.width || current_point.y - offset < 0
            }
            Direction::RightDown => {
                current_point.x + offset >= self.width || current_point.y + offset >= self.height
            }
            Direction::LeftUp => current_point.x - offset < 0 || current_point.y - offset < 0,
            Direction::LeftDown => {
                current_point.x - offset < 0 || current_point.y + offset >= self.height
            }
            _ => panic!("Invalid direction {:?}", direction),
        }
    }

    /// Parses a string into a `Grid` struct.
    ///
    /// This function converts a string representation of grid data into a `Grid` object.
    /// It can handle parsing individual characters or segments of strings separated by
    /// a specified delimiter.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice containing the grid data.
    /// * `delimiter` - An optional character used to split each line into segments. If `None`,
    ///   each character is treated as a separate element.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn Error>>` - A `Result` containing the parsed `Grid` or an error if
    ///   parsing fails or if the input's width is inconsistent across lines.
    ///
    /// # Errors
    ///
    /// * Returns an error if the width of the grid is inconsistent across lines.
    /// * Returns an error if the conversion from a character or string segment to `T` fails.
    pub fn parse(input: &str, delimiter: Option<char>) -> Result<Self, Box<dyn Error>> {
        let mut data: Vec<Vec<T>> = Vec::new();
        let mut width = None;

        for line in input.lines() {
            let elements: Vec<T> = if let Some(delim) = delimiter {
                // Parse using delimiter
                line.split(delim)
                    .map(|s| {
                        T::from_str(s).map_err(|e| format!("Conversion error: {:?}", e).into())
                    })
                    .collect::<Result<Vec<T>, Box<dyn Error>>>()? // Specify the error type
            } else {
                // Parse character by character
                line.chars()
                    .map(|c| {
                        T::from_char(c).map_err(|e| format!("Conversion error: {:?}", e).into())
                    })
                    .collect::<Result<Vec<T>, Box<dyn Error>>>()? // Specify the error type
            };

            let tmp_width = elements.len() as i32;

            if width.is_none() {
                width = Some(tmp_width);
            }

            if tmp_width != width.unwrap() {
                return Err("Invalid input. Width is not consistent".into());
            }

            data.push(elements);
        }

        Ok(Grid::new(data, width.unwrap()))
    }

    /// Retrieves the value at the specified point in the grid.
    /// If the point is out of bounds, returns `None`.
    /// If the value is the default value of `T`, returns `None`.
    /// Otherwise, returns the value at the specified point.
    ///
    /// # Arguments
    /// * `point` - A reference to a `Point` representing the position in the grid.
    ///
    /// # Returns
    /// * An `Option<T>` containing the value at the specified point, or `None` if the point is out of bounds or the value is the default value of `T`.
    pub fn get_value(&self, point: &Point) -> Option<T> {
        let val = self.data[point.y as usize][point.x as usize].clone();
        if val == T::default() {
            return None;
        }
        Some(val)
    }

    /// Sets the value at the specified point in the grid.
    ///
    /// # Arguments
    /// * `point` - A reference to a `Point` representing the position in the grid.
    /// * `value` - The value to set at the specified point.
    pub fn set_value(&mut self, point: &Point, value: T) {
        self.data[point.y as usize][point.x as usize] = value;
    }

    /// Creates new grid with the same size filled by predefined value.
    ///
    /// # Aeguments
    /// * `value` - The value to set at all points
    ///
    pub fn same_size_with<U>(&self, value: U) -> Grid<U>
    where
        U: Default + Clone + Debug + PartialEq,
        U: FromStr + FromChar,
        <U as FromStr>::Err: Debug,
        <U as FromChar>::Err: Debug,
    {
        let data = vec![vec![value.clone(); self.width as usize]; self.height as usize];
        Grid::new(data, self.width)
    }

    /// Checks if the given point is within the grid boundaries.
    ///
    /// # Aeguments
    /// * `point` - A reference to a `Point` representing the position in the grid.
    ///
    pub fn contains(&self, point: &Point) -> bool {
        point.y >= 0 && point.x >= 0 && self.width > point.x && self.height > point.y
    }
}
