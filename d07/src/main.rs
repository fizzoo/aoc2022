use std::{
    io::{self, Read},
    str::FromStr,
};

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1 answer: {}", part1(&input));
    println!("Part 2 answer: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq)]
struct NoParseError;

#[derive(Debug, PartialEq, Eq)]
enum Cd {
    Home,
    Up,
    Down(String),
}
impl FromStr for Cd {
    type Err = NoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^\$ cd (/|..|[a-zA-Z][a-zA-Z.]*)$").unwrap();
        let target = re
            .captures(s)
            .ok_or(NoParseError)?
            .get(1)
            .ok_or(NoParseError)?
            .as_str();

        match target {
            "/" => Ok(Cd::Home),
            ".." => Ok(Cd::Up),
            x => Ok(Cd::Down(String::from(x))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cmd {
    Cd(Cd),
    Ls,
}
impl FromStr for Cmd {
    type Err = NoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = s.parse::<Cd>() {
            return Ok(Cmd::Cd(x));
        }
        let re = Regex::new(r"^\$ ls$").map_err(|_| NoParseError)?;
        if re.is_match(s) {
            Ok(Cmd::Ls)
        } else {
            Err(NoParseError)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct File(String, i64);

#[derive(Debug, PartialEq, Eq)]
enum Output {
    Dir(String),
    File(File),
}
impl FromStr for Output {
    type Err = NoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(dir|[0-9]+) ([a-zA-Z][a-zA-Z.]*)$").unwrap();
        let m = re.captures(s).ok_or(NoParseError)?;
        let w1 = m.get(1).ok_or(NoParseError)?.as_str();
        let w2 = m.get(2).ok_or(NoParseError)?.as_str();

        match w1 {
            "dir" => Ok(Output::Dir(w2.to_string())),
            n => Ok(Output::File(File(
                w2.to_string(),
                n.parse().map_err(|_| NoParseError)?,
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Cmd(Cmd),
    Output(Output),
}
impl FromStr for Line {
    type Err = NoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = s.parse::<Cmd>() {
            return Ok(Line::Cmd(x));
        }
        if let Ok(x) = s.parse::<Output>() {
            return Ok(Line::Output(x));
        }
        Err(NoParseError)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DirTree {
    Branch(String, Vec<DirTree>),
    Leaf(File),
}
impl DirTree {
    fn name(&self) -> &str {
        match self {
            Self::Branch(n, _) => &n,
            Self::Leaf(f) => &f.0,
        }
    }
    fn add_top(&mut self, info: Output) {
        if let Self::Branch(cwd, v) = self {
            match info {
                Output::Dir(name) => match v.iter().find(|x| x.name() == name) {
                    Some(_) => (),
                    None => v.push(DirTree::Branch(name.clone(), Vec::new())),
                },
                Output::File(f) => match v.iter().find(|x| x.name() == f.0) {
                    Some(_) => (),
                    None => v.push(DirTree::Leaf(f)),
                },
            }
        } else {
            panic!("add on a leaf")
        }
    }
    fn add(&mut self, path: &[&str], info: Output) {
        if let Self::Branch(cwd, v) = self {
            match path {
                [] => self.add_top(info),
                s => match v.iter_mut().find(|x| &x.name() == s.last().unwrap()) {
                    Some(d) => d.add(&s[..s.len() - 1], info),
                    None => panic!("pathing into nonexistant dir"),
                },
            }
        } else {
            panic!("add on a leaf")
        }
    }
    fn cumsize(&self) -> i64 {
        match self {
            Self::Leaf(File(_, size)) => *size,
            Self::Branch(_, v) => v.iter().map(Self::cumsize).sum(),
        }
    }
    fn cumsize_lt(&self) -> i64 {
        match self {
            Self::Leaf(_) => 0,
            Self::Branch(_, v) => {
                let cs = self.cumsize();
                (if cs <= 100000 { cs } else { 0 }) + v.iter().map(Self::cumsize_lt).sum::<i64>()
            }
        }
    }
    fn smallest_cumsize_above(&self, minimum: i64) -> i64 {
        match self {
            Self::Leaf(_) => i64::max_value(),
            Self::Branch(_, v) => {
                let cs = self.cumsize();
                let rec_mini = v
                    .iter()
                    .map(|x| x.smallest_cumsize_above(minimum))
                    .min()
                    .unwrap_or(i64::max_value());
                rec_mini.min(if cs >= minimum { cs } else { i64::max_value() })
            }
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut d = DirTree::Branch("root".to_string(), Vec::new());
    let mut path: Vec<String> = Vec::new();
    for l in input.lines() {
        let line = l.parse::<Line>().unwrap();
        // println!("{:?}, {:?}", l, line);
        match line {
            Line::Cmd(Cmd::Cd(Cd::Home)) => path = Vec::new(),
            Line::Cmd(Cmd::Cd(Cd::Up)) => {
                path.pop();
                ()
            }
            Line::Cmd(Cmd::Cd(Cd::Down(name))) => path.push(name),
            Line::Cmd(Cmd::Ls) => (),
            Line::Output(output) => {
                let p: Vec<&str> = path.iter().rev().map(String::as_str).collect();
                d.add(&p, output)
            }
        }
        // println!("{:?}, {:?}", path, d);
    }

    d.cumsize_lt()
}

fn part2(input: &str) -> i64 {
    let mut d = DirTree::Branch("root".to_string(), Vec::new());
    let mut path: Vec<String> = Vec::new();
    for l in input.lines() {
        let line = l.parse::<Line>().unwrap();
        // println!("{:?}, {:?}", l, line);
        match line {
            Line::Cmd(Cmd::Cd(Cd::Home)) => path = Vec::new(),
            Line::Cmd(Cmd::Cd(Cd::Up)) => {
                path.pop();
                ()
            }
            Line::Cmd(Cmd::Cd(Cd::Down(name))) => path.push(name),
            Line::Cmd(Cmd::Ls) => (),
            Line::Output(output) => {
                let p: Vec<&str> = path.iter().rev().map(String::as_str).collect();
                d.add(&p, output)
            }
        }
        // println!("{:?}, {:?}", path, d);
    }

    d.smallest_cumsize_above(d.cumsize() - (70000000 - 30000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cd_fromstr() {
        assert_eq!(str::parse::<Cd>("$ cd .."), Ok(Cd::Up));
        assert_eq!(str::parse::<Cd>("$ cd /"), Ok(Cd::Home));
        assert_eq!(
            str::parse::<Cd>("$ cd asd"),
            Ok(Cd::Down("asd".to_string()))
        );
        assert_eq!(str::parse::<Cd>("$ c asd"), Err(NoParseError));
        assert_eq!(str::parse::<Cd>("cd asd"), Err(NoParseError));
        assert_eq!(str::parse::<Cd>("$ cd ."), Err(NoParseError));
    }
    #[test]
    fn cmd_fromstr() {
        assert_eq!(str::parse::<Cmd>("$ cd .."), Ok(Cmd::Cd(Cd::Up)));
        assert_eq!(
            str::parse::<Cmd>("$ cd asd"),
            Ok(Cmd::Cd(Cd::Down("asd".to_string())))
        );
        assert_eq!(str::parse::<Cmd>("$ ls"), Ok(Cmd::Ls));
    }
    #[test]
    fn line_fromstr() {
        assert_eq!(
            str::parse::<Line>("$ cd .."),
            Ok(Line::Cmd(Cmd::Cd(Cd::Up)))
        );
        assert_eq!(
            str::parse::<Line>("dir a"),
            Ok(Line::Output(Output::Dir("a".to_string())))
        );
    }

    #[test]
    fn dirtree_creation() {
        let mut d = DirTree::Branch("root".to_string(), Vec::new());
        d.add(&[], Output::File(File("a".to_string(), 100)));
        d.add(&[], Output::Dir("b".to_string()));
        d.add(&["b"], Output::File(File("c".to_string(), 123)));
        d.add(&["b"], Output::File(File("a".to_string(), 321)));
        assert_eq!(format!("{:?}", d), "Branch(\"root\", [Leaf(File(\"a\", 100)), Branch(\"b\", [Leaf(File(\"c\", 123)), Leaf(File(\"a\", 321))])])");
    }

    static EXAMPLE1: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn example_parses_ok() {
        for l in EXAMPLE1.lines() {
            assert!(l.parse::<Line>().is_ok());
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 24933642);
    }
}
