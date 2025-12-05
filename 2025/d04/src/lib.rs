use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

fn neighbours(x: isize, y: isize) -> Vec<(isize, isize)> {
	vec![
		(x - 1, y - 1),
		(x - 1, y),
		(x - 1, y + 1),
		(x, y - 1),
		(x, y + 1),
		(x + 1, y - 1),
		(x + 1, y),
		(x + 1, y + 1),
	]
}
pub fn part1(use_sample: bool) -> usize {
	let mut rolls = Vec::with_capacity(138 * 138);
	let mut adj = HashMap::<(isize, isize), u8>::new();
	if use_sample { SAMPLE } else { INPUT }
		.lines()
		.enumerate()
		.for_each(|(y, line)| {
			line.char_indices().for_each(|(x, c)| {
				if c == '@' {
					rolls.push((x as isize, y as isize));
				}
			})
		});
	for (x, y) in &rolls {
		for (ax, ay) in neighbours(*x, *y) {
			*adj.entry((ax, ay)).or_insert(0) += 1;
		}
	}
	rolls
		.iter()
		.filter(|(x, y)| *adj.entry((*x, *y)).or_default() < 4)
		.count()
}

pub fn part2(use_sample: bool) -> usize {
	let mut rolls = Vec::with_capacity(138 * 138);
	if use_sample { SAMPLE } else { INPUT }
		.lines()
		.enumerate()
		.for_each(|(y, line)| {
			line.char_indices().for_each(|(x, c)| {
				if c == '@' {
					rolls.push((x as isize, y as isize));
				}
			})
		});
	let start = rolls.len();
	loop {
		let mut adj = HashMap::<(isize, isize), u8>::new();
		let before = rolls.len();
		for (x, y) in &rolls {
			for (ax, ay) in neighbours(*x, *y) {
				*adj.entry((ax, ay)).or_insert(0) += 1;
			}
		}
		rolls = rolls
			.into_iter()
			.filter(|(x, y)| *adj.entry((*x, *y)).or_default() >= 4)
			.collect();

		if before == rolls.len() {
			break;
		}
	}
	start - rolls.len()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(super::part1(true), 13);
		assert_eq!(super::part1(false), 1543);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(true), 43);
		assert_eq!(super::part2(false), 9038);
	}
}
