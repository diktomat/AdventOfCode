use itertools::Itertools;

pub fn part1() -> usize {
	include_str!("input.txt")
		.lines()
		.map(|present| {
			let sides = present
				.split("x")
				.map(|side| side.parse::<usize>().unwrap())
				.tuple_combinations()
				.map(|(x, y)| 2 * x * y);
			sides.clone().sum::<usize>() + sides.min().unwrap() / 2
		})
		.sum()
}

pub fn part2() -> usize {
	include_str!("input.txt")
		.lines()
		.map(|present| {
			let present = present
				.split("x")
				.map(|side| side.parse::<usize>().unwrap());
			let wrap = present
				.clone()
				.tuple_combinations()
				.map(|(x, y)| 2 * (x + y))
				.min()
				.unwrap();
			let ribbon = present.product::<usize>();
			wrap + ribbon
		})
		.sum()
}
