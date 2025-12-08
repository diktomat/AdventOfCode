fn main() {
	if std::env::args().next_back() == Some("-p".into()) {
		// plain output
		println!("{}\n{}", d08::part1(false), d08::part2(false));
		std::process::exit(0);
	}

	println!("Day 8");
	println!(
		"\tPart 1 Sample: {}, Input: {}",
		d08::part1(true),
		d08::part1(false)
	);
	println!(
		"\tPart 2 Sample: {}, Input: {}",
		d08::part2(true),
		d08::part2(false)
	);
}
