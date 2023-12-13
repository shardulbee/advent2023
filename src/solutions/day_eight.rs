use regex::Regex;
use std::collections::HashMap;

use crate::utils;

#[derive(Hash, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Node {
    source: String,
    left: String,
    right: String,
}
impl Node {
    fn parse(input: &str) -> Node {
        let re =
            Regex::new(r"(?<source>\w{3})\s+=\s+\((?<left>\w{3}),\s+(?<right>\w{3})\)").unwrap();
        let caps = re.captures(input).unwrap();
        Node {
            source: caps.name("source").unwrap().as_str().to_string(),
            left: caps.name("left").unwrap().as_str().to_string(),
            right: caps.name("right").unwrap().as_str().to_string(),
        }
    }
}

pub fn run(test: bool) {
    let read_day_as_string = &utils::read_day_as_string(8, test);
    let input = read_day_as_string.split("\n\n").collect::<Vec<&str>>();
    let left_right_instructions = &input[0]
        .chars()
        .map(|d| match d {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction!: {}", d),
        })
        .collect::<Vec<Direction>>();
    let nodes = &input[1].split('\n').map(Node::parse).collect::<Vec<Node>>();
    println!("Part 1: {}", part_one(left_right_instructions, nodes));
    println!("Part 2: {}", part_two(left_right_instructions, nodes));
}

fn part_one(left_right_instructions: &[Direction], nodes: &[Node]) -> usize {
    let mut source = "AAA";
    let dest = "ZZZ";
    let mut directions = left_right_instructions.iter().cycle();
    let mut num_steps = 0;

    let mut node_cache: HashMap<(&String, &Direction), &String> = HashMap::new();
    for node in nodes {
        node_cache.insert((&node.source, &Direction::Left), &node.left);
        node_cache.insert((&node.source, &Direction::Right), &node.right);
    }

    while source != dest {
        num_steps += 1;

        let new_source = *node_cache
            .get(&(&source.to_string(), directions.next().unwrap()))
            .unwrap();
        source = &new_source.as_str();
    }
    num_steps
}

fn part_two(left_right_instructions: &[Direction], nodes: &[Node]) -> usize {
    let sources = nodes.iter().filter(|n| n.source.ends_with('A'));
    let mut directions = left_right_instructions.iter().cycle();

    let mut node_cache: HashMap<(&String, &Direction), &String> = HashMap::new();
    for node in nodes {
        node_cache.insert((&node.source, &Direction::Left), &node.left);
        node_cache.insert((&node.source, &Direction::Right), &node.right);
    }

    let finals = sources
        .map(|n| {
            let mut source = &n.source;
            let mut num_steps = 0;
            while !source.ends_with('Z') {
                num_steps += 1;
                let new_source = *node_cache
                    .get(&(&source, directions.next().unwrap()))
                    .unwrap();
                source = &new_source;
            }
            num_steps
        })
        .collect::<Vec<usize>>();

    finals.iter().fold(finals[0], |acc, f| lcm(acc, *f))
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
