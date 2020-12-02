
use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut numbers: HashSet<_> = INPUT.split("\n").filter(|s| !s.is_empty()).map(|s| s.trim().parse::<u32>().unwrap()).collect();
    for &number in &numbers {
        if numbers.contains(&(2020 - number)) {
            println!("{} * {} = {}", number, 2020 - number, number * (2020 - number));
            break;
        }
    }    
}
