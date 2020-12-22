use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    P1,
    P2
}

fn combat(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>) -> (Player, VecDeque<u8>) {
    let mut seen = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        let s1: Vec<_> = p1.iter().copied().collect();
        let s2: Vec<_> = p2.iter().copied().collect();
        if !seen.insert((s1, s2)) {
            return (Player::P1, p1);
        }
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if p1.len() >= c1 as usize && p2.len() >= c2 as usize {
            let p1_sub = p1.iter().copied().take(c1 as usize).collect();
            let p2_sub = p2.iter().copied().take(c2 as usize).collect();
            let (winner, _) = combat(p1_sub, p2_sub);
            if winner == Player::P1 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        } else {
            if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }
    }
    if p1.is_empty() { (Player::P2, p2) } else { (Player::P1, p1) }
}

fn main() {
    let input = include_str!("input.txt");
    let (p1, p2) = input.split("\n\n").collect_tuple().unwrap();
    let p1: VecDeque<_> = p1.lines().skip(1).map(|n| n.parse::<u8>().unwrap()).collect();
    let p2: VecDeque<_> = p2.lines().skip(1).map(|n| n.parse::<u8>().unwrap()).collect();
    let (_, winner) = combat(p1, p2);
    // let winner = if p1.is_empty() { p2 } else { p1 };
    let score = winner.into_iter().rev().enumerate().fold(0, |acc, (i, card)| acc + (i + 1) * card as usize);
    println!("{}", score);
}
