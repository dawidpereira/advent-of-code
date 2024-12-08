type Input = Vec<Vec<u32>>;

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|&levels| is_safe_sequence(levels))
        .count()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|&levels| is_safe_sequence(levels) || check_with_removal(levels))
        .count()
}

fn is_safe_sequence(levels: &[u32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut is_increasing = None;
    levels.windows(2).all(|window| {
        let diff = window[1].wrapping_sub(window[0]) as i32;
        let diff_abs = diff.abs();
        if diff_abs > 3 || diff_abs == 0 {
            return false;
        }

        match is_increasing {
            None => {
                is_increasing = Some(diff > 0);
                true
            }
            Some(true) => diff > 0,
            Some(false) => diff < 0,
        }
    })
}

fn check_with_removal(levels: &[u32]) -> bool {
    levels
        .iter()
        .enumerate()
        .any(|(i, _)| {
            let mut reduced = levels.to_vec();
            reduced.remove(i);
            is_safe_sequence(&reduced)
        })
}
