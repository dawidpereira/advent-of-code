use std::collections::HashMap;

type Input = HashMap<bool, Vec<Vec<u32>>>;

pub fn parse(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let part1 = parts.next().expect("Missing first part");
    let part2 = parts.next().expect("Missing second part");

    // Parse the first part into a HashMap
    let map = part1
        .lines()
        .filter_map(|line| {
            let pair: Vec<&str> = line.split('|').collect();
            if pair.len() == 2 {
                let key: u32 = pair[0].parse().expect("Invalid number in first part");
                let val: u32 = pair[1].parse().expect("Invalid number in first part");
                Some((key, val))
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut map, (key, val)| {
            map.entry(key).or_insert_with(Vec::new).push(val);
            map
        });

    // Parse the second part into a vector of vectors
    let arrays: Vec<Vec<u32>> = part2
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.trim().parse().expect("Invalid number in second part"))
                .collect()
        })
        .collect();

    get_fix_order(map, arrays)
}

pub fn part1(input: &Input) -> u32 {
    input
        .get(&true)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|line| line[line.len() / 2])
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .get(&false)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|line| line[line.len() / 2])
        .sum()
}

fn get_fix_order(
    rules: HashMap<u32, Vec<u32>>,
    input: Vec<Vec<u32>>,
) -> HashMap<bool, Vec<Vec<u32>>> {
    let result: &mut HashMap<bool, Vec<Vec<u32>>> = &mut HashMap::new();
    let empty = &Vec::new();

    input.iter().for_each(|line| {
        let mut valid_line = Vec::new();
        let mut is_fixed = false;

        line.iter().for_each(|num| {
            let rule = rules.get(num).unwrap_or(empty);
            let valid = is_valid_order(rule, &valid_line);
            if valid {
                valid_line.push(*num);
                return;
            }

            if let Some(index) = find_first_index_of_any(&valid_line, rule) {
                valid_line.insert(index, *num);
                is_fixed = true;
            }
        });

        if !is_fixed {
            result
                .entry(true)
                .and_modify(|e| e.push(valid_line.clone()))
                .or_insert(vec![valid_line.clone()]);
        } else {
            result
                .entry(false)
                .and_modify(|e| e.push(valid_line.clone()))
                .or_insert(vec![valid_line.clone()]);
        }
    });
    result.clone()
}

fn is_valid_order(rule: &Vec<u32>, valid_line: &Vec<u32>) -> bool {
    let mut valid = true;
    for num in valid_line.iter() {
        valid = rule.iter().all(|r| r != num);
    }
    valid
}

fn find_first_index_of_any(vec1: &Vec<u32>, vec2: &Vec<u32>) -> Option<usize> {
    vec2.iter()
        .filter_map(|&num| vec1.iter().position(|&x| x == num))
        .min()
}
