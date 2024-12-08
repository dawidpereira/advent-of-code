use regex::Regex;

type Input = str;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &Input) -> u32 {
    let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();

    re.captures_iter(&input).fold(0, |acc, cap| {
        let first_number: u32 = cap[1].parse().unwrap();
        let second_number: u32 = cap[2].parse().unwrap();
        acc + first_number * second_number
    })
}

pub fn part2(input: &Input) -> u32 {
    let re = Regex::new(r"(don't\(\)|do\(\))").unwrap();

    let mut capture = true;
    let mut filter_text = String::new();
    let mut last_pos = 0;

    for mat in re.find_iter(input) {
        let section = &input[last_pos..mat.start()];
        if capture {
            filter_text.push_str(section);
        }
        capture = mat.as_str() == "do()";
        last_pos = mat.end();
    }

    if capture {
        filter_text.push_str(&input[last_pos..]);
    }

    part1(&filter_text)
}
