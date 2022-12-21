use std::io::{self, Read};

use anyhow::{Context, Result};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1 answer: {}", part1(&input));
    println!("Part 2 answer: {}", part2(&input));
}

fn parse_line(input: &str) -> Result<((i64, i64), (i64, i64))> {
    let mut iter = input.split_terminator(',');
    let p1 = iter.next().context("p1")?;
    let p2 = iter.next().context("p2")?;
    let mut p1i = p1.split_terminator('-');
    let mut p2i = p2.split_terminator('-');
    let a: i64 = p1i.next().context("a")?.parse()?;
    let b: i64 = p1i.next().context("b")?.parse()?;
    let c: i64 = p2i.next().context("c")?.parse()?;
    let d: i64 = p2i.next().context("d")?.parse()?;
    Ok(((a, b), (c, d)))
}

fn full_overlap(ex: ((i64, i64), (i64, i64))) -> i64 {
    let ((a, b), (c, d)) = ex;
    (c >= a && d <= b || a >= c && b <= d) as i64
}

fn any_overlap(ex: ((i64, i64), (i64, i64))) -> i64 {
    let ((a, b), (c, d)) = ex;
    if a < c {
        if c <= b {
            return 1;
        }
    } else {
        if a <= d {
            return 1;
        }
    }
    0
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|x| full_overlap(x.unwrap()))
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|x| any_overlap(x.unwrap()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 4);
    }
}
