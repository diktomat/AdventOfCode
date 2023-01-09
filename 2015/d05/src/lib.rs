use std::collections::HashSet;

pub fn part1() -> usize {
	include_str!("input.txt")
		.lines()
		.filter(|&line| {
			let mut prev = None;
			let mut vowel_count: u8 = 0;
			let mut has_double = false;
			for c in line.chars() {
				if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
					vowel_count += 1;
				}
				if let Some(prev) = prev {
					match (prev, c) {
						('a', 'b') => return false,
						('c', 'd') => return false,
						('p', 'q') => return false,
						('x', 'y') => return false,
						_ if prev == c => has_double = true,
						_ => (),
					}
				}
				prev = Some(c);
			}
			has_double && vowel_count >= 3
		})
		.count()
}

pub fn part2() -> usize {
	include_str!("input.txt")
		.lines()
		.filter(|&line| {
			let mut pairs = HashSet::new();
			let mut has_combination = false;
			let mut has_pairs = false;

			for window in format!("{line}\n").as_bytes().windows(3) {
				let &[a, b, c] = window else { unreachable!() };
				if a == b && a == c {
					pairs.remove(&(a, b));
					continue;
				}
				if a == c {
					has_combination = true;
				}
				if pairs.contains(&(a, b)) {
					has_pairs = true;
				}
				pairs.insert((a, b));
			}
			has_pairs && has_combination
		})
		.count()
}
