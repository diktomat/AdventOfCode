use std::collections::HashSet;

pub fn part1() -> usize {
	include_str!("input.txt")
		.chars()
		.fold((HashSet::from([(0, 0)]), (0, 0)), |(set, pos), elem| {
			let pos = match elem {
				'>' => (pos.0 + 1, pos.1),
				'<' => (pos.0 - 1, pos.1),
				'^' => (pos.0, pos.1 + 1),
				'v' => (pos.0, pos.1 - 1),
				_ => panic!("Malformed input!"),
			};
			let mut set = set;
			set.insert(pos);
			(set, pos)
		})
		.0
		.len()
}

pub fn part2() -> usize {
	include_str!("input.txt")
		.chars()
		.fold(
			(HashSet::from([(0, 0)]), [(0, 0), (0, 0)]),
			|(set, pos), elem| {
				let mut pos = [pos[1], pos[0]];
				pos[0] = match elem {
					'>' => (pos[0].0 + 1, pos[0].1),
					'<' => (pos[0].0 - 1, pos[0].1),
					'^' => (pos[0].0, pos[0].1 + 1),
					'v' => (pos[0].0, pos[0].1 - 1),
					_ => panic!("Malformed input!"),
				};
				let mut set = set;
				set.insert(pos[0]);
				(set, pos)
			},
		)
		.0
		.len()
}
