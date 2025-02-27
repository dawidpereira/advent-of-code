use crate::util::conversions::FromChar;
use crate::util::direction::Direction;
use crate::util::grid::Grid;
use crate::util::point::Point;
use std::fmt::Debug;
use std::str::FromStr;

/// An iterator over a `Grid`, providing functionality for directional traversal.
///
/// The `GridIterator` struct allows for iterating over the elements of a `Grid` in a specified
/// direction. It supports both standard and diagonal movements across the grid, enabling
/// customized iteration strategies especially useful in grid-based algorithms or graphics.
///
/// # Type Parameters
/// - `'i`: The lifetime tied to the grid, ensuring that the grid is not dropped while the iterator
///   is in use.
/// - `T`: The type of elements contained within the grid.
///
/// # Fields
/// - `grid`: A reference to the grid being iterated over.
/// - `line_start`: The starting point for the current line of iteration.
/// - `current`: The current position of the iterator within the grid.
/// - `direction`: The direction in which the iterator is moving.
/// - `offset`: The step size or distance used during iteration.
/// - `can_change_axis`: Indicates whether the axis of movement can change (for diagonal movement).
/// - `have_next`: Indicates if further elements remain in the iteration.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct GridIterator<'i, T> {
    grid: &'i mut Grid<T>,
    line_start: Point,
    current: Point,
    direction: Direction,
    offset: i32,
    can_change_axis: bool,
    have_next: bool,
}

