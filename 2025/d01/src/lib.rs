static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

#[derive(PartialEq)]
enum Direction {
	Left,
	Right,
}

struct Rotation {
	direction: Direction,
	amount: i32,
}

impl std::str::FromStr for Rotation {
	type Err = std::num::ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts = s.split_at(1);
		let direction = match parts.0 {
			"L" => Direction::Left,
			"R" => Direction::Right,
			_ => panic!("Malformed input!"),
		};
		let amount = parts.1.parse::<i32>()?;
		Ok(Rotation { direction, amount })
	}
}

pub fn part1(use_sample: bool) -> i32 {
	let input = if use_sample { SAMPLE } else { INPUT };

	input
		.lines()
		.fold((0, 50), |(acc, dial), line| {
			let rotation: Rotation = line.parse().unwrap();
			let mut acc = acc;
			let mut dial = dial;

			dial = match rotation.direction {
				Direction::Left => dial - rotation.amount,
				Direction::Right => dial + rotation.amount,
			} % 100;
			if dial < 0 {
				dial += 100;
			};

			acc += match dial {
				0 => 1,
				_ => 0,
			};

			(acc, dial)
		})
		.0
}

pub fn part2(use_sample: bool) -> i32 {
	let input = if use_sample { SAMPLE } else { INPUT };

	input
		.lines()
		.fold((0, 50), |(acc, dial), line| {
			let rotation: Rotation = line.parse().unwrap();
			let mut acc = acc;
			let mut dial = dial;
			if dial == 0 && rotation.direction == Direction::Left {
				dial = 100;
			};
			dial = match rotation.direction {
				Direction::Left => dial - rotation.amount,
				Direction::Right => dial + rotation.amount,
			};
			acc += (dial / 100).abs();
			if dial < 1 {
				acc += 1;
			};
			dial = dial % 100;
			if dial < 0 {
				dial = dial + 100;
			};

			(acc, dial)
		})
		.0
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(super::part1(true), 3);
		assert_eq!(super::part1(false), 1097);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(true), 6);
		assert_eq!(super::part2(false), 7101);
	}
}
