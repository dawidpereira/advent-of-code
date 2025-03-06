use crate::util::{direction::ORTHOGONAL, grid::Grid, point::Point};

type Input = Grid<usize>;

pub fn parse(input: &str) -> Input {
    Grid::parse(input, None).expect("Failed to parse input into Grid<char>")
}

pub fn part1(grid: &Grid<usize>) -> u32 {
    solve(grid, false)
}

pub fn part2(grid: &Grid<usize>) -> u32 {
    solve(grid, true)
}

fn solve(grid: &Grid<usize>, distinct: bool) -> u32 {
    let mut result = 0;
    let mut seen = grid.same_size_with(-1);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid.get_value(&point).unwrap() == 9 {
                let id = y * grid.width + x;
                result += dfs(grid, distinct, &mut seen, id, point);
            }
        }
    }

    result
}

fn dfs(grid: &Grid<usize>, distinct: bool, seen: &mut Grid<i32>, id: i32, point: Point) -> u32 {
    let mut result = 0;

    for next in ORTHOGONAL.map(|direction| point.add(&direction.to_point())) {
        if grid.contains(&next)
            && grid.get_value(&next).unwrap() + 1 == grid.get_value(&point).unwrap()
            && (distinct || seen.get_value(&next).unwrap() != id)
        {
            seen.set_value(&next, id);

            if grid.get_value(&next).unwrap() == 0 {
                result += 1;
            } else {
                result += dfs(grid, distinct, seen, id, next);
            }
        }
    }

    result
}
