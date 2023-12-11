use regex::Regex;

use crate::utils;

fn compute_difference_sequence(input_sequence: &[i32]) -> Vec<i32> {
	let differences = input_sequence.iter().enumerate().map(|(i, x)| {
		if i == 0 { return None };

		Some(x - input_sequence[i - 1])
	}).filter(Option::is_some).map(Option::unwrap).collect::<Vec<i32>>();
	differences
}

fn all_zeroes(sequence: &[i32]) -> bool {
	sequence.iter().all(|x| *x == 0)
}

fn until_zeroes(input_sequence: &Vec<i32>) -> Vec<Vec<i32>> {
	let mut sequences = Vec::new();
	let mut cur_sequence = input_sequence;
	sequences.push(cur_sequence.to_vec());
	while !all_zeroes(&cur_sequence) {
		let differences = compute_difference_sequence(&cur_sequence);
		sequences.push(differences.to_vec());
		cur_sequence = &sequences[sequences.len() - 1];
	}
	sequences
}

fn next_element(input_sequence: &Vec<i32>) -> i32 {
	let sequences = until_zeroes(input_sequence);

	sequences.iter().rfold(0, |acc, x| {
		acc + x[x.len() - 1]
	})
}

fn previous_element(input_sequence: &Vec<i32>) -> i32 {
	let sequences = until_zeroes(input_sequence);

	sequences.iter().rfold(0, |acc, x| {
		x[0] - acc
	})
}


fn part_one(inputs: &Vec<Vec<i32>>) -> i32 {
	inputs.iter().map(|seq| next_element(seq)).sum()
}

fn part_two(inputs: &Vec<Vec<i32>>) -> i32 {
	inputs.iter().map(|seq| previous_element(seq)).sum()
}

pub fn run(test: bool) {
	let lines = utils::read_day_as_lines(9, test);
	let re = Regex::new(r"(\-*\d+)").unwrap();
	let inputs = lines.iter().map(|line| {
		re.captures_iter(line).map(|cap| {
			cap[1].parse::<i32>().unwrap()
		}).collect::<Vec<i32>>()
	}).collect::<Vec<Vec<i32>>>();

	println!("Part one: {}", part_one(&inputs));
	println!("Part two: {}", part_two(&inputs));
} 
