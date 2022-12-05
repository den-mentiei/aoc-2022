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
        .lines()
        .filter_map(|l| l.split_once(' '))
        .try_fold((0, 0), |(mut s1, mut s2), (l, r)| {
            let l = (l.as_bytes().first()? - b'A') as i32 + 1;
            let r = (r.as_bytes().first()? - b'X') as i32 + 1;

            s1 += score(l, r);
            s2 += score(l, 1 + (l + r) % 3);

            Some((s1, s2))
        })
        .expect("No moves parsed.")
}

fn score(l: i32, r: i32) -> i32 {
    (4 + r - l) % 3 * 3 + r
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn part1_example() {
        assert_eq!(solve(INPUT).0, 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve(INPUT).1, 12);
    }
}
