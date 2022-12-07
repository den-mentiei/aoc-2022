use std::io::{self, Read};
use std::collections::HashMap;
use std::path::PathBuf;

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
	traverse(input).into_values().filter(|&s| s <= 100000).sum()
}

fn part2(input: &str) -> usize {
	let sizes = traverse(input);
	let total = sizes.get(&PathBuf::new()).copied().unwrap_or(0);
	sizes
		.into_values()
		.filter(|s| 70000000 - (total - s) >= 30000000)
		.min()
		.unwrap_or(total)
}

fn traverse(input: &str) -> HashMap<PathBuf, usize> {
	input
		.lines()
		.try_fold(
			(HashMap::new(), PathBuf::new()),
			|(mut sizes, mut cwd), l| {
				if let Some(dir) = l.strip_prefix("$ cd ") {
					match dir {
						"/"  => { cwd = PathBuf::new(); },
						".." => { cwd.pop(); },
						dir  => { cwd.push(dir); },
					};
				} else if l.starts_with(|c: char| c.is_ascii_digit()) {
					let size = l.split_once(' ')?.0.parse::<usize>().ok()?;
					let mut p = cwd.clone();
					loop {
						*sizes.entry(p.clone()).or_insert(0) += size;
						if !p.pop() {
							break;
						}
					}
				}
				Some((sizes, cwd))
			},
		)
		.unwrap_or_default()
		.0
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

	#[test]
	fn part1_example() {
		assert_eq!(part1(INPUT), 95437);
	}

	#[test]
	fn part2_example() {
		assert_eq!(part2(INPUT), 24933642);
	}
}
