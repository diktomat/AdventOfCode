fn main() {
	if std::env::args().next_back() == Some("-p".into()) {
		// plain output
		println!("{}\n{}", d04::part1(false), d04::part2(false));
		std::process::exit(0);
	}

	println!("Day 4");
	println!(
		"\tPart 1 Sample: {}, Input: {}",
		d04::part1(true),
		d04::part1(false)
	);
	println!(
		"\tPart 2 Sample: {}, Input: {}",
		d04::part2(true),
		d04::part2(false)
	);
}
