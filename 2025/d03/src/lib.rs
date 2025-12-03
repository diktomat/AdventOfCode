static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

fn helper(bank: &str, count: usize) -> usize {
	fn inner(bank: &[u8], count: usize) -> Vec<u8> {
		if count == 1 {
			return vec![bank.iter().max().unwrap() - 48];
		}
		let idx = bank.len()
			- bank
				.iter()
				.rev()
				.skip(count - 1)
				.enumerate()
				.max_by_key(|b| b.1)
				.expect("No max to be found!")
				.0 - count;
		[inner(&bank[idx + 1..], count - 1), vec![bank[idx] - 48]].concat()
	}

	inner(bank.as_bytes(), count)
		.iter()
		.enumerate()
		.map(|(i, &j)| j as usize * 10_usize.pow(i as u32))
		.sum()
}

pub fn part1(use_sample: bool) -> usize {
	if use_sample { SAMPLE } else { INPUT }
		.lines()
		.map(|bank| helper(bank, 2))
		.sum()
}

pub fn part2(use_sample: bool) -> usize {
	if use_sample { SAMPLE } else { INPUT }
		.lines()
		.map(|bank| helper(bank, 12))
		.sum()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(super::part1(true), 357);
		assert_eq!(super::part1(false), 17430);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(true), 3121910778619);
		assert_eq!(super::part2(false), 171975854269367);
	}
}
