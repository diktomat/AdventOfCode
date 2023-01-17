const INPUT: &str = include_str!("input.txt");

pub fn part1() -> usize {
	INPUT
		.lines()
		.map(|line| line.parse::<Reindeer>().expect("Malformed input!"))
		.map(|r| r.race(2503))
		.max()
		.unwrap()
}

pub fn part2() -> usize {
	let mut state: Vec<_> = INPUT
		.lines()
		.map(|line| line.parse().expect("Malformed input!"))
		.map(|r| RaceState {
			reindeer: r,
			points: 0,
			distance: 0,
		})
		.collect();

	for t in 0..=2503 {
		for r in &mut state {
			let cycle_time = r.reindeer.time_speeding + r.reindeer.time_resting;
			if t % cycle_time < r.reindeer.time_speeding {
				r.distance += r.reindeer.speed;
			}
		}
		state.iter_mut().max_by_key(|r| r.distance).unwrap().points += 1;
	}

	state.iter().map(|s| s.points).max().unwrap()
}

struct Reindeer {
	speed: usize,
	time_speeding: usize,
	time_resting: usize,
}

impl Reindeer {
	fn race(self, time: usize) -> usize {
		let cycle_time = self.time_resting + self.time_speeding;
		let (full_cycles, rem) = (time / cycle_time, time % cycle_time);
		let seconds_at_speed = full_cycles * self.time_speeding + rem.min(self.time_speeding);
		seconds_at_speed * self.speed
	}
}

impl std::str::FromStr for Reindeer {
	type Err = peg::error::ParseError<peg::str::LineCol>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		peg::parser! {
			grammar reindeer_parser() for str {
				rule name() -> String
					= n:$(['A'..='Z']['a'..='z']+) { String::from(n) }

				rule number() -> usize
					= n:$(['0'..='9']+) {? n.parse().or(Err("not an usize")) }

				pub(crate) rule reindeer() -> Reindeer
					= name() " can fly " speed:number() " km/s for " time_speeding:number()
						" seconds, but then must rest for " time_resting:number() " seconds."
					{
						Reindeer{speed, time_speeding, time_resting}
					}
			}
		}
		reindeer_parser::reindeer(s)
	}
}

struct RaceState {
	reindeer: Reindeer,
	points: usize,
	distance: usize,
}
