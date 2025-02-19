use std::collections::HashMap;

type Input = HashMap<i64, Vec<i64>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(':').and_then(|(k, v)| {
                k.trim().parse().ok().map(|key| {
                    (
                        key,
                        v.split_whitespace()
                            .filter_map(|x| x.parse().ok())
                            .collect(),
                    )
                })
            })
        })
        .collect()
}

pub fn part1(input: &Input) -> i64 {
    input
        .iter()
        .map(|v| {
            if is_valid(v.1, v.1.len() - 1, *v.0, false) {
                *v.0
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &Input) -> i64 {
    input
        .iter()
        .map(|v| {
            if is_valid(v.1, v.1.len() - 1, *v.0, true) {
                *v.0
            } else {
                0
            }
        })
        .sum()
}

fn is_valid(values: &Vec<i64>, index: usize, expected: i64, concat: bool) -> bool {
    if index == 0 {
        return expected == values[index];
    }

    let val = values[index];

    (concat
        && expected % power_of_ten(val) == val
        && is_valid(values, index - 1, expected / power_of_ten(val), concat))
        || (expected % val == 0 && is_valid(values, index - 1, expected / val, concat))
        || (expected >= val && is_valid(values, index - 1, expected - val, concat))
}

fn power_of_ten(val: i64) -> i64 {
    if val < 10 {
        10
    } else if val < 100 {
        100
    } else {
        1000
    }
}
