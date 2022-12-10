use std::{fs, num::IntErrorKind};

fn main() {
	let elves = elves();
	println!("{}", part1(&elves));
	println!("{}", part2(&elves));
}

fn elves() -> Vec<i32> {
	let mut cals = vec![0];
	fs::read_to_string("input.txt")
		.expect("Expecting `input.txt` in current directory.")
		.lines()
		.for_each(|l| {
			match l.parse::<i32>() {
				Err(e) if e.kind() == &IntErrorKind::Empty => cals.push(0),
				Ok(cal) => {
					if let Some(last) = cals.last_mut() {
						*last += cal;
					}
				}
				_ => panic!("Malformed input"),
			};
		});
	return cals;
}

fn part1(elves: &Vec<i32>) -> i32 {
	elves.iter().fold(0, |max, &e| max.max(e))
}

fn part2(elves: &Vec<i32>) -> i32 {
	let maxes = elves.iter().fold((0, 0, 0), |maxes, &e| match maxes {
		(huge, bigger, _) if e > huge => (e, huge, bigger),
		(huge, bigger, _) if e > bigger => (huge, e, bigger),
		(huge, bigger, big) if e > big => (huge, bigger, e),
		_ => maxes,
	});
	return maxes.0 + maxes.1 + maxes.2;
}
