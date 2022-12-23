use std::io::{self, Read};

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1 answer: {}", part1(&input));
    println!("Part 2 answer: {}", part2(&input));
}

struct State(Vec<Vec<char>>);

impl State {
    fn print(&self) {
        for v in &self.0 {
            for e in v {
                print!("{} ", e);
            }
            println!("");
        }
    }

    fn mov1(&mut self, mv: Move) {
        for _ in 0..mv.count {
            let popped = self.0[mv.from - 1].pop().unwrap().clone();
            self.0[mv.to - 1].push(popped);
        }
    }

    fn mov2(&mut self, mv: Move) {
        let from = &mut self.0[mv.from - 1];
        let cpd: Vec<char> = from[from.len() - mv.count..].iter().copied().collect();
        from.truncate(from.len() - mv.count);
        self.0[mv.to - 1].extend(cpd);
    }
}

fn parse_state(input: Vec<&str>) -> State {
    let mut res = State(Vec::new());
    for l in input {
        for (x, chunk) in l.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if chunk[0] == '[' {
                while x >= res.0.len() {
                    res.0.push(Vec::new());
                }
                res.0[x].push(chunk[1])
            }
        }
    }
    for v in res.0.iter_mut() {
        v.reverse();
    }
    res
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_move(input: &str) -> Option<Move> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let capture = re.captures(input)?;
    // println!("{:?}", capture);
    Some(Move {
        count: capture.get(1)?.as_str().parse().unwrap(),
        from: capture.get(2)?.as_str().parse().unwrap(),
        to: capture.get(3)?.as_str().parse().unwrap(),
    })
}

fn parse_all(input: &str) -> (State, Vec<Move>) {
    let start_state = parse_state(input.lines().take_while(|x| !x.is_empty()).collect());
    let mut vec: Vec<Move> = Vec::new();
    for mv in input.lines().skip_while(|x| !x.is_empty()).skip(1) {
        vec.push(parse_move(mv).unwrap());
    }
    (start_state, vec)
}
fn part1(input: &str) -> String {
    let (mut state, moves) = parse_all(input);
    println!("{}", "starting state");
    state.print();

    for mv in moves {
        state.mov1(mv);
        println!("{}", "new move");
        state.print();
    }

    let mut res = String::new();
    for v in state.0 {
        res.push(*v.last().unwrap());
    }
    res
}

fn part2(input: &str) -> String {
    let (mut state, moves) = parse_all(input);
    println!("{}", "starting state");
    state.print();

    for mv in moves {
        state.mov2(mv);
        println!("{}", "new move");
        state.print();
    }

    let mut res = String::new();
    for v in state.0 {
        res.push(*v.last().unwrap());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), "MCD");
    }
}
