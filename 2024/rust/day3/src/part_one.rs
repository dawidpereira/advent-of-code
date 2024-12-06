use regex::Regex;

pub fn run(text: &String) {
    let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();

    let mut sum = 0;
    for cap in re.captures_iter(&text) {
        let first_number: i32 = cap[1].parse().unwrap();
        let second_number: i32 = cap[2].parse().unwrap();
        sum += first_number * second_number;
    }

    println!("Result is: {}", sum);
}