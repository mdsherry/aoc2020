use std::str::FromStr;

pub fn parse_lines<'a, T>(lines: &'a str) -> impl Iterator<Item=T> + 'a where T: FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    lines.split("\n").filter(|s| !s.is_empty()).map(|s| s.trim().parse::<T>().unwrap())
}