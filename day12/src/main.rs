use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let map = Map::from_str(&input);
	println!("p1 = {}", part1(&map));
	println!("p2 = {}", part2(&map));

	Ok(())
}

fn part1(map: &Map) -> usize {
	bfs(map, 0)
}

fn part2(map: &Map) -> usize {
	bfs(map, 1)
}

fn bfs(map: &Map, needle: u8) -> usize {
	let mut seen = HashSet::new();
	let mut queue = VecDeque::new();

	seen.insert(map.e);
	queue.push_back((map.e, 0));

	while let Some((p, steps)) = queue.pop_front() {
		let e = map.sample(p);
		if e == needle {
			return steps;
		}

		let u = (p.0, p.1 - 1);
		let d = (p.0, p.1 + 1);
		let l = (p.0 - 1, p.1);
		let r = (p.0 + 1, p.1);

		for n in [u, d, l, r] {
			if map.is_valid(n) {
				let ne = map.sample(n);
				if (e == ne + 1 || ne >= e) && seen.insert(n) {
					queue.push_back((n, steps + 1));
				}
			}
		}
	}

	0
}

type Pos = (isize, isize);

struct Map {
	data: Vec<u8>,
	w: isize,
	h: isize,
	e: Pos,
}

impl Map {
	fn from_str(input: &str) -> Self {
		let parsed = input.lines().map(|l| l.bytes()).fold(
			(Vec::new(), None, 0, (0, 0)),
			|(mut v, w, h, mut e), l| {
				for (i, b) in l.enumerate() {
					if b == b'S' {
						v.push(0);
						continue;
					}
					if b == b'E' {
						e = (i as isize, h);
						v.push(b'z' - b'a' + 1);
						continue;
					}
					v.push(b - b'a' + 1);
				}
				let n = v.len();
				(v, w.or(Some(n)), h + 1, e)
			},
		);

		Self {
			data: parsed.0,
			w: parsed.1.unwrap_or(0) as isize,
			h: parsed.2,
			e: parsed.3,
		}
	}

	fn is_valid(&self, p: Pos) -> bool {
		p.0 >= 0 && p.0 < self.w && p.1 >= 0 && p.1 < self.h
	}

	fn sample(&self, p: Pos) -> u8 {
		self.data[(p.1 * self.w + p.0) as usize]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

	#[test]
	fn part1_example() {
		assert_eq!(part1(&Map::from_str(INPUT)), 31);
	}

	#[test]
	fn part2_example() {
		assert_eq!(part2(&Map::from_str(INPUT)), 29);
	}
}
