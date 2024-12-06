use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

mod part_one;
mod part_two;

fn main() -> io::Result<()> {
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: String = reader.lines()
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");

    println!("Part one:");
    part_one::run(&lines);

    println!("Part two:");
    part_two::run(&lines);

    Ok(())
}
