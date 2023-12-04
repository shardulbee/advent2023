use crate::utils;
use regex::Regex;
use std::collections::{HashMap, HashSet};

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

type CardCounts = HashMap<usize, usize>;
pub fn run(test_mode: bool) {
    let lines = utils::read_day_as_lines(4, test_mode);
    let cards = lines
        .iter()
        .map(|line| Card::parse(line))
        .collect::<Vec<Card>>();
    println!("Part one: {}", part_one(&cards));
    println!("Part two: {}", part_two(&cards));
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
