fn main() {
	if std::env::args().next_back() == Some("-p".into()) {
		// plain output
		println!("{}\n{}", d05::part1(false), d05::part2(false));
		std::process::exit(0);
	}

	println!("Day 5");
	println!(
		"\tPart 1 Sample: {}, Input: {}",
		d05::part1(true),
		d05::part1(false)
	);
	println!(
		"\tPart 2 Sample: {}, Input: {}",
		d05::part2(true),
		d05::part2(false)
	);
}
