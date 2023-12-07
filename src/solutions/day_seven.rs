use crate::utils;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, convert::TryInto};

type Cards = [Card; 5];

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}
impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'J' => Self::Jack,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl HandType {
    fn from_cards(cards: &Cards) -> Self {
        let mut map: HashMap<Card, usize> = HashMap::new();
        cards.iter().fold(&mut map, |acc, c| {
            let current_count = *(acc.get(c).unwrap_or(&0));
            acc.insert(*c, current_count + 1);
            acc
        });
        let counts = map.values().copied().collect::<Vec<usize>>();
        if counts.contains(&5) {
            Self::FiveOfAKind
        } else if counts.contains(&4) {
            Self::FourOfAKind
        } else if counts.contains(&3) && counts.contains(&2) {
            Self::FullHouse
        } else if counts.contains(&3) {
            Self::ThreeOfAKind
        } else if counts.contains(&2) && counts.len() == 3 {
            Self::TwoPair
        } else if counts.contains(&2) {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }

    fn from_cards_alternate_rules(cards: &Cards) -> Self {
        let num_jokers = cards.iter().filter(|c| **c == Card::Jack).count();
        match num_jokers > 0 {
            true => {
                let without_jokers = Self::from_cards(cards);
                match without_jokers {
                    Self::FiveOfAKind | Self::FourOfAKind | Self::FullHouse => Self::FiveOfAKind,
                    Self::ThreeOfAKind => Self::FourOfAKind,
                    Self::HighCard => Self::OnePair,
                    Self::TwoPair => {
                        if num_jokers == 1 {
                            Self::FullHouse
                        } else {
                            Self::FourOfAKind
                        }
                    }
                    Self::OnePair => Self::ThreeOfAKind,
                }
            }
            false => Self::from_cards(cards),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Hand {
    bid: usize,
    cards: [Card; 5],
    alternate_rules: bool,
}
impl Hand {
    fn hand_type(&self) -> HandType {
        match self.alternate_rules {
            true => HandType::from_cards_alternate_rules(&self.cards),
            false => HandType::from_cards(&self.cards),
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                let self_cards = self.cards;
                let other_cards = other.cards;
                for i in 0..5 {
                    match self_cards[i].cmp(&other_cards[i]) {
                        Ordering::Equal => continue,
                        o => return o,
                    }
                }
                Ordering::Equal
            }
            o => o,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(test_mode: bool) {
    let lines = utils::read_day_as_lines(7, test_mode);
    println!("Day 7");
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));
}

pub fn part_one(lines: &[String]) -> usize {
    let re = Regex::new(r"(?<hand>\w+)\s+(?<bid>\d+)").unwrap();
    let mut hands = lines
        .iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let cards: Cards = caps
                .name("hand")
                .unwrap()
                .as_str()
                .chars()
                .map(Card::from_char)
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();
            let bid = caps.name("bid").unwrap().as_str().parse::<usize>().unwrap();
            Hand {
                bid,
                cards,
                alternate_rules: false,
            }
        })
        .collect::<Vec<Hand>>();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}

pub fn part_two(lines: &[String]) -> usize {
    let re = Regex::new(r"(?<hand>\w+)\s+(?<bid>\d+)").unwrap();
    let mut hands = lines
        .iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let cards: Cards = caps
                .name("hand")
                .unwrap()
                .as_str()
                .chars()
                .map(Card::from_char)
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();
            let bid = caps.name("bid").unwrap().as_str().parse::<usize>().unwrap();
            Hand {
                bid,
                cards,
                alternate_rules: true,
            }
        })
        .collect::<Vec<Hand>>();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}
