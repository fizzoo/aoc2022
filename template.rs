use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1 answer: {}", part1(&input));
    println!("Part 2 answer: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    1
}

fn part2(input: &str) -> i64 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(""), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 2);
    }
}
