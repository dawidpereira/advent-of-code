use crate::util::direction::Direction;
use crate::util::point::Point;

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
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, width: i32) -> Self {
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
}

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
    grid: &'i Grid<T>,
    line_start: Point,
    current: Point,
    direction: Direction,
    offset: i32,
    can_change_axis: bool,
    have_next: bool,
}

impl<'i, T> GridIterator<'i, T> {
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
    pub fn new(grid: &'i Grid<T>, direction: &Direction, offset: i32) -> Self {
        let can_change_axis = direction.is_diagonal();
        let starting_point = grid.get_starting_point(&direction);
        Self {
            grid,
            line_start: starting_point.clone(),
            current: starting_point.clone(),
            direction: *direction,
            offset,
            can_change_axis,
            have_next: true,
        }
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
    /// # Returns
    ///
    /// * `i32` - The count of points that satisfy the condition specified by `f`.
    ///
    /// # Panics
    ///
    /// * The function will panic if the `direction` is invalid. Ensure that the `direction`
    ///   is valid to prevent this panic.
    pub fn count<F>(&mut self, f: F, direction: Direction) -> i32
    where
        F: Fn(&Grid<T>, &mut Point, Point, i32) -> bool,
    {
        let step = direction.to_point();

        let mut count = 0;

        loop {
            if !self.have_next() {
                break;
            }

            if f(
                self.grid,
                &mut self.get_current().clone(),
                step,
                self.offset
            ) {
                count += 1;
            }

            self.next();
        }

        count
    }

    /// Checks if there is a next point available in the iteration.
    ///
    /// # Returns
    /// * `true` if there is a next point, `false` otherwise.
    pub fn have_next(&self) -> bool {
        self.have_next
    }

    /// Retrieves the current point in the iteration.
    ///
    /// # Returns
    /// * The current point as a reference.
    pub(crate) fn get_current(&self) -> &Point {
        &self.current
    }

    /// Advances the iterator to the next point.
    ///
    /// This skips to the next valid point in its trajectory across the grid. If there is no valid
    /// next point or if the direction leads to a boundary exceedance, the function returns `Point::EMPTY`.
    ///
    /// # Returns
    /// * The next point, or `Point::EMPTY` if iteration cannot continue.
    ///
    /// # Panics
    /// Panics if an invalid direction leads to unwarranted behavior. Ensure directions are correct to prevent this.
    fn next<'n>(&mut self) -> Point {
        if !self
            .grid
            .exced_bounds(&self.current, &self.direction, self.offset)
        {
            self.current = self.current.add(&self.direction.to_point());
            return self.current;
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

            Direction::RightUp => {
                self.handle_diagonal_direction(&Direction::Right, &Direction::Up)
            }

            Direction::LeftUp => {
                self.handle_diagonal_direction(&Direction::Left, &Direction::Up)
            }

            _ => panic!("Invalid direction {:?}", self.direction),
        }
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
                &next_step_direction,
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
        if self
            .grid
            .exced_bounds(&new_line_point.add(&self.line_start), &new_line_direction, self.offset - 1)
        {
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

    fn get_starting_point(&self, direction: &Direction) -> Point {
        self.grid.get_starting_point(direction)
    }
}
