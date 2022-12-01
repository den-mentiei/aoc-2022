use std::io::{self, Read};
use std::mem::swap;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let i = input
		.trim()
		.lines()
		.fold((Inventory::new(), 0), |(mut i, a), line| {
			if line.is_empty() {
				i.add(a);
				(i, 0)
			} else {
				(i, a + line.parse::<i64>().expect("Failed to parse calories."))
			}
		})
		.0;
	println!("p1 = {}", i.max());
	println!("p2 = {}", i.total());

	Ok(())
}

struct Inventory([i64; 3]);

impl Inventory {
	fn new() -> Self {
		Inventory([0; 3])
	}

	fn total(&self) -> i64 {
		self.0.into_iter().sum()
	}

	fn max(&self) -> i64 {
		self.0[2]
	}

	fn add(&mut self, x: i64) {
		if x > self.0[0] {
			let mut a = x;
			let mut b = self.0[1];
			let mut c = self.0[2];

			if a > b {
				swap(&mut a, &mut b);
			}
			if b > c {
				swap(&mut b, &mut c);
			}
			if a > b {
				swap(&mut a, &mut b);
			}

			self.0[0] = a;
			self.0[1] = b;
			self.0[2] = c;
		}
	}
}
