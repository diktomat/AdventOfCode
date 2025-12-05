use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

fn prepare(input: &str) -> (Vec<(isize, isize)>, HashMap<(isize, isize), u8>) {
	let mut rolls = Vec::with_capacity(138 * 138);
	let mut adj = HashMap::<(isize, isize), u8>::new();
	input.lines().enumerate().for_each(|(y, line)| {
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
	(rolls, adj)
}

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
	let (rolls, mut adj) = prepare(if use_sample { SAMPLE } else { INPUT });
	rolls
		.iter()
		.filter(|(x, y)| *adj.entry((*x, *y)).or_default() < 4)
		.count()
}

pub fn part2(use_sample: bool) -> usize {
	let (mut rolls, mut adj) = prepare(if use_sample { SAMPLE } else { INPUT });
	let start = rolls.len();

	loop {
		let before = rolls.len();
		rolls = rolls
			.into_iter()
			.filter(|(x, y)| {
				if *adj.entry((*x, *y)).or_default() < 4 {
					for (ax, ay) in neighbours(*x, *y) {
						*adj.entry((ax, ay)).or_default() -= 1;
					}
					return false;
				}
				true
			})
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
