use std::fmt::Write;

pub fn part1() -> String {
	let mut password: Password = include_bytes!("input.txt")
		.iter()
		.copied()
		.take_while(|b| (97..=122).contains(b))
		.collect();

	password.inc();
	password.to_string()
}

pub fn part2() -> String {
	let mut password: Password = "cqjxxyzz".bytes().collect();
	password.inc();
	password.to_string()
}

struct Password(Vec<u8>);

impl Password {
	fn valid(&self) -> bool {
		let mut last_chars = (0, 0);
		let mut pairs_count = 0;
		let mut last_pair_idx = 0;
		let mut has_straight = false;

		for (i, &b) in self.0.iter().enumerate() {
			if [105, 108, 111].contains(&b) {
				return false;
			}
			if last_chars.1 == b && (i - last_pair_idx > 1 || i < 2) {
				last_pair_idx = i;
				pairs_count += 1;
			}
			if last_chars.1.checked_sub(last_chars.0) == Some(1)
				&& b.checked_sub(last_chars.1) == Some(1)
			{
				has_straight = true;
			}
			last_chars = (last_chars.1, b);
		}

		pairs_count > 1 && has_straight
	}

	fn inc(&mut self) {
		loop {
			for b in self.0.iter_mut().rev() {
				*b += 1;
				if [105, 108, 111].contains(b) {
					*b += 1;
				}
				if *b <= 122 {
					break;
				}
				*b = 97;
			}
			if self.0.iter().all(|&b| b == 97) {
				self.0.push(97);
			}
			if self.valid() {
				break;
			}
		}
	}
}

impl FromIterator<u8> for Password {
	fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
		Self(iter.into_iter().collect())
	}
}

impl std::fmt::Display for Password {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for &b in &self.0 {
			f.write_char(b as char)?;
		}
		Ok(())
	}
}
