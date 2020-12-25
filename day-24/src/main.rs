use core::panic;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct Cursor {
    x: i32,
    y: i32
}

impl Cursor {
    fn new() -> Self { Default::default() }
    fn e(self) -> Self {
        Cursor { x: self.x + 2, ..self }
    }
    fn w(self) -> Self {
        Cursor { x: self.x - 2, ..self }
    }
    fn ne(self) -> Self {
        Cursor { x: self.x + 1, y: self.y + 1 }
    }
    fn nw(self) -> Self {
        Cursor { x: self.x - 1, y: self.y + 1 }
    }
    fn se(self) -> Self {
        Cursor { x: self.x + 1, y: self.y - 1 }
    }
    fn sw(self) -> Self {
        Cursor { x: self.x - 1, y: self.y - 1 }
    }
}

fn line_to_tile(s: &str) -> Cursor {
    let mut cur = Cursor::new();
    let mut it = s.bytes();
    while let Some(c) = it.next() {
        cur = match c {
            b'e' => cur.e(),
            b'w' => cur.w(),
            b'n' => match it.next().unwrap() {
                b'e' => cur.ne(),
                b'w' => cur.nw(),
                _ => panic!()
            },
            b's' => match it.next().unwrap() {
                b'e' => cur.se(),
                b'w' => cur.sw(),
                _ => panic!()
            },
            _ => panic!()
        };
    }
    cur
}

fn neighbours((x, y): (i32, i32)) -> [(i32, i32); 6] {
    [(x - 2, y), (x - 1, y + 1), (x+1, y+1), (x+2, y), (x+1, y-1), (x-1, y-1)]
}

fn circle_of_life(tiles: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new = HashSet::new();
    let mut white_neighbours = vec![];
    for &tile in tiles {
        let count = neighbours(tile).iter().filter(|xy| if tiles.contains(*xy) { true } else { white_neighbours.push(**xy); false }).count();
        if count == 1 || count == 2 {
            new.insert(tile);
        }
    }
    for tile in white_neighbours {
        let count = neighbours(tile).iter().filter(|xy| tiles.contains(*xy)).count();
        if count == 2 {
            new.insert(tile);
        }
    }
    new
}
fn main() {
    let input = include_str!("input.txt");
    let mut tiles = HashSet::new();
    input.lines().map(line_to_tile).for_each(|cur| {
        if !tiles.insert((cur.x, cur.y)) {
            tiles.remove(&(cur.x, cur.y));
        }
    });
    println!("{}", tiles.len());
    for _ in 0..100 {
        tiles = circle_of_life(&tiles);
    }
    println!("{}", tiles.len());
    
}


#[cfg(test)]
mod test {
    use super::{Cursor, line_to_tile};
    #[test]
    fn test_cursor() {
        assert_eq!(Cursor::new(), Cursor::new().ne().w().se());
        assert_eq!(Cursor::new(), Cursor::new().nw().w().sw().e().e());
        assert_eq!(Cursor::new(), Cursor::new().ne().sw());
        assert_eq!(Cursor { x: 2, y: 2 }, Cursor::new().ne().ne());
        assert_eq!(Cursor::new().e(), Cursor::new().ne().se());
        assert_eq!(Cursor::new().ne().nw(), Cursor::new().nw().ne());

    }
    #[test]
    fn test_line_to_tile() {
        assert_eq!(Cursor::new(), line_to_tile("newse"));
        assert_eq!(Cursor::new(), line_to_tile("nwwswee"));
        assert_eq!(Cursor::new(), line_to_tile("nesw"));
        assert_eq!(Cursor { x: 2, y: 2 }, line_to_tile("nene"));
        assert_eq!(Cursor::new().e(), line_to_tile("nese"));
        assert_eq!(Cursor::new().ne().nw(), line_to_tile("nwne"));

    }
}