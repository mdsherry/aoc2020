mod part1;
use std::ops::Bound::{Included, Excluded};
use std::collections::BTreeSet;
use util::parse_lines;
static INPUT: &str = include_str!("input.txt");

fn main() {
    let numbers: BTreeSet<_> = parse_lines::<u32>(INPUT).collect();
    'outer: for &number in &numbers {
        for &other_number in numbers.range((Excluded(number), Included(2020 - number))) {
            let sum = number + other_number;
            if numbers.contains(&(2020 - sum)) {
                println!("{} * {} * {} = {}", number, other_number, sum, number * other_number * (2020 - sum));
                break 'outer;
            }
        }
    }    
}
