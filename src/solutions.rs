use crate::utils;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct DayOne;
impl DayOne {
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

        let calibration_values =
            lines_as_ints
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
            .map(|s| Self::replace_all(s, &re, &lookup))
            .collect::<Vec<String>>();
        Self::part_one(&transformed)
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
        println!("Part one: {}", Self::part_one(&lines));
        println!("Part two: {:?}", Self::part_two(&test_lines, re, lookup));
    }
}

struct Draw {
    num_blue: i32,
    num_red: i32,
    num_green: i32,
}
impl Draw {
    fn parse_draw(draw: &str) -> Draw {
        let re = Regex::new(r"(\d+)\s+(blue|red|green)").expect("Invalid regex");

        let mut num_blue = 0;
        let mut num_red = 0;
        let mut num_green = 0;

        for cap in re.captures_iter(draw) {
            let num = cap[1].parse::<i32>().unwrap();
            let color = &cap[2];
            match color {
                "blue" => num_blue = num,
                "red" => num_red = num,
                "green" => num_green = num,
                _ => (),
            }
        }
        Draw {
            num_blue,
            num_red,
            num_green,
        }
    }
}

pub struct Game {
    id: i32,
    draws: Vec<Draw>,
}

const MAX_NUM_RED: i32 = 12;
const MAX_NUM_GREEN: i32 = 13;
const MAX_NUM_BLUE: i32 = 14;

impl Game {
    pub fn parse_game(game: &str) -> Game {
        let first = game.split(':').collect::<Vec<&str>>();
        let id = first[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let draws = first[1];
        let draws = draws.split(';').collect::<Vec<&str>>();
        let draws = draws
            .iter()
            .map(|draw| Draw::parse_draw(draw))
            .collect::<Vec<Draw>>();
        Game { id, draws }
    }

    fn is_possible(&self) -> bool {
        self.draws.iter().all(|draw| {
            draw.num_blue <= MAX_NUM_BLUE
                && draw.num_red <= MAX_NUM_RED
                && draw.num_green <= MAX_NUM_GREEN
        })
    }

    pub fn power(&self) -> i32 {
        let max_blue = self
            .draws
            .iter()
            .map(|draw| draw.num_blue)
            .max()
            .unwrap_or(0);
        let max_red = self
            .draws
            .iter()
            .map(|draw| draw.num_red)
            .max()
            .unwrap_or(0);
        let max_green = self
            .draws
            .iter()
            .map(|draw| draw.num_green)
            .max()
            .unwrap_or(0);
        max_blue * max_red * max_green
    }
}

struct Games(Vec<Game>);
impl Games {
    fn possible_games(&self) -> Vec<i32> {
        let games = self
            .0
            .iter()
            .filter(|g| g.is_possible())
            .map(|game| game.id)
            .collect::<Vec<i32>>();
        games
    }
}

pub struct DayTwo;
impl DayTwo {
    pub fn run(test_mode: bool) {
        let lines = utils::read_day_as_lines(2, test_mode);
        let games = Games(
            lines
                .iter()
                .map(|line| Game::parse_game(line))
                .collect::<Vec<Game>>(),
        );

        println!("Part one: {}", Self::part_one(&games));
        println!("Part two: {}", Self::part_two(&games));
    }

    fn part_one(games: &Games) -> i32 {
        games.possible_games().iter().sum()
    }

    fn part_two(games: &Games) -> i32 {
        games.0.iter().map(|game| game.power()).sum()
    }
}

#[derive(Debug, PartialEq)]
enum SchematicCoordinate {
    Number(u32),
    Empty,
    Symbol(char),
}

#[derive(Debug, PartialEq)]
struct Coord(i32, i32);
struct Schematic(Vec<Vec<SchematicCoordinate>>);
impl Schematic {
    fn parse_schematic(lines: &[String]) -> Schematic {
        let out = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => SchematicCoordinate::Empty,
                        '0'..='9' => SchematicCoordinate::Number(c.to_digit(10).unwrap()),
                        _ => SchematicCoordinate::Symbol(c),
                    })
                    .collect::<Vec<SchematicCoordinate>>()
            })
            .collect::<Vec<Vec<SchematicCoordinate>>>();
        Schematic(out)
    }

    fn adjacent_coords(&self, coord: &Coord) -> Vec<Coord> {
        let (row, col) = (coord.0, coord.1);
        let hops = Vec::from([
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]);
        let possibles = hops.iter().map(|(r, c)| (row + r, col + c));
        possibles
            .filter(|(r, c)| {
                !(*r < 0 || *c < 0 || *r >= self.0.len() as i32 || *c >= self.0[0].len() as i32)
            })
            .map(|(r, c)| Coord(r, c))
            .collect::<Vec<Coord>>()
    }

    fn get_from_coord(&self, coord: &Coord) -> Option<&SchematicCoordinate> {
        let (row, col) = (coord.0, coord.1);
        if row < 0 || col < 0 || row >= self.0.len() as i32 || col >= self.0[0].len() as i32 {
            None
        } else {
            Some(&self.0[row as usize][col as usize])
        }
    }

    fn has_symbol_around(&self, row: usize, col: usize) -> bool {
        let adjacents = self.adjacent_coords(&Coord(row as i32, col as i32));
        adjacents.iter().any(|coord| {
            matches!(
                self.get_from_coord(coord),
                Some(SchematicCoordinate::Symbol(_))
            )
        })
    }

    fn get_number(&self, row: usize, col: usize) -> Option<u32> {
        match self.0[row][col] {
            SchematicCoordinate::Number(n) => {
                // scan left and right, collecting all digits until we hit a
                // symbol or empty space. Then return as a number
                let mut digits = Vec::from([n]);
                let mut col_i = col as i32 - 1;
                while col_i >= 0 {
                    match self.0[row][col_i as usize] {
                        SchematicCoordinate::Number(n) => {
                            digits.insert(0, n);
                            col_i -= 1;
                        }
                        _ => break,
                    }
                }
                let mut col_i = col + 1;
                while col_i < self.0[row].len() {
                    match self.0[row][col_i] {
                        SchematicCoordinate::Number(n) => {
                            digits.push(n);
                            col_i += 1;
                        }
                        _ => break,
                    }
                }
                Some(digits.iter().fold(0, |acc, n| acc * 10 + n))
            }
            _ => None,
        }
    }
}

