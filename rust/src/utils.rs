use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_day_as_lines(day: u32, is_test: bool) -> Vec<String> {
    let filename = match is_test {
        true => format!(
            "/Users/shardul/src/github.com/shardulbee/advent2023/data/day{:1}_test.data",
            day
        ),
        false => format!(
            "/Users/shardul/src/github.com/shardulbee/advent2023/data/day{:1}.data",
            day
        ),
    };
    //print the filename
    // println!("Reading file: {}", filename);
    read_file_as_lines(&filename)
}

pub fn read_file_as_lines(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}
