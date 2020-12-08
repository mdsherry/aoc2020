use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let groups = INPUT.split("\n\n");
    let counts = groups.map(|group| {
        let group_bitmap = group.lines().map(|line| {
            line.bytes().fold(0u32, |mask, value| mask | (1 << (value - b'a')))
        }).fold(0u32, |mask, value| mask | value);
        group_bitmap.count_ones()
    });
    println!("{}", counts.sum::<u32>());
}
