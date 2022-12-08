use std::collections::HashSet;
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let ctx = Context::new(&input);
	println!("p1 = {}", part1(&ctx));
	println!("p2 = {}", part2(&ctx));

	Ok(())
}

fn part1(ctx: &Context) -> usize {
	let w = ctx.w;
	let h = ctx.h;

	let lr = (1..h - 1).flat_map(|r| ctx.step(w * r, (1..w - 1).map(move |c| (r, c))));
	let rl = (1..h - 1).flat_map(|r| ctx.step(w * r + w - 1, (1..w - 1).map(move |c| (r, c)).rev()));
	let tb = (1..w - 1).flat_map(|c| ctx.step(c, (1..h - 1).map(move |r| (r, c))));
	let bt = (1..w - 1).flat_map(|c| ctx.step(w * (h - 1) + c, (1..h - 1).map(move |r| (r, c)).rev()));

	HashSet::<(usize, usize)>::from_iter(lr.chain(rl).chain(tb).chain(bt)).len() + w * h - (w - 2) * (h - 2)
}

fn part2(ctx: &Context) -> usize {
	let mut stack: Vec<usize> = Vec::new();
	let mut score = Vec::from_iter(std::iter::repeat([0; 4]).take(ctx.map.len()));

	let m = &ctx.map;
	let w = ctx.w;
	let h = ctx.h;

	for r in 0..h {
		// left to right
		for c in 0..w {
			let t = m[w * r + c];
			while let Some(x) = stack.last().copied() {
				if t < m[w * r + x] {
					break;
				}

				stack.pop();
				score[w * r + x][0] = c - x;
			}
			stack.push(c);
		}
		for x in stack.drain(..) {
			score[w * r + x][0] = w - x - 1;
		}

		// right to left
		for c in (0..w).rev() {
			let t = m[w * r + c];
			while let Some(x) = stack.last().copied() {
				if t < m[w * r + x] {
					break;
				}

				stack.pop();
				score[w * r + x][1] = x - c;
			}
			stack.push(c);
		}
		for x in stack.drain(..) {
			score[w * r + x][1] = x;
		}
	}
	for c in 0..w {
		// top to bottom
		for r in 0..h {
			let t = m[w * r + c];
			while let Some(x) = stack.last().copied() {
				if t < m[w * x + c] {
					break;
				}

				stack.pop();
				score[w * x + c][2] = r - x;
			}
			stack.push(r);
		}
		for x in stack.drain(..) {
			score[w * x + c][2] = w - x - 1;
		}
		// bottom to top
		for r in (0..h).rev() {
			let t = m[w * r + c];
			while let Some(x) = stack.last().copied() {
				if t < m[w * x + c] {
					break;
				}

				stack.pop();
				score[w * x + c][3] = x - r;
			}
			stack.push(r);
		}
		for x in stack.drain(..) {
			score[w * x + c][3] = x;
		}
	}

	score
		.into_iter()
		.map(|[l, r, d, u]| l * r * d * u)
		.max()
		.unwrap_or(0)
}

struct Context {
	map: Vec<u8>,
	w: usize,
	h: usize,
}

impl Context {
	fn new(input: &str) -> Self {
		let parsed = input.lines().map(|l| l.bytes().map(|b| b - b'0')).fold(
			(Vec::new(), None, 0),
			|(mut v, w, h), l| {
				v.extend(l);
				let n = v.len();
				(v, w.or(Some(n)), h + 1)
			},
		);
		let map = parsed.0;
		let w = parsed.1.unwrap_or(0);
		let h = parsed.2;

		Self { map, w, h }
	}

	fn step<'s>(
		&'s self,
		hi_index: usize,
		indices: impl Iterator<Item = (usize, usize)> + 's,
	) -> impl Iterator<Item = (usize, usize)> + 's
	{
		indices
			.scan(self.map[hi_index], move |hi, (r, c)| {
				let t = self.map[self.w * r + c];
				let result = Some((t > *hi).then_some((r, c)));
				*hi = (*hi).max(t);
				result
			})
			.flatten()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"30373
25512
65332
33549
35390"#;

	#[test]
	fn part1_example() {
		assert_eq!(part1(&Context::new(INPUT)), 21);
	}

	#[test]
	fn part2_example() {
		assert_eq!(part2(&Context::new(INPUT)), 8);
	}
}
