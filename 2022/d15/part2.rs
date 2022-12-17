fn main() {
	let input: Vec<Vec<i32>> = include_str!("input.txt")
		.lines()
		.map(|line| {
			line.split(|c: char| !c.is_numeric() && c != '-')
				.filter(|e| !e.is_empty())
				.map(|e| e.parse().unwrap())
				.collect()
		})
		.map(|e: Vec<i32>| vec![e[0], e[1], e[2], e[1], distance(e[0], e[1], e[2], e[3])])
		.collect();

	let limit = 4000000;

	'main: for sensor in &input {
		let sx = sensor[0];
		let sy = sensor[1];
		let dist = sensor[4];

		'circle: for pt in circle(sx, sy, dist + 1) {
			let cx = pt[0];
			let cy = pt[1];
			if cx < 0 || cy < 0 || cx > limit || cy > limit {
				continue;
			}

			for tst in &input {
				if distance(tst[0], tst[1], cx, cy) <= tst[4] {
					continue 'circle;
				}
			}
			println!("{}", tunefreq(cx, cy));
			break 'main;
		}
	}
}

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
	(x1 - x2).abs() + (y1 - y2).abs()
}

fn tunefreq(x: i32, y: i32) -> i64 {
	x as i64 * 4000000i64 + y as i64
}

fn circle(cx: i32, cy: i32, r: i32) -> Vec<Vec<i32>> {
	let mut ret = Vec::new();
	for x in 0..=r {
		let dy = r - x;
		ret.push(vec![cx + x, cy + dy]);
		ret.push(vec![cx + x, cy - dy]);
		ret.push(vec![cx - x, cy + dy]);
		ret.push(vec![cx - x, cy - dy]);
	}
	ret
}
