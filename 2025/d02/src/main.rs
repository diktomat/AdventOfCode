fn main() {
	if std::env::args().next_back() == Some("-p".into()) {
		// plain output
		println!("{}\n{}", d02::part1(false), d02::part2(false));
		std::process::exit(0);
	}

	println!("Day 2");
	println!(
		"\tPart 1 Sample: {}, Input: {}",
		d02::part1(true),
		d02::part1(false)
	);
	println!(
		"\tPart 2 Sample: {}, Input: {}",
		d02::part2(true),
		d02::part2(false)
	);
}
