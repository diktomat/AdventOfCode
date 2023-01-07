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
