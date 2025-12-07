static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

pub fn part1(use_sample: bool) -> usize {
	let mut manifold = if use_sample { SAMPLE } else { INPUT }.lines();
	let mut beams = vec![];
	beams.push(
		manifold
			.nth(0)
			.unwrap()
			.char_indices()
			.find(|&(_, c)| c == 'S')
			.unwrap()
			.0,
	);
	let mut splits = 0;
	for line in manifold {
		let mut newbeams = vec![];
		for beam in beams {
			match line.as_bytes()[beam] {
				b'^' => {
					splits += 1;
					newbeams.push(beam + 1);
					newbeams.push(beam - 1);
				}
				b'.' => {
					newbeams.push(beam);
				}
				_ => {}
			}
		}
		newbeams.dedup();
		beams = newbeams;
	}
	splits
}

pub fn part2(use_sample: bool) -> usize {
	let mut manifold = if use_sample { SAMPLE } else { INPUT }.lines();
	let first = manifold.next().unwrap();
	let mut beams = vec![0; first.len()];
	beams[first.char_indices().find(|&(_, c)| c == 'S').unwrap().0] = 1;

	for line in manifold {
		let old = beams.clone();
		beams = vec![0; first.len()];
		for (beam, count) in old.iter().enumerate() {
			match line.as_bytes()[beam] {
				b'^' => {
					beams[beam - 1] += count;
					beams[beam + 1] += count;
				}
				b'.' => beams[beam] += count,
				_ => unreachable!(),
			}
		}
	}

	beams.iter().sum()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(super::part1(true), 21);
		assert_eq!(super::part1(false), 1662);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(true), 40);
		assert_eq!(super::part2(false), 40941112789504);
	}
}
