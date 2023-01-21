#![allow(non_snake_case)]

fn main() {
	macro_rules! print_day {
		($day:ident) => {
			let day = stringify!($day).replace('d', "");
			println!("Day {day} Part 1: {}", $day::part1());
			println!("Day {day} Part 2: {}", $day::part2());
		};
	}

	print_day!(d01);
	print_day!(d02);
	print_day!(d03);
	print_day!(d04);
	print_day!(d05);
	print_day!(d06);
	print_day!(d07);
	print_day!(d08);
	print_day!(d09);
	print_day!(d10);
	print_day!(d11);
	print_day!(d12);
	print_day!(d13);
	print_day!(d14);
}
