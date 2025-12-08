use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");
static SAMPLE: &str = include_str!("sample.txt");

struct Jbox {
	x: usize,
	y: usize,
	z: usize,
}

impl std::str::FromStr for Jbox {
	type Err = std::num::ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut nums = s.split(',');
		Ok(Jbox {
			x: nums.next().unwrap().parse()?,
			y: nums.next().unwrap().parse()?,
			z: nums.next().unwrap().parse()?,
		})
	}
}

impl Jbox {
	fn dist(&self, other: &Jbox) -> usize {
		// square root, anyone?
		self.x.abs_diff(other.x).pow(2)
			+ self.y.abs_diff(other.y).pow(2)
			+ self.z.abs_diff(other.z).pow(2)
	}
}

struct Distance {
	b1: usize,
	b2: usize,
	dist: usize,
}

fn unionize(conns: &mut Vec<HashSet<usize>>, b1: usize, b2: usize) {
	let c1 = conns
		.iter()
		.enumerate()
		.find(|(_, conn)| conn.contains(&b1));
	let c2 = conns
		.iter()
		.enumerate()
		.find(|(_, conn)| conn.contains(&b2));

	match (c1, c2) {
		(Some((id1, _)), Some((id2, _))) => {
			if id1 != id2 {
				let c2 = conns[id2].clone();
				conns[id1].extend(c2);
				conns.remove(id2);
			}
		}
		(Some((id1, _)), None) => {
			conns[id1].insert(b2);
		}
		(None, Some((id2, _))) => {
			conns[id2].insert(b1);
		}
		(None, None) => conns.push(HashSet::from([b1, b2])),
	}
}

fn calc_dists(boxes: &Vec<Jbox>) -> Vec<Distance> {
	let mut dists = vec![];
	for i in 0..boxes.len() {
		for j in i + 1..boxes.len() {
			dists.push(Distance {
				b1: i,
				b2: j,
				dist: *&boxes[i].dist(&boxes[j]),
			});
		}
	}
	dists.sort_by_key(|d| d.dist);
	dists
}

pub fn part1(use_sample: bool) -> usize {
	let boxes: Vec<_> = if use_sample { SAMPLE } else { INPUT }
		.lines()
		.map(|l| l.parse::<Jbox>().unwrap())
		.collect();
	let dists = calc_dists(&boxes);
	let mut conns: Vec<HashSet<usize>> = vec![];

	for i in 0..if use_sample { 10 } else { 1000 } {
		let Distance { b1, b2, dist: _ } = dists[i];
		unionize(&mut conns, b1, b2);
	}

	conns.sort_by_key(|c| c.len());
	conns
		.iter()
		.rev()
		.take(3)
		.fold(1, |prod, conn| prod * conn.len())
}

pub fn part2(use_sample: bool) -> usize {
	let boxes: Vec<_> = if use_sample { SAMPLE } else { INPUT }
		.lines()
		.map(|l| l.parse::<Jbox>().unwrap())
		.collect();
	let dists = calc_dists(&boxes);
	let mut conns: Vec<HashSet<usize>> = vec![];

	for i in 0..dists.len() {
		let Distance { b1, b2, dist: _ } = dists[i];
		unionize(&mut conns, b1, b2);

		if conns[0].len() == boxes.len() {
			return boxes[b1].x * boxes[b2].x;
		}
	}

	unreachable!()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(super::part1(true), 40);
		assert_eq!(super::part1(false), 330786);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(true), 25272);
		assert_eq!(super::part2(false), 3276581616);
	}
}
