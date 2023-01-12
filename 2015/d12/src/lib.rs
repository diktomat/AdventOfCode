use std::ops::ControlFlow;

use serde_json::Value;

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> i32 {
	INPUT
		.chars()
		.fold((0, 0, 1, false), |(sum, cur, sign, in_string), c| match c {
			'0'..='9' => (sum, cur * 10 + i32::from(c as u8 - 48), sign, in_string),
			'-' => (sum, cur, -1, in_string),
			'"' => (sum, cur, sign, !in_string),
			_ => (sum + cur * sign, 0, 1, in_string),
		})
		.0
}

pub fn part2() -> i64 {
	sum_traverse(&serde_json::from_str(INPUT).unwrap())
}

fn sum_traverse(input: &Value) -> i64 {
	match &input {
		Value::Number(num) => num.as_i64().unwrap(),
		Value::Array(arr) => arr.iter().map(sum_traverse).sum(),
		Value::Object(obj) => match obj.values().try_fold(0, |acc, val| match val {
			Value::String(str) if str == "red" => ControlFlow::Break(0),
			val => ControlFlow::Continue(acc + sum_traverse(val)),
		}) {
			ControlFlow::Break(_) => 0,
			ControlFlow::Continue(val) => val,
		},
		_ => 0,
	}
}
