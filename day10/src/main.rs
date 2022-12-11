#![feature(iter_repeat_n)]

use std::io::{self, Read};
use std::iter;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	println!("p1 = {}", part1(&input));
	println!("p2:");
	println!("{}", part2(&input));

	Ok(())
}

const W: usize = 40;
const H: usize = 6;

fn part1(input: &str) -> isize {
	signal(input)
		.map(|(x, i)| (x, i + 1))
		.map(|(x, i)| match i {
			i if i % W == 20 => x * (i as isize),
			_ => 0,
		})
		.sum()
}

fn part2(input: &str) -> String {
	signal(input)
		.take(W * H)
		.map(|(s, c)| {
			let x = (c % W) as isize;
			if s - 1 <= x && x <= s + 1 {
				'█'
			} else {
				'.'
			}
		})
		.enumerate()
		.flat_map(|(i, p)| {
			if i % W == 0 {
				Either::Left(['\n', p].into_iter())
			} else {
				Either::Right(iter::once(p))
			}
		})
		.collect()
}

fn signal(input: &str) -> impl Iterator<Item = (isize, usize)> + '_ {
	input
		.lines()
		.map(|l| l.split_once(' ').and_then(|(_, v)| v.parse::<isize>().ok()))
		.scan(1, |s, c| {
			let x = *s;
			if let Some(v) = c {
				*s = x + v;
				Some(iter::repeat_n(x, 2))
			} else {
				Some(iter::repeat_n(x, 1))
			}
		})
		.flatten()
		.zip(0..)
}

enum Either<L, R> {
	Left(L),
	Right(R),
}

impl<L, R> Iterator for Either<L, R>
where
	L: Iterator,
	R: Iterator<Item = L::Item>,
{
	type Item = <L as Iterator>::Item;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Either::Left(l) => l.next(),
			Either::Right(r) => r.next(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

	#[test]
	fn part1_example() {
		assert_eq!(part1(INPUT), 13140);
	}

	#[test]
	fn part2_example() {
		let answer = r#"██..██..██..██..██..██..██..██..██..██..
███...███...███...███...███...███...███.
████....████....████....████....████....
█████.....█████.....█████.....█████.....
██████......██████......██████......████
███████.......███████.......███████....."#;

		assert_eq!(part2(INPUT).trim(), answer);
	}
}
