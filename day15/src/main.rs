use std::io::{self, Read};
use std::collections::HashSet;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let data = parse(&input);
	println!("p1 = {}", part1(&data, 2_000_000));
	println!("p2 = {}", part2::<4_000_000>(&data));

	Ok(())
}

fn part1(data: &[(Point, Point)], row: i32) -> i64 {
	let mut uniq = HashSet::new();

	for (sensor, beacon) in data {
		let d = distance(*sensor, *beacon);

		for dx in 0.. {
			if dx + (sensor.1 - row).abs() > d {
				break;
			} else {
				uniq.insert(sensor.0 + dx);
				uniq.insert(sensor.0 - dx);
			}
		}
	}

	for (_, beacon) in data {
		if beacon.1 == row {
			uniq.remove(&beacon.0);
		}
	}

	uniq.len() as i64
}

fn part2<const N: i64>(data: &[(Point, Point)]) -> i64 {
	let diamonds: Vec<_> = data
		.iter()
		.map(|(s, b)| (s, distance(*s, *b)))
		.collect();

	let mut ascends: Vec<i32> = diamonds
		.iter()
		.flat_map(|(s, d)| {
			let x0 = s.0 - d - 1;
			let x1 = s.0 + d + 1;
			let y  = s.1;
			// Ascending line: y = k, x = k + c
			// x0 = k + c -> c = x0 - y
			[x0 - y, x1 - y]
		})
		.collect();
	ascends.sort();

	let mut descends: Vec<i32> = diamonds
		.iter()
		.flat_map(|(s, d)| {
			let x0 = s.0 - d - 1;
			let x1 = s.0 + d + 1;
			let y  = s.1;
			// Descending line: y = -k, x = k + c
			// x0 = k + c -> c = x0 + y
			[x0 + y, x1 + y]
		})
		.collect();
	descends.sort();

	let ascends  = keep_dupes(ascends);
	let descends = keep_dupes(descends);

	for asc in ascends {
		for desc in &descends {
			let p = intersect_diagonals(asc, *desc);
			if p.0 < 0 || p.0 > N as i32 || p.1 < 0 || p.1 > N as i32 {
				continue;
			}

			if !diamonds.iter().any(|(s, d)| distance(**s, p) <= *d) {
				return p.0 as i64 * 4000000 as i64 + p.1 as i64;
			}
		}
	}

	-1
}

fn intersect_diagonals(ascend_x0: i32, descend_x0: i32) -> Point {
	let a = ascend_x0;
	let d = descend_x0;
	// Intersection is at a + k == d + n where k == -n
	// -> a + k   = d - k
	// -> a + 2*k = d
	let k = (d - a) / 2;
	(a + k, k)
}

fn keep_dupes(xs: Vec<i32>) -> Vec<i32> {
	let mut xs: Vec<i32> = xs
		.windows(2)
		.filter_map(|w| match w {
			[a, b] if a == b => Some(*a),
			_                => None,
		})
		.collect();
	xs.dedup();
	xs
}

fn parse(input: &str) -> Vec<(Point, Point)> {
	input
		.lines()
		.filter_map(parse_line)
		.collect()
}

fn parse_line(s: &str) -> Option<(Point, Point)> {
	let (sensor, beacon) = s.split_once(": closest beacon is at x=")?;
	let (sx, sy) = sensor.strip_prefix("Sensor at x=")?.split_once(", y=")?;
	let (bx, by) = beacon.split_once(", y=")?;

	let s = (sx.parse().ok()?, sy.parse().ok()?);
	let b = (bx.parse().ok()?, by.parse().ok()?);

	Some((s, b))
}

type Point = (i32, i32);

fn distance((ax, ay): Point, (bx, by): Point) -> i32 {
	(ax - bx).abs() + (ay - by).abs()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

	#[test]
	fn test_p1() {
		let data = parse(&INPUT);
		assert_eq!(part1(&data, 10), 26);
	}

	#[test]
	fn test_p2() {
		let data = parse(&INPUT);
		assert_eq!(part2::<20>(&data), 56000011);
	}
}
