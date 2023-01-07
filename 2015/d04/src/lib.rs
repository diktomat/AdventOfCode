pub fn part1() -> usize {
	let secret = include_str!("input.txt").trim();
	(1..)
		.find(|i| format!("{:x}", md5::compute(format!("{secret}{i}"))).starts_with("00000"))
		.unwrap()
}

pub fn part2() -> usize {
	let secret = include_str!("input.txt").trim();
	(1..)
		.find(|i| format!("{:x}", md5::compute(format!("{secret}{i}"))).starts_with("000000"))
		.unwrap()
}
