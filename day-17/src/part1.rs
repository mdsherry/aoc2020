use std::collections::{HashMap, HashSet};

struct TimeCube {
    cube: HashSet<(i32, i32, i32)>
}

impl TimeCube {
    fn new(template: &str) -> Self {
        let mut cube = HashSet::new();
        for (y, line) in template.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' { cube.insert((x as i32, y as i32, 0)); }
            }
        }

        TimeCube { cube }
    }
    fn tick(&mut self) {
        let mut new_cube = HashSet::new();
        let mut neighbours: HashMap<_, u8> = HashMap::new();
        for &(x, y, z) in &self.cube {
            for &dx in &[-1, 0, 1] {
                for &dy in &[-1, 0, 1] {
                    for &dz in &[-1, 0, 1] {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }
                        *neighbours.entry((x + dx, y + dy, z + dz)).or_default() += 1u8;
                    }
                }
            }
        }
        for (&point, &neighbour_count) in &neighbours {
            if neighbour_count == 3 || (neighbour_count == 2 && self.cube.contains(&point)) {
                new_cube.insert(point);
            }
        }
        self.cube = new_cube;
    }

    fn popcount(&self) -> usize {
        self.cube.len()
    }
}

static INPUT: &str = "###...#.
.##.####
.####.##
###.###.
.##.####
#.##..#.
##.####.
.####.#.";

fn main() {
    let test_input = ".#.\n..#\n###";
    let mut timecube = TimeCube::new(INPUT);
    println!("{}", timecube.popcount());
    for _ in 0..6 {
        timecube.tick();
        println!("{}", timecube.popcount());
    }
}
