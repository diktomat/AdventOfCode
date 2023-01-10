pub fn part1() -> usize {
	let mut escaped = 0;
	for line in include_str!("input.txt").lines() {
		escaped += 2;
		let mut escaping = false;
		for c in line.chars() {
			match (escaping, c) {
				(false, '\\') => (escaping, escaped) = (true, escaped + 1),
				(true, 'x') => (escaping, escaped) = (false, escaped + 2),
				(true, _) => escaping = false,
				_ => (),
			}
		}
	}
	escaped
}

pub fn part2() -> usize {
	include_str!("input.txt")
		.lines()
		.map(|line| line.chars().filter(|c| ['\\', '"'].contains(c)).count() + 2)
		.sum()
}
