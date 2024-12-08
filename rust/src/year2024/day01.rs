use std::collections::HashMap;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();
    for line in input.lines() {
        if let Some((left, right)) = split_lines(line) {
            left_nums.push(left.parse().unwrap());
            right_nums.push(right.parse().unwrap());
        }
    }

    left_nums.sort();
    right_nums.sort();

    (left_nums, right_nums)
}

pub fn part1(input: &Input) -> u32 {
    let (left_nums, right_nums) = input.clone();

    left_nums.iter()
        .zip(right_nums.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    let (left_nums, right_nums) = input.clone();
    let mut right_map = HashMap::new();

    right_nums.iter().for_each(|&right_num| {
        right_map.entry(right_num)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    left_nums.iter()
        .filter_map(|left| right_map.get(left).map(|e| left * *e))
        .sum()
}

fn split_lines(line: &str) -> Option<(&str, &str)> {
    let mut parts = line.split_whitespace();
    let left = parts.next()?;
    let right = parts.next()?;
    Some((left, right))
}