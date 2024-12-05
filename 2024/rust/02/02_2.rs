use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_path = "input.txt";

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line_result in reader.lines() {
        let line = line_result?;

        if line.trim().is_empty() {
            continue;
        }

        let levels: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if is_safe_sequence(&levels) || check_with_removal(&levels) {
            safe_count += 1;
        }
    }

    println!("Number of safe reports: {}", safe_count);
    Ok(())
}

fn is_safe_sequence(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut is_increasing = None;
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if diff.abs() > 3 || diff.abs() == 0 {
            return false;
        }

        match is_increasing {
            None => {
                is_increasing = Some(diff > 0);
            }
            Some(true) => {
                if diff <= 0 {
                    return false;
                }
            }
            Some(false) => {
                if diff >= 0 {
                    return false;
                }
            }
        }
    }
    true
}

fn check_with_removal(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let mut reduced = levels.to_vec();
        reduced.remove(i);
        if is_safe_sequence(&reduced) {
            return true;
        }
    }
    false
}
