
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
        let count = passwd.chars().filter(|c| c == char).count();
        count >= *low && count <= *high
    }).count();
    println!("{}", count);
}
