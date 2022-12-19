use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1 answer: {}", part1(&input));
    println!("Part 2 answer: {}", part2(&input));
}

fn split_line(input: &str) -> Vec<i64> {
    println!(
        "split_line on [{:?}] has split len [{:?}]",
        input,
        input.split_terminator(' ').collect::<Vec<&str>>().len()
    );
    input
        .split_terminator(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    let res = input.lines().map(|l| split_line(l)).collect();
    println!("parse on {:?} -> {:?}", input, res);
    res
}

fn part1(input: &str) -> i64 {
    let v = parse(input);
    let mut max_elf = 0;
    let mut cur_elf = 0;
    // this is awful (my first time doing rust in a while), how to generically group sequences of not-empty vecs?
    for l in v {
        if l.is_empty() {
            cur_elf = 0;
        }
        for cals in l {
            cur_elf += cals;
        }
        if cur_elf > max_elf {
            max_elf = cur_elf;
        }
    }
    max_elf
}

fn part2(input: &str) -> i64 {
    let v = parse(input);
    let mut elves: Vec<i64> = [0].to_vec();
    for l in v {
        if l.is_empty(){
            elves.push(0);
        }
        for cals in l{
            *elves.last_mut().unwrap() += cals;
        }
    }
    elves.sort_by(|a, b| b.partial_cmp(a).unwrap());
    elves[0] + elves[1] + elves[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            ),
            24000
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            ),
            45000
        );
    }
}
