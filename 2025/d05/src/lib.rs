static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

pub fn part1(use_sample: bool) -> usize {
	let (fresh, avail) = if use_sample { SAMPLE } else { INPUT }
		.split_once("\n\n")
		.expect("Malformed input!");
	let mut ranges = Vec::new();
	for range in fresh.lines() {
		let (from, to) = range.split_once('-').expect("Malformed input!");
		ranges.push(
			from.parse::<usize>().expect("Malformed input!")
				..=to.parse::<usize>().expect("Malformed input!"),
		);
	}
	avail
		.lines()
		.filter(|ing| {
			ranges
				.iter()
				.any(|range| range.contains(&ing.parse().expect("Malformed input!")))
		})
		.count()
}

pub fn part2(use_sample: bool) -> usize {
	let (fresh, _) = if use_sample { SAMPLE } else { INPUT }
		.split_once("\n\n")
		.expect("Malformed input!");
	let mut ranges = Vec::<(usize, usize)>::new();
	for range in fresh.lines() {
		let range = range.split_once('-').expect("Malformed input!");
		let range = (
			range.0.parse::<usize>().expect("Malformed input!"),
			range.1.parse::<usize>().expect("Malformed input!"),
		);
		ranges.push(range);
	}
	ranges.sort_by(|a, b| a.0.cmp(&b.0));
	let mut i = 0;
	let mut res = Vec::new();
	while i < ranges.len() {
		let mut cur = ranges[i];
		let mut j = i + 1;
		while j < ranges.len() && ranges[j].0 <= cur.1 {
			if ranges[j].1 > cur.1 {
				cur.1 = ranges[j].1;
			}
			j += 1;
		}
		res.push(cur);
		i = j;
	}
	res.iter().map(|r| r.0.abs_diff(r.1) + 1).sum()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(super::part1(true), 3);
		assert_eq!(super::part1(false), 737);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(true), 14);
		assert_eq!(super::part2(false), 357485433193284);
	}
}
