static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

struct Rotation {
	direction: isize,
	amount: isize,
}

impl std::str::FromStr for Rotation {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (direction, amount) = s.split_at(1);
		let direction = match direction {
			"L" => -1,
			"R" => 1,
			_ => panic!("Malformed input!"),
		};
		let amount = amount.parse().expect("Malformed input!");
		Ok(Rotation { direction, amount })
	}
}

pub fn part1(use_sample: bool) -> usize {
	if use_sample { SAMPLE } else { INPUT }
		.lines()
		.fold((0, 50), |(acc, dial), line| {
			let rotation: Rotation = line.parse().expect("Malformed input!");
			let dial = (dial + rotation.amount * rotation.direction).rem_euclid(100);
			(if dial == 0 { acc + 1 } else { acc }, dial)
		})
		.0
}

pub fn part2(use_sample: bool) -> isize {
	if use_sample { SAMPLE } else { INPUT }
		.lines()
		.fold((0, 50), |(mut acc, mut dial), line| {
			let rotation: Rotation = line.parse().expect("Malformed input!");

			if dial == 0 && rotation.direction == -1 {
				dial = 100;
			};
			dial += rotation.amount * rotation.direction;

			if dial <= 0 {
				acc += 1;
			};

			(acc + (dial / 100).abs(), dial.rem_euclid(100))
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
