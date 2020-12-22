use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let (p1, p2) = input.split("\n\n").collect_tuple().unwrap();
    let mut p1: VecDeque<_> = p1.lines().skip(1).map(|n| n.parse::<u8>().unwrap()).collect();
    let mut p2: VecDeque<_> = p2.lines().skip(1).map(|n| n.parse::<u8>().unwrap()).collect();
    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    let winner = if p1.is_empty() { p2 } else { p1 };
    let score = winner.into_iter().rev().enumerate().fold(0, |acc, (i, card)| acc + (i + 1) * card as usize);
    println!("{}", score);
}
