use std::{cell::Cell, collections::HashMap};

#[derive(Clone, Debug)]
struct Tile {
    number: i32,
    blocks: Cell<[[bool; 10]; 10]>,
    sealed: Cell<bool>,
}

fn bools_to_bits(b: impl Iterator<Item = bool>) -> u16 {
    let mut rv = 0;
    for bit in b {
        rv <<= 1;
        if bit {
            rv |= 1;
        }
    }
    rv
}

impl Tile {
    fn render_line(&self, i: usize) {
        for cell in self.blocks.get()[i].iter() {
            if *cell {
                print!("#")
            } else {
                print!(".")
            }
        }
    }

    fn parse(s: &str) -> Self {
        let title = s.lines().next().unwrap();
        let number = title
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse()
            .unwrap();
        let mut rv = vec![];
        for line in s.lines().skip(1) {
            if line.is_empty() {
                continue;
            }
            let mut it = line.bytes().map(|b| b == b'#');
            let row = [
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            ];
            rv.push(row);
        }
        let blocks = [
            rv[0], rv[1], rv[2], rv[3], rv[4], rv[5], rv[6], rv[7], rv[8], rv[9],
        ];
        Tile {
            number,
            blocks: Cell::new(blocks),
            sealed: Cell::new(false),
        }
    }
    fn top_edge(&self) -> [u16; 2] {
        let top = bools_to_bits(self.blocks.get()[0].iter().copied());
        let top_r = bools_to_bits(self.blocks.get()[0].iter().copied().rev());
        [top, top_r]
    }
    fn bottom_edge(&self) -> [u16; 2] {
        let bottom = bools_to_bits(self.blocks.get()[9].iter().copied());
        let bottom_r = bools_to_bits(self.blocks.get()[9].iter().copied().rev());
        [bottom, bottom_r]
    }
    fn left_edge(&self) -> [u16; 2] {
        let left = bools_to_bits(self.blocks.get().iter().map(|row| row[0]));
        let left_r = bools_to_bits(self.blocks.get().iter().map(|row| row[0]).rev());
        [left, left_r]
    }
    fn right_edge(&self) -> [u16; 2] {
        let right = bools_to_bits(self.blocks.get().iter().map(|row| row[9]));
        let right_r = bools_to_bits(self.blocks.get().iter().map(|row| row[9]).rev());
        [right, right_r]
    }
    fn edges(&self) -> [u16; 8] {
        let [top, top_r] = self.top_edge();
        let [bottom, bottom_r] = self.bottom_edge();
        let [left, left_r] = self.left_edge();
        let [right, right_r] = self.right_edge();
        [top, top_r, bottom, bottom_r, left, left_r, right, right_r]
    }

    fn reorient_to_bottom(&self, edge: u16) {
        assert!(!self.sealed.get());
        if self.bottom_edge()[0] == edge {
            // Do nothing
        } else if self.bottom_edge()[1] == edge {
            self.flip_h();
        } else if self.right_edge()[0] == edge {
            self.rotate_cw();
            self.flip_h();
        } else if self.right_edge()[1] == edge {
            self.rotate_cw();
        } else if self.top_edge()[0] == edge {
            self.flip_v();
        } else if self.top_edge()[1] == edge {
            self.flip_v();
            self.flip_h();
        } else if self.left_edge()[0] == edge {
            self.rotate_cw();
            self.flip_v();
            self.flip_h();
        } else if self.left_edge()[1] == edge {
            self.rotate_cw();
            self.flip_v();
        } else {
            panic!("No such edge!");
        }
        assert_eq!(edge, self.bottom_edge()[0]);
    }

    fn reorient_to_top(&self, edge: u16) {
        self.reorient_to_bottom(edge);
        self.flip_v();
        assert_eq!(edge, self.top_edge()[0]);
    }

    fn reorient_to_left(&self, edge: u16) {
        self.reorient_to_bottom(edge);
        self.rotate_cw();
        assert_eq!(edge, self.left_edge()[0]);
    }

    fn reorient_to_right(&self, edge: u16) {
        self.reorient_to_bottom(edge);
        self.rotate_cw();
        self.flip_h();
        assert_eq!(edge, self.right_edge()[0]);
    }

    fn flip_h(&self) {
        let mut blocks = self.blocks.get();
        for row in blocks.iter_mut() {
            *row = [
                row[9], row[8], row[7], row[6], row[5], row[4], row[3], row[2], row[1], row[0],
            ];
        }
        self.blocks.set(blocks);
    }

