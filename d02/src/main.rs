use std::io::{self, Read};

use anyhow::{bail, Result};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1 answer: {}", part1(&input));
    println!("Part 2 answer: {}", part2(&input));
}

#[derive(Debug)]
enum Move1 {
    A,
    B,
    C,
}
#[derive(Debug)]
enum Move2 {
    X,
    Y,
    Z,
}

fn parse_line(input: &str) -> Result<(Move1, Move2)> {
    let spl: Vec<&str> = input.split_whitespace().collect();
    let m1: Move1 = match spl.get(0) {
        Some(&"A") => Move1::A,
        Some(&"B") => Move1::B,
        Some(&"C") => Move1::C,
        _ => bail!(format!("No parse ABC: {}", input)),
    };
    let m2: Move2 = match spl.get(1) {
        Some(&"X") => Move2::X,
        Some(&"Y") => Move2::Y,
        Some(&"Z") => Move2::Z,
        _ => bail!(format!("No parse XYZ: {}", input)),
    };
    Ok((m1, m2))
}

fn score1(m1: Move1, m2: Move2) -> i64 {
    match (m1, m2) {
        (Move1::A, Move2::Z) => 3,
        (Move1::B, Move2::X) => 1,
        (Move1::C, Move2::Y) => 2,
        (Move1::A, Move2::X) => 4,
        (Move1::B, Move2::Y) => 5,
        (Move1::C, Move2::Z) => 6,
        (Move1::A, Move2::Y) => 8,
        (Move1::B, Move2::Z) => 9,
        (Move1::C, Move2::X) => 7,
    }
}

fn score2(m1: Move1, m2: Move2) -> i64 {
    match (m1, m2) {
        (Move1::A, Move2::X) => 3,
        (Move1::B, Move2::X) => 1,
        (Move1::C, Move2::X) => 2,
        (Move1::A, Move2::Y) => 4,
        (Move1::B, Move2::Y) => 5,
        (Move1::C, Move2::Y) => 6,
        (Move1::A, Move2::Z) => 8,
        (Move1::B, Move2::Z) => 9,
        (Move1::C, Move2::Z) => 7,
    }
}

fn part1(input: &str) -> i64 {
    let moves = input.lines().map(|l| parse_line(l).unwrap());
    let mut cur_score = 0;
    for (m1, m2) in moves {
        cur_score += score1(m1, m2)
    }
    cur_score
}

fn part2(input: &str) -> i64 {
    let moves = input.lines().map(|l| parse_line(l).unwrap());
    let mut cur_score = 0;
    for (m1, m2) in moves {
        cur_score += score2(m1, m2)
    }
    cur_score
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &'static str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 12);
    }
}