impl<'i, T> GridIterator<'i, T>
where
    T: Default + Clone + Debug + PartialEq,
    T: FromStr + FromChar,
    <T as FromStr>::Err: Debug,
    <T as FromChar>::Err: Debug,
{
    /// Constructs a new `GridIterator`.
    ///
    /// Initializes a `GridIterator` for a given grid, allowing navigation in the specified
    /// direction. The iterator is set up to use a given offset as a step size, making it possible
    /// to traverse the grid in flexible patterns.
    ///
    /// # Arguments
    /// * `grid` - A reference to the `Grid` struct that contains the data to iterate over.
    /// * `direction` - A reference to the `Direction` enum indicating the iteration's direction.
    /// * `offset` - An integer specifying the step size for each iteration movement.
    ///
    /// # Returns
    /// A newly constructed instance of `GridIterator`.
    ///
    /// # Panics
    /// Panics if the direction provided results in invalid initial configuration.
    pub fn new(grid: &'i mut Grid<T>, direction: &Direction, offset: i32) -> Self {
        let can_change_axis = direction.is_diagonal();
        let starting_point = grid.get_starting_point(direction);
        Self {
            grid,
            line_start: starting_point,
            current: starting_point,
            direction: *direction,
            offset,
            can_change_axis,
            have_next: true,
        }
    }

    /// Advances the iterator to the next point.
    ///
    /// This skips to the next valid point in its trajectory across the grid. If there is no valid
    /// next point or if the direction leads to a boundary exceedance, the function returns `Point::EMPTY`.
    ///
    /// # Arguments
    /// * `have_custom_path` - A boolean indicating whether a custom path is being used.
    ///
    /// # Returns
    /// * The next point, or `Point::EMPTY` if iteration cannot continue.
    ///
    /// # Panics
    /// Panics if an invalid direction leads to unwarranted behavior. Ensure directions are correct to prevent this.
    pub fn next<'n>(&mut self, wrap_enabled: bool) -> Point {
        if !self
            .grid
            .exced_bounds(&self.current, &self.direction, self.offset)
        {
            self.current = self.current.add(&self.direction.to_point());
            return self.current;
        }

        if !wrap_enabled {
            return self.brake();
        }

        match self.direction {
            Direction::Right => self.handle_one_direction(&Direction::Right, &Direction::Down),
            Direction::Down => self.handle_one_direction(&Direction::Down, &Direction::Right),
            Direction::Left => self.handle_one_direction(&Direction::Left, &Direction::Up),
            Direction::Up => self.handle_one_direction(&Direction::Up, &Direction::Left),
            Direction::RightDown => {
                self.handle_diagonal_direction(&Direction::Right, &Direction::Down)
            }
            Direction::LeftDown => {
                self.handle_diagonal_direction(&Direction::Left, &Direction::Down)
            }
            Direction::RightUp => self.handle_diagonal_direction(&Direction::Right, &Direction::Up),
            Direction::LeftUp => self.handle_diagonal_direction(&Direction::Left, &Direction::Up),
            Direction::Stop => self.brake(),
        }
    }

    /**
    Sets the current position of the iterator within the grid.

    This function updates the iterator's current position to the specified point. It ensures
    that the new position is within the bounds of the grid.

    # Arguments

    * `point` - A reference to the `Point` to which the iterator's position should be set.

    # Panics

    * The function will panic if the `point` is outside the bounds of the grid.
    */
    pub fn set_current_position(&mut self, point: &Point) {
        if point.x < 0 || point.y < 0 || point.x >= self.grid.width || point.y >= self.grid.height {
            panic!("Invalid point: {:?}. Bounds exceeded", point);
        }
        self.current = *point;
    }

    /// Sets the value at the current position of the iterator in the grid.
    ///
    /// This function updates the grid's value at the iterator's current position to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set at the current position in the grid.
    pub fn set_current_value(&mut self, value: T) {
        self.grid.set_value(&self.current, value);
    }

    /// Changes the direction of the iterator.
    ///
    /// Updates the iterator's direction to the specified direction. It also adjusts the ability
    /// to change axis based on whether the new direction is diagonal.
    ///
    /// # Arguments
    ///
    /// * `direction` - A reference to the new `Direction` for the iterator.
    pub fn change_direction(&mut self, direction: &Direction) {
        self.direction = *direction;
        self.can_change_axis = direction.is_diagonal();
        self.have_next = !self
            .grid
            .exced_bounds(&self.current, &self.direction, self.offset);
        self.line_start = self.current
    }

    /// Counts the number of points in the grid that satisfy a given condition.
    ///
    /// This function iterates over the grid and applies a specified condition
    /// to each point. It returns the total count of points that meet this condition.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure or function that defines the condition to be checked for each point.
    ///   It takes the following parameters:
    ///   - `&Grid<T>`: A reference to the grid.
    ///   - `&mut Point`: A mutable reference to the current point in the grid, allowing modifications.
    ///   - `Point`: A point representing the direction derived from the `direction` parameter.
    ///   - `i32`: An integer representing an offset, which can be used in calculations within the closure.
    ///
    /// * `direction` - The direction in which to iterate over the grid. This parameter
    ///   is used to determine the step size and direction of iteration.
    ///
    /// * `have_custom_path` - A boolean indicating whether a custom path is being used.
    ///
    /// # Returns
    ///
    /// * `i32` - The count of points that satisfy the condition specified by `f`.
    ///
    /// # Panics
    ///
    /// * The function will panic if the `direction` is invalid. Ensure that the `direction`
    ///   is valid to prevent this panic.
    pub fn count_with<F>(&mut self, f: F) -> i32
    where
        F: Fn(&Grid<T>, &Point, &Point, &i32) -> bool,
    {
        let original_position = self.current;
        let original_direction = self.direction;

        let step = self.direction.to_point();
        let mut count = 0;

        loop {
            if !self.have_next() {
                break;
            }

            let is_valid = f(self.grid, &self.current, &step, &self.offset);

            if is_valid {
                count += 1;
            }

            self.next(true);
        }

        self.restore(&original_position, &original_direction);

        count
    }

    ///
    /// Counts the number of points in the grid that satisfy a given condition.
    ///
    /// This function iterates over the grid and applies a specified condition to each point.
    /// # Arguments
    /// * `value` - The value to count in the grid.
    ///
    /// # Returns
    ///
    /// * i32 - The count of points that satisfy the condition specified by `f`.
    pub fn count(&mut self, value: &T) -> i32 {
        let original_position = self.current;
        let original_direction = self.direction;

        let mut count = 0;

        loop {
            if !self.have_next() {
                break;
            }

            let val = &self.get_current_value().unwrap_or_default();

            if val == value {
                count += 1
            }

            self.next(true);
        }

        self.restore(&original_position, &original_direction);

        count
    }

    /// Searches for a specific value in the grid that satisfies a given condition.
    ///
    /// This function iterates over the grid, applying a specified condition to each value
    /// to find a match. If a value meets the condition, it returns the value along with
    /// its position in the grid.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure or function that defines the condition to be checked for each value.
    ///   It takes the following parameters:
    ///   - `&Vec<T>`: A reference to a vector containing the values to find.
    ///   - `T`: The current value from the grid to be checked against the condition.
    /// * `find` - A vector of values to search for within the grid.
    ///
    /// # Returns
    ///
    /// * `Option<(T, Point)>` - Returns an `Option` containing a tuple of the found value and
    ///   its position as a `Point` if a match is found, or `None` if no match is found.
    ///
    /// # Panics
    ///
    /// * The function will panic if the current value in the grid cannot be unwrapped. Ensure
    ///   that the grid is properly initialized and contains valid values to prevent this panic.
    pub fn find<F>(&mut self, f: F, find: &Vec<T>) -> Option<(T, Point)>
    where
        F: Fn(&Vec<T>, T) -> bool,
        T: Clone, // Ensure T can be cloned
    {
        let original_position = self.current;
        let original_direction = self.direction;
        let mut result = None;

        loop {
            if !self.have_next() {
                break;
            }

            let current = self.get_current_position();
            let val = self.get_current_value().unwrap().clone(); // Clone the value
            if f(find, val.clone()) {
                result = Some((val, *current));
                break;
            }

            self.next(true);
        }

        self.restore(&original_position, &original_direction);
        result // Return the owned value
    }

    /// Checks if there is a next point available in the iteration.
    ///
    /// # Returns
    /// * `true` if there is a next point, `false` otherwise.
    pub fn have_next(&self) -> bool {
        self.have_next
    }

    /**
    Retrieves the current position of the iterator within the grid.

    This function returns the current position as a `Point`, indicating where
    the iterator is currently located in the grid.

    # Returns

    * A reference to the `Point` representing the current position of the iterator.
    */
    pub fn get_current_position(&self) -> &Point {
        &self.current
    }

    /// Retrieves the value at the current position of the iterator in the grid.
    ///
    /// This function returns the value located at the iterator's current position
    /// within the grid. It returns `None` if the position is invalid or out of bounds.
    ///
    /// # Returns
    ///
    /// * `Option<T>` - An `Option` containing the value at the current position if valid,
    ///   or `None` if the position is invalid.
    pub fn get_current_value(&self) -> Option<T> {
        self.grid.get_value(&self.current)
    }

    /**
    Retrieves the current direction of the iterator.

    # Returns

    * A reference to the current `Direction` of the iterator.
    */
    pub fn get_current_direction(&self) -> &Direction {
        &self.direction
    }

    /// Determines if the iterator can move in a specified direction without exceeding bounds.
    ///
    /// Evaluates whether moving in the given direction would keep the iterator within the grid's limits.
    ///
    /// # Arguments
    ///
    /// * `direction` - A reference to the `Direction` to check.
    ///
    /// # Returns
    ///
    /// * `true` if the iterator can move in the specified direction without exceeding bounds, `false` otherwise.
    pub fn can_move(&self, direction: &Direction) -> bool {
        !self
            .grid
            .exced_bounds(&self.current, direction, self.offset)
    }

    /// Determines the next point when moving in a linear direction.
    ///
    /// Alternates between primary and new line directions if the next step exceeds bounds.
    ///
    /// # Arguments
    /// * `next_step` - The primary direction of movement.
    /// * `new_line` - The fallback direction when primary movement exceeds bounds.
    ///
    /// # Returns
    /// * The next point in the iteration.
    fn handle_one_direction(&mut self, next_step: &Direction, new_line: &Direction) -> Point {
        if !self
            .grid
            .exced_bounds(&self.current, next_step, self.offset + 1)
        {
            self.current = self.current.add(&next_step.to_point());
            return self.current;
        }

        if self.grid.exced_bounds(&self.current, new_line, 1) {
            return self.brake();
        }

        self.line_start = self.line_start.add(&new_line.to_point());
        self.current = self.line_start;
        self.current
    }

    /// Determines the next point when moving diagonally.
    ///
    /// Supports diagonal movements by adjusting both axes simultaneously.
    ///
    /// # Arguments
    /// * `next_step_direction` - The initial direction of movement.
    /// * `new_line_direction` - An auxiliary direction if the primary step exceeds grid bounds.
    ///
    /// # Returns
    /// * The next point or `Point::EMPTY` if it cannot proceed.
    fn handle_diagonal_direction(
        &mut self,
        next_step_direction: &Direction,
        new_line_direction: &Direction,
    ) -> Point {
        if self.can_change_axis {
            let next_point = next_step_direction.to_point();
            if !self.grid.exced_bounds(
                &next_point.add(&self.line_start),
                next_step_direction,
                self.offset,
            ) {
                self.line_start = self.line_start.add(&next_point);
                self.current = self.line_start;
                return self.current;
            }

            self.line_start = self
                .grid
                .get_starting_point(&self.direction)
                .add(&new_line_direction.to_point());
            self.current = self.line_start;
            self.can_change_axis = false;
            return self.current;
        }

        let new_line_point = new_line_direction.to_point();
        // Check if next step exceeds the grid bounds and brake if it does
        if self.grid.exced_bounds(
            &new_line_point.add(&self.line_start),
            new_line_direction,
            self.offset - 1,
        ) {
            return self.brake();
        }
        self.line_start = self.line_start.add(&new_line_point);
        self.current = self.line_start;
        self.current
    }

    fn brake<'b>(&mut self) -> Point {
        self.have_next = false;
        Point::EMPTY
    }

    fn restore(&mut self, original_position: &Point, original_direction: &Direction) {
        self.current = *original_position;
        self.direction = *original_direction;
    }
}
