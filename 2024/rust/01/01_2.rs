use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let path = "01.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut left_nums = Vec::new();
    let mut right_nums = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((left, right)) = split_line(&line) {
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<i32>(), right.parse::<i32>()) {
                left_nums.push(left_num);
                right_nums.entry(right_num)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    left_nums.sort();

    let mut sum = 0;
    for left in left_nums.iter() {
        let mut right = 0;
        if let Some(e) = right_nums.get(left) {
            right = *e;
        }

        let tmp = left * right;
        sum += tmp.abs();
    }

    println!("Result is: {}", sum);
    Ok(())
}

fn split_line(line: &str) -> Option<(&str, &str)> {
    let mut parts = line.split_whitespace();
    let left = parts.next()?;
    let right = parts.next()?;
    Some((left, right))
}
