#![feature(iter_array_chunks)]

use std::io::{self, Read};
use std::iter;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	println!("p1 = {}", part1(&input));
	println!("p2 = {}", part2(&input));

	Ok(())
}

fn part1(input: &str) -> u64 {
	solve(&mut parse(input), 20, |x| x / 3)
}

fn part2(input: &str) -> u64 {
	let mut monkeys = parse(input);
	let base = monkeys.iter().fold(1, |b, m| b * m.test as u64);
	solve(&mut monkeys, 10000, |x| x % base)
}

fn parse(input: &str) -> Vec<Monkey> {
	input
		.lines()
		.chain(iter::once("\n"))
		.array_chunks()
		.filter_map(|[_, i, o, d, t, f, _]| {
			let items = parse_items(i);
			let op = parse_op(o)?;
			let test = parse_test(d)?;
			let pass = parse_action(t)?;
			let fail = parse_action(f)?;
			Some(Monkey {
				items,
				op,
				test,
				pass,
				fail,
				times: 0,
			})
		})
		.collect()
}

fn solve<F>(monkeys: &mut [Monkey], rounds: usize, f: F) -> u64
where
	F: Fn(u64) -> u64,
{
	for _ in 0..rounds {
		for m in 0..monkeys.len() {
			let Monkey {
				op,
				test,
				pass,
				fail,
				..
			} = monkeys[m];
			for i in 0..monkeys[m].items.len() {
				let w = monkeys[m].items[i];
				let w = f(match op.0 {
					b'+' => w + op.1.unwrap_or(w),
					b'*' => w * op.1.unwrap_or(w),
					_ => w,
				});
				let n = if w % test as u64 == 0 { pass } else { fail };
				monkeys[n].items.push(w);
			}
			monkeys[m].times += monkeys[m].items.len() as u64;
			monkeys[m].items.clear();
		}
	}

	monkeys.sort_by(|a, b| b.times.cmp(&a.times));
	monkeys[0].times * monkeys[1].times
}

struct Monkey {
	items: Vec<u64>,
	op: (u8, Option<u64>),
	test: u32,
	pass: usize,
	fail: usize,
	times: u64,
}

fn parse_items(input: &str) -> Vec<u64> {
	input
		.trim()
		.strip_prefix("Starting items: ")
		.unwrap_or_default()
		.split(", ")
		.filter_map(|s| s.parse().ok())
		.collect()
}

fn parse_op(input: &str) -> Option<(u8, Option<u64>)> {
	let mut parts = input
		.trim()
		.strip_prefix("Operation: new = old ")?
		.split_whitespace();
	let o = parts.next()?.bytes().next()?;
	let r = parts.next()?.parse().ok();
	Some((o, r))
}

fn parse_test(input: &str) -> Option<u32> {
	input
		.trim()
		.strip_prefix("Test: divisible by ")?
		.parse()
		.ok()
}

fn parse_action(input: &str) -> Option<usize> {
	input
		.trim_start_matches(|c| c != ':')
		.strip_prefix(": throw to monkey ")?
		.parse()
		.ok()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
	If true: throw to monkey 2
	If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
	If true: throw to monkey 2
	If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
	If true: throw to monkey 1
	If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
	If true: throw to monkey 0
	If false: throw to monkey 1"#;

	#[test]
	fn part1_example() {
		assert_eq!(part1(INPUT), 10605);
	}

	#[test]
	fn part2_example() {
		assert_eq!(part2(INPUT), 2713310158);
	}
}
