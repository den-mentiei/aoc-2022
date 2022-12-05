#![feature(iter_array_chunks)]
#![feature(map_many_mut)]

use std::collections::HashMap;
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

fn part1(input: &str) -> String {
    solve::<true>(input)
}

fn part2(input: &str) -> String {
    solve::<false>(input)
}

fn solve<const REV: bool>(input: &str) -> String {
    let (image, moves) = input.split_once(" 1").expect("Wrong input format.");
    moves
        .lines()
        .skip(2)
        .filter_map(try_parse_move)
        .try_fold(parse_stacks(image), |mut acc, (count, from, to)| {
            let [stack_from, stack_to] = acc.get_many_mut([&from, &to])?;
            if REV {
                stack_to.extend(stack_from.drain(stack_from.len() - count..).rev());
            } else {
                stack_to.extend(stack_from.drain(stack_from.len() - count..));
            }
            Some(acc)
        })
        .and_then(|m| {
            (0..m.len()).try_fold(String::with_capacity(m.len()), |mut acc, i| {
                let c = m.get(&i).and_then(|v| v.last()).copied()? as char;
                acc.push(c);
                Some(acc)
            })
        })
        .unwrap_or_default()
}

fn try_parse_move(input: &str) -> Option<(usize, usize, usize)> {
    input
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .filter_map(|s| s.parse::<usize>().ok())
        .array_chunks()
        .map(|[n, f, t]| (n, f - 1, t - 1))
        .next()
}

fn parse_stacks(input: &str) -> HashMap<usize, Vec<u8>> {
    input
        .lines()
        .rev()
        .flat_map(|s| {
            s.as_bytes()
                .iter()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| c.is_ascii_uppercase())
        })
        .fold(HashMap::new(), |mut acc, (i, &c)| {
            acc.entry(i).or_insert_with(Vec::new).push(c);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), "CMZ");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), "MCD");
    }
}
