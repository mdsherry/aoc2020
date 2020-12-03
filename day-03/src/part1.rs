enum Tile {
    Empty,
    Tree
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Tree,
            _ => unimplemented!()
        }
    }
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    let map: Vec<_> = INPUT.lines().map(|line| line.chars().map(|c| c.into()).collect::<Vec<Tile>>()).collect();
    let mut x = 0;
    let mut seen = 0;
    for y in 0..map.len() {
        match map[y][x] {
            Tile::Empty => (),
            Tile::Tree => {seen += 1;}
        }
        x += 3;
        if x >= map[y].len() {
            x -= map[y].len();
        }
    }
    println!("{}", seen);
}
