pub fn part1() -> usize {
	let mut input = include_bytes!("input.txt")
		.iter()
		.take_while(|&b| (48..=57).contains(b))
		.map(|&c| c - 48)
		.collect::<Vec<u8>>();

	for _ in 0..40 {
		input = look_and_say(&input);
	}

	input.len()
}

pub fn part2() -> usize {
	let mut input = include_bytes!("input.txt")
		.iter()
		.take_while(|&b| (48..=57).contains(b))
		.map(|&c| c - 48)
		.collect::<Vec<u8>>();

	for _ in 0..50 {
		input = look_and_say(&input);
	}

	input.len()
}

fn look_and_say(input: &Vec<u8>) -> Vec<u8> {
	let mut output = Vec::with_capacity(input.len() * 2);
	let mut last = input[0];
	let mut count = 1;
	for &char in input[1..].iter() {
		if char == last {
			count += 1;
		} else {
			output.push(count);
			output.push(last);
			last = char;
			count = 1;
		}
	}
	output.push(count);
	output.push(last);
	output
}
