use std::collections::HashSet;

type Input = Vec<u32>;

pub fn parse(input: &str) -> Input {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut right = input.len() - 1;
    let mut disk_memory: Vec<u64> = vec![];
    let mut file_size = input[right];
    let mut file_value = right / 2;

    for (left, &block_size) in input.iter().enumerate() {
        if left >= right {
            for _ in 0..file_size {
                disk_memory.push(file_value as u64);
            }
            break;
        }
        if left % 2 == 0 {
            for _ in 0..block_size {
                disk_memory.push((left / 2) as u64);
            }
        } else {
            for _ in 0..block_size {
                if file_size == 0 && right >= 2 {
                    right = right.saturating_sub(2);
                    file_size = input[right];
                    file_value = right / 2;
                }
                disk_memory.push(file_value as u64);
                file_size = file_size.saturating_sub(1);
            }
        }
    }

    let mut result: u64 = 0;
    disk_memory.iter().enumerate().for_each(|(index, &value)| {
        result = result.saturating_add(value * index as u64);
    });
    result
}

pub fn part2(input: &Input) -> u64 {
    let mut right = input.len() - 1;
    let mut disk_memory: Vec<u64> = vec![];
    let mut file_size = input[right];
    let mut file_value = right / 2;
    let mut moved_file_indexes: HashSet<usize> = HashSet::new();

    for (left, &block_size) in input.iter().enumerate() {
        if left >= right {
            for _ in 0..file_size {
                disk_memory.push(file_value as u64);
            }
            break;
        }
        if moved_file_indexes.contains(&left) {
            disk_memory.resize(disk_memory.len() + input[left] as usize, 0);
            continue;
        }

        if left % 2 == 0 {
            for _ in 0..block_size {
                disk_memory.push((left / 2) as u64);
            }
        } else {
            let mut empty_block_size = block_size;

            while empty_block_size > 0 {
                for index in (0..=right).rev().step_by(2) {
                    if index <= left || empty_block_size == 0 {
                        disk_memory.resize(disk_memory.len() + empty_block_size as usize, 0);
                        empty_block_size = 0;
                        break;
                    }
                    if moved_file_indexes.contains(&index) {
                        if index == right {
                            right = right.saturating_sub(2);
                        }
                        continue;
                    }
                    file_size = input[index];
                    file_value = index / 2;
                    if file_size > empty_block_size {
                        continue;
                    }
                    moved_file_indexes.insert(index);

                    if index == right {
                        right = right.saturating_sub(2);
                    }
                    for _ in (0..file_size).rev() {
                        disk_memory.push(file_value as u64);
                        empty_block_size = empty_block_size.saturating_sub(1);
                    }
                    break;
                }
            }
        }
    }

    let mut result: u64 = 0;
    disk_memory.iter().enumerate().for_each(|(index, &value)| {
        result = result.saturating_add(value * index as u64);
    });
    result
}
