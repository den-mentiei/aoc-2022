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
    count(input, |(b0, e0, b1, e1)| b0 <= b1 && e0 >= e1 || b0 >= b1 && e0 <= e1)
}

fn part2(input: &str) -> usize {
    count(input, |(b0, e0, b1, e1)| b0 <= e1 && e0 >= b1)
}

fn count<F>(input: &str, f: F) -> usize
where
    F: Fn(&(u32, u32, u32, u32)) -> bool,
{
    input
        .lines()
        .filter_map(try_parse_intervals)
        .filter(f)
        .count()
}

fn try_parse_intervals(s: &str) -> Option<(u32, u32, u32, u32)> {
    let mut parts = s.split([',', '-']);
    let b0 = parts.next()?.parse().ok()?;
    let e0 = parts.next()?.parse().ok()?;
    let b1 = parts.next()?.parse().ok()?;
    let e1 = parts.next()?.parse().ok()?;
    Some((b0, e0, b1, e1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 4);
    }
}