    fn flip_v(&self) {
        let mut blocks = self.blocks.get();
        blocks = [
            blocks[9], blocks[8], blocks[7], blocks[6], blocks[5], blocks[4], blocks[3], blocks[2],
            blocks[1], blocks[0],
        ];
        self.blocks.set(blocks);
    }

    fn rotate_cw(&self) {
        let blocks = self.blocks.get();
        let mut new_blocks = [[false; 10]; 10];
        for i in 0..10 {
            for j in 0..10 {
                new_blocks[i][9 - j] = blocks[j][i];
            }
        }
        self.blocks.set(new_blocks);
    }
}

fn main() {
    let tiles: Vec<_> = include_str!("input.txt")
        .split("\n\n")
        .map(|s| Tile::parse(s))
        .collect();
    let mut edge_map: HashMap<u16, Vec<&Tile>> = HashMap::new();
    println!("Tile count: {}", tiles.len());
    for tile in &tiles {
        for edge in tile.edges().iter() {
            edge_map.entry(*edge).or_default().push(tile);
        }
    }

    let mut placements = HashMap::new();
    placements.insert((0, 0), &tiles[0]);
    let mut to_process = vec![(0, 0)];
    let mut i = 0;
    while let Some(next) = to_process.pop() {
        i += 1;
        if i > 144 {
            break;
        }
        let tile = placements[&(next)];
        
        if !placements.contains_key(&(next.0, next.1 + 1)) {
            let edge = tile.bottom_edge()[0];

            if let Some(neighbours) = edge_map.get(&edge) {
                if let Some(neighbour) = neighbours
                    .iter()
                    .filter(|&&t| t.number != tile.number)
                    .next()
                {
                    
                    neighbour.reorient_to_top(edge);
                    neighbour.sealed.set(true);
                    placements.insert((next.0, next.1 + 1), *neighbour);
                    to_process.push((next.0, next.1 + 1));
                }
            }
        }
        if !placements.contains_key(&(next.0, next.1 - 1)) {
            let edge = tile.top_edge()[0];
            if let Some(neighbours) = edge_map.get(&edge) {
                if let Some(neighbour) = neighbours
                    .iter()
                    .filter(|&&t| t.number != tile.number)
                    .next()
                {
                    neighbour.reorient_to_bottom(edge);
                    neighbour.sealed.set(true);
                    placements.insert((next.0, next.1 - 1), *neighbour);
                    to_process.push((next.0, next.1 - 1));
                }
            }
        }
        if !placements.contains_key(&(next.0 - 1, next.1)) {
            let edge = tile.left_edge()[0];
            if let Some(neighbours) = edge_map.get(&edge) {
                if let Some(neighbour) = neighbours
                    .iter()
                    .filter(|&&t| t.number != tile.number)
                    .next()
                {
                    neighbour.reorient_to_right(edge);
                    neighbour.sealed.set(true);
                    placements.insert((next.0 - 1, next.1), *neighbour);
                    to_process.push((next.0 - 1, next.1));
                }
            }
        }
        if !placements.contains_key(&(next.0 + 1, next.1)) {
            let edge = tile.right_edge()[0];
            if let Some(neighbours) = edge_map.get(&edge) {
                if let Some(neighbour) = neighbours
                    .iter()
                    .filter(|&&t| t.number != tile.number)
                    .next()
                {
                    neighbour.reorient_to_left(edge);
                    neighbour.sealed.set(true);
                    placements.insert((next.0 + 1, next.1), *neighbour);
                    to_process.push((next.0 + 1, next.1));
                }
            }
        }
    }
    // println!("{:?}", placements);

    let min_x = placements.keys().map(|k| k.0).min().unwrap();
    let min_y = placements.keys().map(|k| k.1).min().unwrap();
    let max_x = placements.keys().map(|k| k.0).max().unwrap();
    let max_y = placements.keys().map(|k| k.1).max().unwrap();
    for y in min_y..=max_y {
        for row in 0..10 {
            for x in min_x..=max_x {
                if let Some(tile) = placements.get(&(x, y)) {
                    tile.render_line(row);
                    print!("  ")
                } else {
                    print!("            ");
                }
            }
            println!();
        }
        println!();
    }
    println!("{}", placements[&(min_x, min_y)].number as u64 * placements[&(max_x, min_y)].number as u64 * placements[&(min_x, max_y)].number as u64 * placements[&(max_x, max_y)].number as u64);
}
