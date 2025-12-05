fn main() {
	macro_rules! print_day {
		($day:ident) => {
			let day = stringify!($day).replace('d', "");
			println!("Day {day} Part 1: {}", $day::part1(false));
			println!("Day {day} Part 2: {}", $day::part2(false));
		};
	}

	print_day!(d01);
	print_day!(d02);
	print_day!(d03);
	print_day!(d04);
	print_day!(d05);
}
