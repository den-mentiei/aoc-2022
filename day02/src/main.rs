use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let r = solve(&input);
	println!("p1 = {}", r.0);
	println!("p2 = {}", r.1);

	Ok(())
}

fn solve(input: &str) -> (i32, i32) {
	input
		.trim()
		.lines()
		.map(|l| {
			let mut parts = l.trim().split(' ');
			let l = parts.next().expect("Failed to parse the opponent move.");
			let r = parts.next().expect("Failed to parse my move.");
			(l, r)
		})
		.try_fold((0, 0), |(mut s1, mut s2), (l, r)| {
			let l = (l.bytes().nth(0)? - b'A') as i32;
			let r = (r.bytes().nth(0)? - b'X') as i32;

			s1 += r + 1 + (3 * ((r - l + 4) % 3));
			s2 += r * 3 + (l + r + 2) % 3 + 1;

			Some((s1, s2))
		})
		.expect("No moves parsed.")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = r#"
			A Y
			B X
			C Z"#;
		assert_eq!(solve(&input).0, 15);
	}

	#[test]
	fn part2_example() {
		let input = r#"
			A Y
			B X
			C Z"#;
		assert_eq!(solve(&input).1, 12);
	}
}
