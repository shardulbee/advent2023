use crate::utils;
use regex::Regex;
use std::collections::HashMap;

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
