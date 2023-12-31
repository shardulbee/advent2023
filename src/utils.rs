use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn read_day_as_lines(day: u32, is_test: bool) -> Vec<String> {
    let filename = match is_test {
        true => format!("./data/day{:1}_test.data", day),
        false => format!("./data/day{:1}.data", day),
    };
    read_file_as_lines(&filename)
}
pub fn read_day_as_string(day: u32, is_test: bool) -> String {
    let filename = match is_test {
        true => format!("./data/day{:1}_test.data", day),
        false => format!("./data/day{:1}.data", day),
    };
    read_file_as_string(&filename)
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

fn read_file_as_string(filename: &str) -> String {
    let mut contents = String::new();
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let _ = reader.read_to_string(&mut contents);

    contents
}
