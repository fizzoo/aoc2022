use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1 answer: {}", part1(&input));
    println!("Part 2 answer: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let v = input.chars().collect::<Vec<char>>();
    'outer: for i in 0..v.len() - 4 {
        for j in i..i + 4 {
            for k in j + 1..i + 4 {
                // println!("{}, {}, {}", i, j, k)
                if v[j] == v[k] {
                    continue 'outer;
                }
            }
        }
        return i+4
    }
    unimplemented!()
}

fn part2(input: &str) -> usize {
    let v = input.chars().collect::<Vec<char>>();
    'outer: for i in 0..v.len() - 14 {
        for j in i..i + 14 {
            for k in j + 1..i + 14 {
                // println!("{}, {}, {}", i, j, k)
                if v[j] == v[k] {
                    continue 'outer;
                }
            }
        }
        return i+14
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
