use crate::utils;
use regex::Regex;
use std::collections::HashMap;

fn part_one(lines: &[String]) -> i32 {
    let mut lines_as_ints: Vec<Vec<i32>> = Vec::new();
    lines.iter().for_each(|line| {
        let mut ints: Vec<i32> = Vec::new();
        line.split("").for_each(|c| match c.parse::<i32>() {
            Ok(i) => ints.push(i),
            Err(_e) => (),
        });
        lines_as_ints.push(ints);
    });

    let calibration_values = lines_as_ints
        .iter()
        .map(|ints| match (ints.first(), ints.last()) {
            (Some(first), Some(last)) => 10 * first + last,
            _ => 0,
        });
    calibration_values.sum()
}

fn replace_all(s: &String, re: &regex::Regex, lookup: &HashMap<&str, &str>) -> String {
    let mut news = s.to_string();
    loop {
        match re.find(&news) {
            Some(m) => {
                let replacement_value = *lookup.get(m.as_str()).unwrap();
                news = re.replacen(&news, 1, replacement_value).to_string();
            }
            None => {
                break news;
            }
        }
    }
}

fn part_two(lines: &[String], re: regex::Regex, lookup: HashMap<&str, &str>) -> i32 {
    let transformed = lines
        .iter()
        .map(|s| replace_all(s, &re, &lookup))
        .collect::<Vec<String>>();
    part_one(&transformed)
}

pub fn run(test_mode: bool) {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let lookup: HashMap<&str, &str> = HashMap::from([
        ("one", "1e"),
        ("two", "2o"),
        ("three", "3e"),
        ("four", "4r"),
        ("five", "5e"),
        ("six", "6x"),
        ("seven", "7n"),
        ("eight", "8t"),
        ("nine", "9e"),
    ]);

    let lines = utils::read_day_as_lines(1, test_mode);
    let test_lines = utils::read_day_as_lines(1, test_mode);
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {:?}", part_two(&test_lines, re, lookup));
}
