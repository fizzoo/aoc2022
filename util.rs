
fn split_line<T>(input: &str) -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    input
        .split_terminator(' ')
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}

fn parse_space_sep_lines<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let res = input.lines().map(|l| split_line(l)).collect();
    println!("parse on {:?} -> {:?}", input, res);
    res
}
