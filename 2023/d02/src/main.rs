static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

fn main() {
	println!("Day 2");
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
		.filter_map(|line| {
			if let Some((game, cubes)) = line.split_once(": ") {
				for set in cubes.split("; ") {
					for color in set.split(", ") {
						match color.split_once(' ') {
							Some((i, "red")) if i.parse::<u32>().unwrap() > 12 => return None,
							Some((i, "green")) if i.parse::<u32>().unwrap() > 13 => return None,
							Some((i, "blue")) if i.parse::<u32>().unwrap() > 14 => return None,
							_ => (),
						}
					}
				}
				return game
					.chars()
					.filter(char::is_ascii_digit)
					.collect::<String>()
					.parse::<u32>()
					.ok();
			}
			None
		})
		.sum()
}

fn part2(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			if let Some((_, cubes)) = line.split_once(": ") {
				let mut colors = [0, 0, 0];
				for set in cubes.split("; ") {
					for color in set.split(", ") {
						match color.split_once(' ') {
							Some((i, "red")) => {
								colors[0] = i.parse::<u32>().unwrap().max(colors[0])
							}
							Some((i, "green")) => {
								colors[1] = i.parse::<u32>().unwrap().max(colors[1])
							}
							Some((i, "blue")) => {
								colors[2] = i.parse::<u32>().unwrap().max(colors[2])
							}
							_ => panic!("Malformed input!"),
						}
					}
				}
				return colors[0] * colors[1] * colors[2];
			}
			panic!("Malformed input!");
		})
		.sum()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(super::part1(super::SAMPLE), 8);
		assert_eq!(super::part1(super::INPUT), 2727);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(super::SAMPLE), 2286);
		assert_eq!(super::part2(super::INPUT), 56580);
	}
}
