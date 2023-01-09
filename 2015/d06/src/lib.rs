use regex::Regex;

pub fn part1() -> usize {
	let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
	let mut lights = Box::new([[false; 1000]; 1000]);

	for cap in re.captures_iter(include_str!("input.txt")) {
		let p1: (usize, usize) = (cap[2].parse().unwrap(), cap[3].parse().unwrap());
		let p2: (usize, usize) = (cap[4].parse().unwrap(), cap[5].parse().unwrap());

		for row in lights.iter_mut().take(p2.0 + 1).skip(p1.0) {
			for light in row.iter_mut().take(p2.1 + 1).skip(p1.1) {
				*light = match &cap[1] {
					"turn on" => true,
					"turn off" => false,
					"toggle" => !*light,
					_ => panic!(),
				}
			}
		}
	}
	lights
		.iter()
		.map(|row| row.iter().filter(|&&light| light).count())
		.sum()
}

pub fn part2() -> isize {
	let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
	let mut lights = Box::new([[0; 1000]; 1000]);

	for cap in re.captures_iter(include_str!("input.txt")) {
		let p1: (usize, usize) = (cap[2].parse().unwrap(), cap[3].parse().unwrap());
		let p2: (usize, usize) = (cap[4].parse().unwrap(), cap[5].parse().unwrap());

		for row in lights.iter_mut().take(p2.0 + 1).skip(p1.0) {
			for light in row.iter_mut().take(p2.1 + 1).skip(p1.1) {
				*light += match &cap[1] {
					"turn on" => 1,
					"toggle" => 2,
					"turn off" if *light > 0 => -1,
					"turn off" => 0,
					_ => panic!(),
				}
			}
		}
	}
	lights.iter().map(|&i| i.iter().sum::<isize>()).sum()
}