pub struct DayThree;
impl DayThree {
    pub fn run(test_mode: bool) {
        let lines = utils::read_day_as_lines(3, test_mode);
        let schematic = Schematic::parse_schematic(&lines);
        println!("Part one: {}", Self::part_one(&schematic));
        println!("Part two: {}", Self::part_two(&schematic));
    }
    fn part_one(schematic: &Schematic) -> u32 {
        let mut row_idx = 0;
        let mut part_nums: Vec<u32> = Vec::new();
        while row_idx < schematic.0.len() {
            let mut col_idx = 0;
            while col_idx < schematic.0[row_idx].len() {
                match (
                    schematic.get_number(row_idx, col_idx),
                    schematic.has_symbol_around(row_idx, col_idx),
                ) {
                    (Some(n), true) => {
                        // println!("Found number {} which has symbol around", n);
                        part_nums.push(n);
                        while col_idx < schematic.0[row_idx].len()
                            && schematic.get_number(row_idx, col_idx).is_some()
                        {
                            col_idx += 1;
                        }
                    }
                    _ => col_idx += 1,
                }
            }
            row_idx += 1;
        }
        part_nums.iter().sum()
    }

    fn part_two(schematic: &Schematic) -> u32 {
        let mut sum = 0;
        schematic.0.iter().enumerate().for_each(|(row_idx, row)| {
            row.iter().enumerate().for_each(|(col_idx, col)| {
                if let SchematicCoordinate::Symbol('*') = col {
                    let coord = Coord(row_idx as i32, col_idx as i32);
                    let adjacent_coords = schematic.adjacent_coords(&coord);
                    let mut nums = adjacent_coords
                        .iter()
                        .filter(|coord| {
                            matches!(
                                schematic.get_from_coord(coord),
                                Some(SchematicCoordinate::Number(_))
                            )
                        })
                        .map(|coord| {
                            schematic
                                .get_number(coord.0 as usize, coord.1 as usize)
                                .unwrap()
                        })
                        .collect::<Vec<u32>>();
                    nums.sort();
                    nums.dedup();
                    if nums.len() == 2 {
                        sum += nums.iter().product::<u32>();
                    }
                }
            })
        });
        sum
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    chosen_numbers: HashSet<usize>,
}
impl Card {
    fn parse(s: &str) -> Card {
        let re = Regex::new(r"Card\s+(\d+):\s+((\d\s*)+)\s+\|\s+((\d\s*)+)").unwrap();
        let captures = re.captures(s).unwrap();
        let mut winning = HashSet::new();
        let mut chosen = HashSet::new();

        let id = captures[1].parse::<usize>().unwrap();
        let winning_numbers = captures[2]
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let chosen_numbers = captures[4]
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        winning.extend(winning_numbers);
        chosen.extend(chosen_numbers);

        Card {
            id,
            winning_numbers: winning,
            chosen_numbers: chosen,
        }
    }

    fn num_winning(&self) -> usize {
        self.winning_numbers
            .intersection(&self.chosen_numbers)
            .count()
    }

    fn score(&self) -> usize {
        let num_winning = self.num_winning();
        if num_winning == 0 {
            0
        } else {
            2usize.pow(num_winning as u32 - 1)
        }
    }
}

// type CardSet = HashMap<usize, Card>;
type CardCounts = HashMap<usize, usize>;

pub struct DayFour;
impl DayFour {
    pub fn run(test_mode: bool) {
        let lines = utils::read_day_as_lines(4, test_mode);
        let cards = lines
            .iter()
            .map(|line| Card::parse(line))
            .collect::<Vec<Card>>();
        println!("Part one: {}", Self::part_one(&cards));
        println!("Part two: {}", Self::part_two(&cards));
    }
    fn part_one(cards: &[Card]) -> usize {
        cards.iter().map(|c| c.score()).sum()
    }
    fn part_two(cards: &[Card]) -> usize {
        let mut counts = CardCounts::new();
        cards.iter().for_each(|card| {
            counts.insert(card.id, 1);
        });
        println!("Counts before: {:?}", counts);
        for card in cards.iter() {
            let count = *counts.get(&card.id).unwrap_or(&0);
            let num_winning = card.num_winning();
            println!("Card id: {}, num_winning: {}", card.id, num_winning);
            if num_winning > 0 {
                let range = (card.id + 1)..=(card.id + num_winning);
                range.for_each(|id| {
                    let current_count = counts.get(&id).unwrap_or(&0);
                    counts.insert(id, current_count + count);
                });
            }
        }
        println!("Counts after: {:?}", counts);
        counts.values().sum()
    }
}
