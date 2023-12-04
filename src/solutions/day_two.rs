use crate::utils;
use regex::Regex;

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

pub fn run(test_mode: bool) {
    let lines = utils::read_day_as_lines(2, test_mode);
    let games = Games(
        lines
            .iter()
            .map(|line| Game::parse_game(line))
            .collect::<Vec<Game>>(),
    );

    println!("Part one: {}", part_one(&games));
    println!("Part two: {}", part_two(&games));
}

fn part_one(games: &Games) -> i32 {
    games.possible_games().iter().sum()
}

fn part_two(games: &Games) -> i32 {
    games.0.iter().map(|game| game.power()).sum()
}
