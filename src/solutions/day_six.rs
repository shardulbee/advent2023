use crate::utils::*;
use regex::Regex;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}
impl Race {
    fn get_winners(&self) -> Vec<usize> {
        let mut winners = Vec::new();

        for i in 1..=self.time {
            let distance_for_i = i * (self.time - i);
            if distance_for_i > self.distance {
                winners.push(i);
            }
        }

        winners
    }
}

pub fn run(test_mode: bool) {
    let lines = read_day_as_lines(6, test_mode);
    let times_str = lines.get(0).unwrap();
    let distances_str = lines.get(1).unwrap();

    let re = Regex::new(r"\d+").unwrap();

    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();

    re.find_iter(times_str)
        .for_each(|t| times.push(t.as_str().parse::<usize>().unwrap()));
    re.find_iter(distances_str)
        .for_each(|t| distances.push(t.as_str().parse::<usize>().unwrap()));

    let zipped = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect::<Vec<Race>>();

    // println!("Zipped: {:?}", zipped);

    // let parsed_times = captured_times.iter().map(|c| c.parse::<usize>())

    println!("Part one: {}", part_one(&zipped));
    println!("Part two: {}", part_two(times_str, distances_str));
}

fn part_one(races: &[Race]) -> usize {
    races.iter().map(|r| r.get_winners().len()).product()
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

    Race { time, distance }.get_winners().len()
}
