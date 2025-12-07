fn main() {
	if std::env::args().next_back() == Some("-p".into()) {
		// plain output
		println!("{}\n{}", d07::part1(false), d07::part2(false));
		std::process::exit(0);
	}

	println!("Day 7");
	println!(
		"\tPart 1 Sample: {}, Input: {}",
		d07::part1(true),
		d07::part1(false)
	);
	println!(
		"\tPart 2 Sample: {}, Input: {}",
		d07::part2(true),
		d07::part2(false)
	);
}
