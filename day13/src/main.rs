use std::cmp::Ordering;
use std::io::{self, Read};
use std::iter::Peekable;
use std::str::Bytes;

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
	input
		.split("\n\n")
		.filter_map(|l| l.split_once('\n'))
		.map(|(l, r)| {
			let l = Packet::from_str(l);
			let r = Packet::from_str(r);
			l.cmp(&r)
		})
		.enumerate()
		.filter(|&(_, o)| o == Ordering::Less)
		.map(|(i, _)| i + 1)
		.sum()
}

fn part2(input: &str) -> usize {
	let mut packets = input
		.lines()
		.filter(|l| !l.trim().is_empty())
		.map(|l| Packet::from_str(l.trim()))
		.collect::<Vec<_>>();

	let d1 = Packet::from_str("[[2]]");
	let d2 = Packet::from_str("[[6]]");

	packets.push(d1.clone());
	packets.push(d2.clone());
	packets.sort_unstable();

	packets
		.iter()
		.enumerate()
		.filter(|&(_, p)| p == &d1 || p == &d2)
		.map(|(i, _)| i + 1)
		.product()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
	Literal(u8),
	List(Vec<Packet>),
}

impl Packet {
	fn from_str(input: &str) -> Self {
		let mut bytes = input.bytes().peekable();
		let c = bytes.next().expect("(input shouldn't be empty");
		if c == b'[' {
			Self::parse_list(&mut bytes)
		} else {
			panic!("top-level item should be a list")
		}
	}

	fn parse_list(bytes: &mut Peekable<Bytes>) -> Self {
		let mut items = Vec::new();
		while let Some(c) = bytes.peek() {
			match c {
				b'0'..=b'9' => {
					items.push(Self::parse_value(bytes));
				},
				b'[' => {
					bytes.next();
					items.push(Self::parse_list(bytes));
				},
				b']' => {
					bytes.next();
					break;
				},
				b',' => {
					bytes.next();
				},
				_ => panic!("unknown charatcer in packet"),
			}
		}
		Self::List(items)
	}

	fn parse_value(bytes: &mut Peekable<Bytes>) -> Self {
		let mut value = 0;
		while let Some(c) = bytes.next_if(|&c| c.is_ascii_digit()) {
			value = value * 10 + c - b'0';
		}
		Self::Literal(value)
	}
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		use Packet::*;
		match (self, other) {
			(Literal(x), Literal(y)) => x.partial_cmp(y),
			(List(x), List(y)) => {
				let mut xs = x.iter();
				let mut ys = y.iter();
				loop {
					match (xs.next(), ys.next()) {
						(None, None)    => break Some(Ordering::Equal),
						(None, Some(_)) => break Some(Ordering::Less),
						(Some(_), None) => break Some(Ordering::Greater),
						(Some(x), Some(y)) => match x.partial_cmp(y) {
							Some(Ordering::Equal) => continue,
							c => break c,
						},
					}
				}
			},
			(Literal(x), List(_)) => {
				List(vec![Literal(*x)]).partial_cmp(other)
			},
			(List(_), Literal(y)) => {
				self.partial_cmp(&List(vec![Literal(*y)]))
			},
		}
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

	#[test]
	fn part1_example() {
		assert_eq!(part1(INPUT), 13);
	}

	#[test]
	fn part2_example() {
		assert_eq!(part2(INPUT), 140);
	}
}
