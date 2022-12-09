use std::collections::HashSet;
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

fn part1(input: &str) -> usize {
	solve::<2>(input)
}

fn part2(input: &str) -> usize {
	solve::<10>(input)
}

fn solve<const N: usize>(input: &str) -> usize {
	input
		.lines()
		.filter_map(try_parse_move)
		.fold(
			(HashSet::<Pos>::new(), [(0, 0); N]),
			|(mut visited, mut knots), (dir, n)| {
				for _ in 0..n {
					knots[0] = next(knots[0], dir);
					for k in 1..N {
						knots[k] = follow(knots[k - 1], knots[k]);
					}
					visited.insert(knots[N - 1]);
				}

				(visited, knots)
			})
		.0
		.len()
}

fn try_parse_move(input: &str) -> Option<(u8, usize)> {
	input
		.split_once(' ')
		.and_then(|(dir, n)| Some((dir.bytes().next()?, n.parse::<usize>().ok()?)))
}

type Pos = (isize, isize);

fn next((x, y): Pos, dir: u8) -> Pos {
	let (dx, dy) = match dir {
		b'R' => ( 1,  0),
		b'L' => (-1,  0),
		b'U' => ( 0, -1),
		b'D' => ( 0,  1),
		_ => (0, 0),
	};
	(x + dx, y + dy)
}

fn follow((hx, hy): Pos, (tx, ty): Pos) -> Pos {
	let dx = hx - tx;
	let dy = hy - ty;

	if dx.abs() <= 1 && dy.abs() <= 1 {
		(tx, ty)
	} else if dx.abs() > dy.abs() {
		(hx - signum(dx), hy)
	} else if dx.abs() < dy.abs() {
		(hx, hy - signum(dy))
	} else {
		(hx - signum(dx), hy - signum(dy))
	}
}

fn signum(x: isize) -> isize {
	match x {
		n if n < 0 => -1,
		0 => 0,
		_ => 1,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

	#[test]
	fn part1_example() {
		assert_eq!(part1(INPUT), 13);
	}

	#[test]
	fn part2_example1() {
		assert_eq!(part2(INPUT), 1);
	}

	#[test]
	fn part2_example2() {
		let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
		assert_eq!(part2(input), 36);
	}
}
