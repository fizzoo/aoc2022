use std::{
    collections::HashSet,
    io::{self, Read},
    str::Lines,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1 answer: {}", part1(&input));
    println!("Part 2 answer: {}", part2(&input));
}

fn letter_prio(input: char) -> i64 {
    match input {
        'a'..='z' => 1 + (input as i64 - 'a' as i64),
        'A'..='Z' => 27 + (input as i64 - 'A' as i64),
        _ => unimplemented!(),
    }
}

fn rucksack_prio(input: &str) -> i64 {
    assert_eq!(input.len() % 2, 0);
    let left: HashSet<char> = input[..input.len() / 2].chars().collect();
    for c in input[input.len() / 2..].chars() {
        if left.contains(&c) {
            return letter_prio(c);
        }
    }
    unimplemented!();
}

fn part1(input: &str) -> i64 {
    input.lines().map(rucksack_prio).sum()
}

fn badge(ls: &mut Lines) -> Option<i64> {
    // This should be possible with just one mutable hashset and some
    // temporary ones...
    let b1: HashSet<char> = ls.next()?.chars().collect();
    let b2: HashSet<char> = ls.next()?.chars().collect();
    let b3: HashSet<char> = ls.next()?.chars().collect();
    let i1: HashSet<char> = b1.intersection(&b2).copied().collect();
    let i2: HashSet<char> = i1.intersection(&b3).copied().collect();
    assert!(i2.len() == 1);
    Some(letter_prio(i2.iter().next().copied()?))
}

fn part2(input: &str) -> i64 {
    let mut ls: Lines = input.lines();
    let mut acc = 0;
    loop {
        match badge(&mut ls) {
            None => return acc,
            Some(c) => acc += c,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 70);
    }
}
