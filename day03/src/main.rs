#![feature(iter_array_chunks)]

use std::io::{self, Read};

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
	input
		.trim()
		.lines()
		.map(|l| l.trim())
		.map(|s| {
			let (l, r) = s.split_at(s.len() / 2);
			let bits_l = bits(l);
			let bits_r = bits(r);
			(bits_l & bits_r).trailing_zeros() as u64
		})
		.sum()
}

fn part2(input: &str) -> u64 {
	input
		.trim()
		.lines()
		.map(|l| l.trim())
		.array_chunks()
		.map(|[a, b, c]| {
			let bits_a = bits(a);
			let bits_b = bits(b);
			let bits_c = bits(c);
			(bits_a & bits_b & bits_c).trailing_zeros() as u64
		})
		.sum()
}

fn bits(s: &str) -> u64 {
	s
		.bytes()
		.fold(0, |acc, b| acc | (1_u64 << prio(b)))
}

fn prio(x: u8) -> u8 {
	if x.is_ascii_lowercase() {
		x - b'a' + 1
	} else {
		x - b'A' + 27
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = r#"
			vJrwpWtwJgWrhcsFMMfFFhFp
			jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
			PmmdzqPrVvPwwTWBwg
			wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
			ttgJtRGJQctTZtZT
			CrZsJsPPZsGzwwsLwLmpwMDw"#;
		assert_eq!(part1(&input), 157);
	}

	#[test]
	fn part2_example() {
		let input = r#"
			vJrwpWtwJgWrhcsFMMfFFhFp
			jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
			PmmdzqPrVvPwwTWBwg
			wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
			ttgJtRGJQctTZtZT
			CrZsJsPPZsGzwwsLwLmpwMDw"#;
		assert_eq!(part2(&input), 70);
	}
}
