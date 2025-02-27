use crate::util::direction::Direction;
use crate::util::grid::Grid;
use crate::util::grid_iterator::GridIterator;
use crate::util::point::Point;

type Input = Grid<char>;

pub fn parse(input: &str) -> Input {
    Grid::parse(input, None).expect("Failed to parse input into Grid<char>")
}

pub fn part1(input: &Input) -> i32 {
    let input: &mut Input = &mut input.clone();
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
            iterator.count_with(|grid, check_point, step, chunk_size| {
                search_part1(grid, check_point, step, chunk_size, &SEARCH_PATTERN)
            })
        })
        .sum()
}

pub fn part2(input: &Input) -> i32 {
    let input: &mut Input = &mut input.clone();
    let direction = &Direction::RightDown;
    let mut iterator = GridIterator::new(input, &direction, 3);
    iterator.count_with(|grid, check_point, step, chunk_size| {
        search_part2(grid, check_point, step, chunk_size)
    })
}

fn search_part1(
    input: &Input,
    check_point: &Point,
    step: &Point,
    chunk_size: &i32,
    search_pattern: &[char],
) -> bool {
    let mut xmas = true;
    let mut samx = true;
    let mut check_point = check_point.clone();

    for i in 0..*chunk_size {
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

fn search_part2(input: &Input, check_point: &Point, step: &Point, chunk_size: &i32) -> bool {
    const MAS: [char; 3] = ['M', 'A', 'S'];
    let pivot_step = &Point::LEFT_DOWN;
    let pivot_check_point = &check_point.clone().add(&Point::new(2, 0));

    let left = search_part1(input, check_point, step, chunk_size, &MAS);
    let right = search_part1(input, pivot_check_point, pivot_step, chunk_size, &MAS);
    left && right
}
