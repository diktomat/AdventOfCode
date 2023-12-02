static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

fn main() {
	println!("Day 1");
	println!(
		"\tPart 1 Sample: {}, Input: {}",
		part1(SAMPLE),
		part1(INPUT)
	);
	println!(
		"\tPart 2 Sample: {}, Input: {}",
		part2(SAMPLE),
		part2(INPUT)
	);
}

fn part1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
			digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
		})
		.sum()
}

fn part2(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let mut first = None;
			let mut last = None;

			for (i, c) in line.char_indices() {
				if c.is_ascii_digit() {
					if first.is_none() {
						first = c.to_digit(10)
					}
					last = c.to_digit(10)
				} else {
					let word = [line.get(i..i + 3), line.get(i..i + 4), line.get(i..i + 5)];
					let mut num = None;

					match word {
						[Some("one"), _, _] => num = Some(1),
						[Some("two"), _, _] => num = Some(2),
						[_, _, Some("three")] => num = Some(3),
						[_, Some("four"), _] => num = Some(4),
						[_, Some("five"), _] => num = Some(5),
						[Some("six"), _, _] => num = Some(6),
						[_, _, Some("seven")] => num = Some(7),
						[_, _, Some("eight")] => num = Some(8),
						[_, Some("nine"), _] => num = Some(9),
						_ => (),
					}

					if num.is_some() {
						if first.is_none() {
							first = num;
						}
						last = num;
					}
				}
			}
			first.unwrap() * 10 + last.unwrap()
		})
		.sum()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(super::part1(super::SAMPLE), 209);
		assert_eq!(super::part1(super::INPUT), 53194);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(super::SAMPLE), 281);
		assert_eq!(super::part2(super::INPUT), 54249);
	}
}
