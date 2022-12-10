use std::fs;

pub fn part1() -> i32 {
	fs::read_to_string(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))
		.expect("Missing input.txt!")
		.chars()
		.fold(0, |acc, e| {
			acc + match e {
				'(' => 1,
				')' => -1,
				_ => panic!("Malformed input!"),
			}
		})
}

pub fn part2() -> i32 {
	fs::read_to_string(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))
		.expect("Missing input.txt!")
		.chars()
		.try_fold((0, 0), |(story, pos), e| match story {
			-1 => Err(pos),
			_ => Ok((
				story
					+ match e {
						'(' => 1,
						')' => -1,
						_ => panic!("Malformed input!"),
					},
				pos + 1,
			)),
		})
		.expect_err("Never reaching basement!")
}
