use crate::util::direction::Direction;
use crate::util::grid::{Grid, GridIterator};
use crate::util::point::Point;
use std::ops::Add;

type Input = Grid<Vec<char>>;

pub fn parse(input: &str) -> Input {
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut width: Option<i32> = None;

    for line in input.lines() {
        let tmp_width = line.len() as i32;

        if width.is_none() {
            width = Some(tmp_width);
        }

        if tmp_width != width.unwrap() {
            panic!("Invalid input. Width is not consistent");
        }

        let mut data_line = vec![' '; tmp_width as usize];

        let mut i = 0;
        for c in line.chars() {
            data_line[i] = c;
            i += 1;
        }
        data.push(data_line);
    }

    Grid::new(data, width.unwrap()) as Input
}

pub fn part1(input: &Input) -> i32 {
    const SEARCH_PATTERN: [char; 4] = ['X', 'M', 'A', 'S'];
    let directions = [
        Direction::Right,     //5
        Direction::Down,      //3
        Direction::RightDown, //5
        Direction::LeftDown,  //5
    ];
    directions
        .iter()
        .map(|direction| {
            let mut iterator = GridIterator::new(input, direction, 4);
            iterator.count(
                |grid, check_point, step, chunk_size| {
                    search_part1(
                        grid,
                        *check_point,
                        step,
                        chunk_size,
                        &SEARCH_PATTERN
                    )
                },
                *direction,
            )
        })
        .sum()
}

pub fn part2(input: &Input) -> i32 {
    let direction = Direction::RightDown;
    let mut iterator = GridIterator::new(input,  &direction, 3);
    iterator.count(
        |grid, check_point, step, chunk_size| {
            search_part2(
                grid,
                *check_point,
                step,
                chunk_size
            )
        },
        direction,
    )
}

fn search_part1(
    input: &Input,
    mut check_point: Point,
    step: Point,
    chunk_size: i32,
    search_pattern: &[char],
) -> bool {
    let mut xmas = true;
    let mut samx = true;

    for i in 0..chunk_size {
        let data = input.data[check_point.y as usize][check_point.x as usize];
        if data != search_pattern[i as usize] {
            xmas = false;
        }

        if data != search_pattern[(chunk_size - i - 1) as usize] {
            samx = false;
        }

        if !xmas && !samx {
            break;
        }

        check_point = check_point.add(&step);
    }

    xmas || samx
}

fn search_part2(
    input: &Input,
    mut check_point: Point,
    step: Point,
    chunk_size: i32,
) -> bool {
    const MAS: [char; 3] = ['M', 'A', 'S'];
    let pivot_step = Point::LEFT_DOWN;
    let pivot_check_point = check_point.clone().add(&Point::new(2, 0));

    let left = search_part1(input, check_point, step, chunk_size, &MAS);
    let right = search_part1(input, pivot_check_point, pivot_step, chunk_size, &MAS);
    left && right
}
