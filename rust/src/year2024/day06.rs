use crate::util::direction::Direction;
use crate::util::grid::Grid;
use crate::util::grid_iterator::GridIterator;
use crate::util::point::Point;
use std::collections::{HashSet};
use std::vec;

type Input = Grid<char>;

pub fn parse(input: &str) -> Input {
    Grid::parse(input, None).expect("Failed to parse input into Grid<char>")
}

pub fn part1(input: &Input) -> i32 {
    let input: &mut Input = &mut input.clone();
    let binding = &mut input.clone();
    let mut iterator = initialize_iterator(binding);
    process_grid(&mut iterator, false, &mut input.clone()).0
}

pub fn part2(input: &Input) -> i32 {
    let input: &mut Input = &mut input.clone();
    let binding = &mut input.clone();
    let mut iterator = initialize_iterator(binding);
    process_grid(&mut iterator, true, &mut input.clone()).1
}

fn process_grid(
    iterator: &mut GridIterator<char>,
    should_count_loops: bool,
    input: &mut Input,
) -> (i32, i32) {
    let mut count = 0;
    let obstacle: &mut Vec<Point> = &mut Vec::new();
    let starting_point = iterator.get_current_position().clone();
    let starting_direction = iterator.get_current_direction().clone();

    loop {
        if !iterator.have_next() {
            break;
        }
        let data = iterator.get_current_value();
        match data {
            Some('.') => {
                iterator.set_current_value('X');
                count += 1;
            }
            Some('^') => {
                count += 1;
            }
            Some('X') => {}
            Some('#') => {
                turn_right(iterator);
            }
            _ => panic!("Invalid data {:?}", data),
        };

        if should_count_loops {
            count_loop(
                iterator.get_current_position(),
                input.clone(),
                starting_point.clone(),
                starting_direction.clone(),
                obstacle,
            );
        }

        iterator.next(false);
        continue;
    }

    let distinct_obstacles: HashSet<_> = obstacle.into_iter().collect();
    (count, distinct_obstacles.iter().count() as i32)
}

fn count_loop(
    current_position: &Point,
    input: Input,
    starting_point: Point,
    starting_direction: Direction,
    obstacles: &mut Vec<Point>,
) {
    let loop_input = &mut input.clone();
    let obstacles_map: &mut HashSet<(Point, Direction)> = &mut HashSet::new();
    let iterator = &mut GridIterator::new(loop_input, &starting_direction, 1);

    iterator.set_current_position(current_position);
    let current_value = iterator.get_current_value();
    if current_value == Some('^') || current_value == Some('#') {
        return;
    }
    let obstacle_candidate = iterator.get_current_position().clone();

    iterator.set_current_value('O');
    iterator.set_current_position(&starting_point);

    loop {
        let data = iterator.get_current_value();
        match data {
            Some('^') | Some('.') | Some('X') => {}
            Some('#') | Some('O') => {
                if !obstacles_map.insert((
                    iterator.get_current_position().clone(),
                    iterator.get_current_direction().clone(),
                )) {
                    obstacles.push(obstacle_candidate);
                    break;
                }
                turn_right(iterator);
            }
            _ => panic!("Invalid data {:?}", data),
        }

        iterator.next(false);
        if !iterator.have_next() {
            break;
        }
    }
}

fn turn_right(iterator: &mut GridIterator<char>) {
    let current_direction = iterator.get_current_direction();
    let new_position = iterator
        .get_current_position()
        .sub(&current_direction.to_point());
    let new_direction = current_direction.turn_right();
    iterator.set_current_position(&new_position);
    iterator.change_direction(&new_direction);
}

fn initialize_iterator(input: &mut Input) -> GridIterator<char> {
    let mut iterator = GridIterator::new(input, &Direction::Right, 1);
    let directions: Vec<char> = vec!['^', 'v', '<', '>'];

    let (val, position) = iterator.find(find, &directions).unwrap();
    let direction = Direction::parse(val).unwrap();

    iterator.set_current_position(&position);
    iterator.change_direction(&direction);
    iterator
}

fn find(vec: &Vec<char>, right: char) -> bool {
    vec.iter().any(|&c| c == right)
}
