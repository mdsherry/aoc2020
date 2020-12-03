mod part1;
enum Tile {
    Empty,
    Tree,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Tree,
            _ => unimplemented!(),
        }
    }
}

static INPUT: &str = include_str!("input.txt");

fn check_angle(map: &Vec<Vec<Tile>>, dx: usize, dy: usize) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut seen = 0;
    while y < map.len() {
        match map[y][x] {
            Tile::Empty => (),
            Tile::Tree => {
                seen += 1;
            }
        }
        x += dx;
        if x >= map[y].len() {
            x -= map[y].len();
        }
        y += dy;
    }
    seen
}

fn main() {
    let map: Vec<_> = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Tile>>())
        .collect();
    let slopes = [[1usize, 1usize], [3, 1], [5, 1], [7, 1], [1, 2]];
    let prod = slopes
        .into_iter()
        .cloned()
        .map(|[dx, dy]| check_angle(&map, dx, dy))
        .fold(1, |a, b| a * b);
    println!("{}", prod);
}
