use std::collections::HashMap;

use itertools::Itertools;

pub fn part1() -> usize {
	let cities = parse_input();
	find_shortest_distance(&cities, 1)
}

pub fn part2() -> usize {
	let cities = parse_input();
	find_shortest_distance(&cities, -1)
}

struct Route {
	dest: String,
	len: isize,
}

fn parse_input() -> HashMap<String, Vec<Route>> {
	let mut cities: HashMap<String, Vec<Route>> = HashMap::new();

	include_str!("input.txt")
		.lines()
		.map(|line| {
			let line = line.split_whitespace().collect::<Vec<&str>>();
			(
				line[0].to_string(),
				Route {
					dest: line[2].to_string(),
					len: line[4].parse().unwrap(),
				},
			)
		})
		.for_each(|(city, route)| {
			cities.entry(route.dest.clone()).or_default();
			cities.entry(city).or_default().push(route);
		});

	cities
}

fn find_shortest_distance(cities: &HashMap<String, Vec<Route>>, comparator: isize) -> usize {
	let mut res = isize::MAX;

	'outer: for perm in cities.iter().permutations(cities.len()) {
		let mut distance = 0;
		for (from, dest) in perm.iter().tuple_windows() {
			if let Some(route) = from.1.iter().find(|route| route.dest == *dest.0) {
				distance += route.len;
			} else if let Some(route) = dest.1.iter().find(|route| route.dest == *from.0) {
				distance += route.len;
			} else {
				continue 'outer;
			}
		}
		res = res.min(distance * comparator);
	}

	res.unsigned_abs()
}
