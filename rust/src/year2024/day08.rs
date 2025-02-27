use std::collections::HashMap;

use crate::util::{direction::Direction, grid::Grid, grid_iterator::GridIterator, point::Point};

type Input = (Grid<char>, HashMap<char, Vec<Point>>);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input, None).expect("Failed to parse input into Grid<char>");
    let mut antennas = HashMap::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            let frequency = grid.get_value(&point).unwrap();

            if frequency != '.' {
                antennas
                    .entry(frequency)
                    .or_insert_with(Vec::new)
                    .push(point);
            }
        }
    }
    (grid, antennas)
}

pub fn part1(input: &Input) -> i64 {
    let (grid, antennas) = input;
    let mut locations = grid.same_size_with(0);

    for frequrency in antennas.values() {
        for &first in frequrency {
            for &second in frequrency {
                if first != second {
                    let distance = second.sub(&first);
                    let antinode = second.add(&distance);

                    if grid.contains(&antinode) {
                        locations.set_value(&antinode, 1);
                    }
                }
            }
        }
    }

    let mut iterator = GridIterator::new(&mut locations, &Direction::Right, 1);
    iterator.count(&1) as i64
}

pub fn part2(input: &Input) -> i64 {
    let (grid, antennas) = input;
    let mut locations = grid.same_size_with(0);

    for frequrency in antennas.values() {
        for &first in frequrency {
            for &second in frequrency {
                if first != second {
                    let distance = second.sub(&first);
                    let mut antinode = second;

                    while grid.contains(&antinode) {
                        locations.set_value(&antinode, 1);
                        antinode = antinode.add(&distance);
                    }
                }
            }
        }
    }

    let mut iterator = GridIterator::new(&mut locations, &Direction::Right, 1);
    iterator.count(&1) as i64
}
