use crate::utils;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Range(usize, usize);
impl Range {
    fn intersect(&self, other: &Range) -> Option<Range> {
        let disjoint = (self.0 < other.0 && self.1 <= other.0) || (self.0 >= other.1);
        let self_contains_other = self.0 <= other.0 && self.1 >= other.1;
        let other_contains_self = other.0 <= self.0 && self.1 <= other.1;

        if disjoint {
            None
        } else if self_contains_other {
            Some(other.clone())
        } else if other_contains_self {
            Some(self.clone())
        } else if self.0 < other.0 {
            Some(Range(other.0, self.1))
        } else {
            Some(Range(self.0, other.1))
        }
    }
}

#[derive(Debug)]
struct Mapping {
    source_range: Range,
    destination_range: Range,
}
impl Mapping {
    fn get_destination(&self, source: usize) -> Option<usize> {
        match source >= self.source_range.0 && source < self.source_range.1 {
            true => {
                let source_offset = source - self.source_range.0;
                Some(source_offset + self.destination_range.0)
            }
            false => None,
        }
    }

    fn parse(input: &str) -> Mapping {
        let parts = input.split(' ').collect::<Vec<&str>>();
        assert_eq!(parts.len(), 3);

        let source_range_start = parts[1].parse::<usize>().unwrap();
        let destination_range_start = parts[0].parse::<usize>().unwrap();
        let range_length = parts[2].parse::<usize>().unwrap();

        Mapping {
            source_range: Range(source_range_start, source_range_start + range_length),
            destination_range: Range(
                destination_range_start,
                destination_range_start + range_length,
            ),
        }
    }
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}
impl Map {
    fn parse(input: &str) -> Map {
        let mut lines = input.split('\n').collect::<VecDeque<&str>>();
        lines.pop_front();
        let mappings = lines
            .iter()
            .map(|l| Mapping::parse(l))
            .collect::<Vec<Mapping>>();
        Map { mappings }
    }

    fn map(&self, source: usize) -> usize {
        for mapping in &self.mappings {
            let destination = mapping.get_destination(source);
            match destination {
                None => continue,
                Some(destination) => return destination,
            }
        }
        source
    }

    fn map_range(&self, range: &Range) -> Vec<Range> {
        let mut result = Vec::new();
        let mut to_map = Vec::from([range.clone()]);
        while let Some(range) = to_map.pop() {
            let mut mapped = false;
            for mapping in &self.mappings {
                let intersection = mapping.source_range.intersect(&range);
                match intersection {
                    None => continue,
                    Some(intersection) => {
                        println!(
                            "Found intersection between {:?} and {:?}: {:?}",
                            intersection, mapping.source_range, range
                        );
                        let destination = mapping.get_destination(intersection.0).unwrap();
                        let length = intersection.1 - intersection.0;
                        result.push(Range(destination, destination + length));
                        mapped = true;

                        if range.0 < mapping.source_range.0 {
                            let new_left_range = Range(range.0, mapping.source_range.0);
                            println!("Adding new left range: {:?}", new_left_range);
                            to_map.push(new_left_range);
                        }
                        if range.1 > mapping.source_range.1 {
                            let new_right_range = Range(mapping.source_range.1, range.1);
                            println!("Adding new right range: {:?}", new_right_range);
                            to_map.push(new_right_range);
                        }
                        break;
                    }
                }
            }

            if !mapped {
                result.push(range.clone());
            }
        }
        result
    }
}

pub fn run(test_mode: bool) {
    let input = utils::read_day_as_string(5, test_mode);
    let mut parts = input
        .split("\n\n")
        .map(|p| p.trim())
        .collect::<VecDeque<&str>>();

    let seed_line = parts.pop_front().unwrap();
    let seeds_as_strs = seed_line.split(": ").collect::<Vec<&str>>()[1];
    let seeds = seeds_as_strs
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let maps = parts.iter().map(|p| Map::parse(p)).collect::<Vec<Map>>();

    println!("Part one: {}", part_one(&seeds, &maps));
    println!("Part two: {}", part_two(seeds_as_strs, &maps));
}

fn part_one(seeds: &[usize], maps: &[Map]) -> usize {
    seeds
        .iter()
        .map(|s| {
            let mut result = *s;
            for map in maps {
                result = map.map(result);
            }
            result
        })
        .min()
        .unwrap()
}

fn part_two(seeds_as_strs: &str, maps: &[Map]) -> usize {
    let re = Regex::new(r"((?<range_start>\d+) (?<range_length>\d+))+").unwrap();

    let mut ranges = re
        .captures_iter(seeds_as_strs)
        .map(|c| {
            let range_start = c
                .name("range_start")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let range_length = c
                .name("range_length")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            Range(range_start, range_start + range_length)
        })
        .collect::<Vec<Range>>();

    for map in maps {
        ranges = ranges
            .iter()
            .flat_map(|r| map.map_range(r))
            .collect::<Vec<Range>>();
    }
    ranges.iter().map(|r| r.0).min().unwrap_or(0)
}
