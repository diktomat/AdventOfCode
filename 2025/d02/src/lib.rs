static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

fn does_repeat(s: &str, chunk_size: usize) -> bool {
	let len = s.len();
	if len % chunk_size != 0 {
		return false;
	};
	if len == chunk_size {
		return true;
	}

	let (left, rest) = s.split_at(chunk_size);
	let (mid, _) = rest.split_at(chunk_size);
	if left == mid {
		return does_repeat(rest, chunk_size);
	}
	false
}

pub fn part1(use_sample: bool) -> usize {
	if use_sample { SAMPLE } else { INPUT }
		.trim_end()
		.split(',')
		.map(|range| {
			let (start, stop) = range.split_once('-').expect("Malformed input!");
			(start.parse::<usize>().unwrap()..=stop.parse::<usize>().unwrap())
				.filter(|id| {
					let id = id.to_string();
					let len = id.len();
					if len % 2 != 0 {
						return false;
					}
					does_repeat(&id, len / 2)
				})
				.sum::<usize>()
		})
		.sum()
}

pub fn part2(use_sample: bool) -> usize {
	if use_sample { SAMPLE } else { INPUT }
		.trim_end()
		.split(',')
		.map(|range| {
			let (start, stop) = range.split_once('-').expect("Malformed input!");
			(start.parse::<usize>().unwrap()..=stop.parse::<usize>().unwrap())
				.filter(|id| {
					let id = id.to_string();
					(1..=id.len() / 2)
						.rev()
						.any(|chunk_size| does_repeat(&id, chunk_size))
				})
				.sum::<usize>()
		})
		.sum()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(super::part1(true), 1227775554);
		assert_eq!(super::part1(false), 23534117921);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(true), 4174379265);
		assert_eq!(super::part2(false), 31755323497);
	}
}
