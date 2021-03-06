mod part1;

static INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Debug)]
enum Splits {
    Front,
    Back,
    Left,
    Right,
}
use std::collections::HashSet;

use Splits::*;
impl Splits {
    fn from(c: char) -> Self {
        match c {
            'F' => Front,
            'B' => Back,
            'L' => Left,
            'R' => Right,
            _ => panic!("Invalid split {}", c),
        }
    }
    fn split(self, bottom: u8, top: u8) -> (u8, u8) {
        let delta = ((top + 1) - bottom) / 2;
        match self {
            Front | Left => (bottom, delta + bottom-  1),
            Back | Right => (bottom + delta, top),
        }
    }
}

fn main() {
    
    let seats: HashSet<_> = INPUT.lines().map(|line| {
        let row = &line[..7];
        let (top, bottom) = row.chars()
            .map(|c| Splits::from(c))
            .fold((0u8, 127u8), |(top, bottom), split| split.split(top, bottom));
        assert_eq!(top, bottom);
        let row = bottom;
        let col = &line[7..];
        let (left, right) = col.chars()
            .map(|c| Splits::from(c))
            .fold((0u8, 7u8), |(top, bottom), split| split.split(top, bottom));
        assert_eq!(left, right);
        let col = left;
        (row, col)
    }).collect();
    // let expected_rows: HashSet<_> = (0..127).collect();
    // let actual_rows: HashSet<_> = seats.iter().map(|x| x.0).collect();
    // let mut missing: HashSet<_> = expected_rows.difference(&actual_rows).collect();
    let expected_seats: HashSet<_> = (6..113).flat_map(|row| (0..7).map(move |col| (row, col))).collect();
    for missing in expected_seats.difference(&seats) {
        println!("{:?}: {}", missing, missing.0 as u16 * 8 + missing.1 as u16);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_split() {
        assert_eq!((0, 63), Front.split(0, 127));
        assert_eq!((32, 63), Back.split(0, 63));
        assert_eq!((32, 47), Front.split(32, 63));
        assert_eq!((40, 47), Back.split(32, 47));
        assert_eq!((44, 47), Back.split(40, 47));
        assert_eq!((44, 45), Front.split(44, 47));
        assert_eq!((44, 45), Front.split(44, 47));
        
    }
}
