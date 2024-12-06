use regex::Regex;
use crate::part_one;

pub fn run(text: &String) {
    let do_regex = Regex::new(r"(don't\(\)|do\(\))").unwrap();

    let mut capture = true;
    let mut filter_text = String::new();
    let mut last_pos = 0;

    for mat in do_regex.find_iter(text) {
        let section = &text[last_pos..mat.start()];
        if capture {
            filter_text.push_str(section);
        }
        capture = mat.as_str() == "do()";
        last_pos = mat.end();
    }

    if capture {
        filter_text.push_str(&text[last_pos..]);
    }

    part_one::run(&filter_text);
}
