#[test]
fn day01() {
	assert_eq!(d01::part1(), 74);
	assert_eq!(d01::part2(), 1795);
}

#[test]
fn day02() {
	assert_eq!(d02::part1(), 1586300);
	assert_eq!(d02::part2(), 3737498);
}

#[test]
fn day03() {
	assert_eq!(d03::part1(), 2081);
	assert_eq!(d03::part2(), 2341);
}

#[test]
#[ignore] // takes >15s in debug
fn day04() {
	assert_eq!(d04::part1(), 117946);
	assert_eq!(d04::part2(), 3938038);
}

#[test]
fn day05() {
	assert_eq!(d05::part1(), 258);
	assert_eq!(d05::part2(), 53);
}

#[test]
fn day06() {
	assert_eq!(d06::part1(), 569999);
	assert_eq!(d06::part2(), 17836115);
}

#[test]
fn day07() {
	assert_eq!(d07::part1(), 956);
	assert_eq!(d07::part2(), 40149);
}

#[test]
fn day08() {
	assert_eq!(d08::part1(), 1333);
	assert_eq!(d08::part2(), 2046);
}

#[test]
fn day09() {
	assert_eq!(d09::part1(), 207);
	assert_eq!(d09::part2(), 804);
}
