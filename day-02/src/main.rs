mod part1;

use regex::Regex;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let re = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
    let count = INPUT.lines().map(|s| {
        let caps = re.captures(s).unwrap();
        let low = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let high = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let char = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let passwd = caps.get(4).unwrap().as_str().to_owned();
        (low, high, char, passwd)
    }).filter(|(low, high, char, passwd)| {
        (passwd.as_bytes()[*low - 1] == *char as u8) ^ (passwd.as_bytes()[*high - 1] == *char as u8)
    }).count();
    println!("{}", count);
}
