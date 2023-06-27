#![feature(array_windows)]

use std::collections::HashSet;
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let p1 = part1(&input);
	let p2 = part2(&input);
	println!("p1 = {p1}");
	println!("p2 = {p2}");

	Ok(())
}

fn part1(input: &str) -> usize {
	let (mut cave, depth) = parse(input);
	pour(&mut cave, depth, false)
}

fn part2(input: &str) -> usize {
	let (mut cave, depth) = parse(input);
	pour(&mut cave, depth, true)
}

fn parse(input: &str) -> (Cave, i16) {
	let paths = input
		.trim()
		.lines()
		.map(|l| parse_path(l.trim()));

	let mut cave = HashSet::new();

	let mut depth = i16::MIN;
	for path in paths {
		for &[(x0, y0), (x1, y1)] in path.array_windows() {
			depth = depth.max(y0).max(y1);

			if x0 == x1 {
				for y in y0.min(y1)..=y0.max(y1) {
					cave.insert((x0, y));
				}
			} else {
				for x in x0.min(x1)..=x0.max(x1) {
					cave.insert((x, y0));
				}
			}
		}
	}

	(cave, depth)
}

fn pour(
	cave: &mut HashSet<(i16, i16)>,
	depth: i16,
	has_floor: bool,
) -> usize {
	let start = (500, 0);
	for times in 1.. {
		let (mut x, mut y) = start;
		let mut done = false;
		while y <= depth && !done {
			done = true;
			for (nx, ny) in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
				if !cave.contains(&(nx, ny)) {
					x = nx;
					y = ny;
					done = false;
					break;
				}
			}
		}
		if done && (x, y) == start {
			return times;
		}
		if has_floor || done {
			cave.insert((x, y));
		} else {
			return times - 1;
		}
	}
	0
}

type Point = (i16, i16);
type Cave  = HashSet<Point>;

fn parse_path(input: &str) -> Vec<Point> {
	input
		.split(" -> ")
		.filter_map(|s| {
			let (x, y) = s.split_once(',')?;
			let x = x.parse::<i16>().ok()?;
			let y = y.parse::<i16>().ok()?;
			Some((x, y))
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

	#[test]
	fn test_part1() {
		let p1 = part1(INPUT);
		assert_eq!(p1, 24);
	}

	#[test]
	fn test_part2() {
		let p2 = part2(INPUT);
		assert_eq!(p2, 93);
	}
}
