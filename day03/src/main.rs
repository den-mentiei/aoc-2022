#![feature(iter_array_chunks)]

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

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (l, r) = s.split_at(s.len() / 2);
            (bits(l) & bits(r)).trailing_zeros()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .filter_map(|c| {
            c.into_iter()
                .map(bits)
                .reduce(|l, r| l & r)
                .map(|m| m.trailing_zeros())
        })
        .sum()
}

fn bits(s: &str) -> u64 {
    s.bytes().fold(0_u64, |acc, b| acc | (1 << prio(b)))
}

fn prio(x: u8) -> u8 {
    if x.is_ascii_lowercase() {
        x - b'a' + 1
    } else {
        x - b'A' + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 70);
    }
}
