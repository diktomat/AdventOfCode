use std::collections::HashMap;
use Instruction::*;
use Wire::*;

pub fn part1() -> u16 {
	let mut wires: HashMap<u16, Instruction> = HashMap::new();
	let mut cache: HashMap<u16, u16> = HashMap::new();
	for (wire, instruction) in include_str!("input.txt").lines().map(parse) {
		wires.insert(wire, instruction);
	}

	get_wire(&wires, Gate(97), &mut cache)
}

pub fn part2() -> u16 {
	let mut wires: HashMap<u16, Instruction> = HashMap::new();
	let mut cache: HashMap<u16, u16> = HashMap::new();
	for (wire, instruction) in include_str!("input.txt").lines().map(parse) {
		wires.insert(wire, instruction);
	}
	wires.insert(98, Let(Value(956)));

	get_wire(&wires, Gate(97), &mut cache)
}

#[derive(Clone, Copy)]
enum Wire {
	Gate(u16),
	Value(u16),
}
impl std::fmt::Debug for Wire {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value(val) => f.write_fmt(format_args!("{}", val)),
			Gate(val) => {
				let left = (val & 0xFF00) >> 8;
				let right = val & 0xFF;
				f.write_fmt(format_args!(
					"{}{}",
					left as u8 as char, right as u8 as char
				))
			}
		}
	}
}

enum Instruction {
	Let(Wire),
	And(Wire, Wire),
	Or(Wire, Wire),
	Not(Wire),
	RShift(Wire, Wire),
	LShift(Wire, Wire),
}
impl std::fmt::Debug for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Let(from) => f.write_fmt(format_args!("= {:?}", from)),
			And(left, right) => f.write_fmt(format_args!("{left:?} & {right:?}")),
			Or(left, right) => f.write_fmt(format_args!("{left:?} | {right:?}")),
			Not(left) => f.write_fmt(format_args!("!{left:?}")),
			RShift(left, right) => f.write_fmt(format_args!("{left:?} << {right:?}")),
			LShift(left, right) => f.write_fmt(format_args!("{left:?} >> {right:?}")),
		}
	}
}

// I should really start learning Nom..
fn parse(line: &str) -> (u16, Instruction) {
	let line = line.split_whitespace();
	let ins = line
		.clone()
		.find(|word| word.chars().all(|c| c.is_ascii_uppercase()))
		.unwrap_or("LET");
	let line: Vec<Wire> = line
		.map(|word| match word {
			_ if word.chars().all(|c| c.is_ascii_digit()) => Value(word.parse().unwrap()),
			_ if word.chars().all(|c| c.is_ascii_lowercase()) => {
				Gate(word.chars().fold(0, |acc, c| (acc << 8) + c as u16))
			}
			_ => Value(0),
		})
		.collect();

	let res = match ins {
		"LET" => (line[2], Let(line[0])),
		"AND" => (line[4], And(line[0], line[2])),
		"OR" => (line[4], Or(line[0], line[2])),
		"NOT" => (line[3], Not(line[1])),
		"LSHIFT" => (line[4], LShift(line[0], line[2])),
		"RSHIFT" => (line[4], RShift(line[0], line[2])),
		_ => panic!("Unexpected instruction!"),
	};
	match res {
		(Gate(wire), ins) => (wire, ins),
		_ => unreachable!(),
	}
}

fn get_wire(wires: &HashMap<u16, Instruction>, wire: Wire, cache: &mut HashMap<u16, u16>) -> u16 {
	let wire = match wire {
		Value(n) => return n,
		Gate(num) => num,
	};
	if let Some(&val) = cache.get(&wire) {
		return val;
	}
	let value = match wires.get(&wire).unwrap() {
		Let(from) => get_wire(wires, *from, cache),
		And(left, right) => get_wire(wires, *left, cache) & get_wire(wires, *right, cache),
		Or(left, right) => get_wire(wires, *left, cache) | get_wire(wires, *right, cache),
		Not(left) => !get_wire(wires, *left, cache),
		LShift(left, right) => get_wire(wires, *left, cache) << get_wire(wires, *right, cache),
		RShift(left, right) => get_wire(wires, *left, cache) >> get_wire(wires, *right, cache),
	};
	cache.insert(wire, value);
	value
}
