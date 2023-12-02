use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let re = Regex::new(r"(?<number>one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let lookup: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let lines = read_file_as_lines("../data/day1.data");
    let test_lines = read_file_as_lines("../data/day1_test.data");
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {:?}", part_two(&lines, re, lookup));
}

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

fn replace_all(s: String, re: &regex::Regex, lookup: &HashMap<&str, &str>) -> String {
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
        .map(|s| replace_all(s.to_string(), &re, &lookup))
        .collect::<Vec<String>>();
    part_one(&transformed)
}

fn read_file_as_lines(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}
