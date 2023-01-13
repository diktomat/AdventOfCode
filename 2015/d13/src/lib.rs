use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> i32 {
	let persons = parse(INPUT);
	arrange_happiness(&persons)
}

pub fn part2() -> i32 {
	let mut persons = parse(INPUT);
	persons.insert("self".to_string(), HashMap::new());
	arrange_happiness(&persons)
}

fn parse(input: &str) -> HashMap<String, HashMap<String, i32>> {
	let mut res: HashMap<String, HashMap<String, i32>> = HashMap::new();
	for line in input.lines() {
		let line = line.split_whitespace().collect::<Vec<_>>();
		if line.len() < 11 {
			panic!("Malformed input!");
		}

		let person = line[0].to_string();
		let sign = match line.get(2) {
			Some(&"gain") => 1,
			Some(&"lose") => -1,
			_ => panic!("Malformed input!"),
		};
		let happiness = line[3].parse::<i32>().unwrap() * sign;
		let neighbour = line[10].chars().filter(|c| c.is_alphabetic()).collect();

		res.entry(person).or_default().insert(neighbour, happiness);
	}
	res
}

fn arrange_happiness(persons: &HashMap<String, HashMap<String, i32>>) -> i32 {
	persons
		.iter()
		.permutations(persons.len())
		.fold(0, |acc, perm| {
			perm.iter()
				.enumerate()
				.fold(0, |happiness, (i, (_, neighbours))| {
					let left = (i as isize - 1).rem_euclid(perm.len() as isize) as usize;
					let right = (i + 1).rem_euclid(perm.len());
					happiness
						+ neighbours.get(perm[left].0).unwrap_or(&0)
						+ neighbours.get(perm[right].0).unwrap_or(&0)
				})
				.max(acc)
		})
}
