use crate::utils::*;
use regex::Regex;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}
impl Race {
    fn num_winners(&self) -> usize {
        (1..=self.time)
            .filter(|i| i * (self.time - i) > self.distance)
            .count()
    }
}

pub fn run(test_mode: bool) {
    let lines = read_day_as_lines(6, test_mode);
    let times_str = lines.get(0).unwrap();
    let distances_str = lines.get(1).unwrap();

    println!("Part one: {}", part_one(times_str, distances_str));
    println!("Part two: {}", part_two(times_str, distances_str));
}

fn part_one(times_str: &str, distances_str: &str) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let times = re
        .find_iter(times_str)
        .map(|t| t.as_str().parse::<usize>().unwrap());
    let distances = re
        .find_iter(distances_str)
        .map(|t| t.as_str().parse::<usize>().unwrap());
    let races = times
        .zip(distances)
        .map(|(t, d)| Race {
            time: t,
            distance: d,
        })
        .collect::<Vec<Race>>();

    races.iter().map(|r| r.num_winners()).product()
}

fn part_two(times_str: &str, distances_str: &str) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let time = re
        .find_iter(times_str)
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let distance = re
        .find_iter(distances_str)
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    Race { time, distance }.num_winners()
}
