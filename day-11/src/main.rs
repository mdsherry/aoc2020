#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Empty,
    Full,
    Floor
}

static INPUT: &str = include_str!("input.txt");

fn adjacent_filled(x: usize, y: usize, map: &Vec<Vec<State>>) -> u8 {
    let mut count = 0;
    let max_x = map[0].len() as i32;
    let max_y = map.len() as i32;
    for &(dx, dy) in &[(-1i32, -1i32), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        let mut ex = x as i32 + dx;
        let mut ey = y as i32 + dy;
        while ex >= 0 && ex < max_x && ey >= 0 && ey < max_y {
            if map[ey as usize][ex as usize] == State::Full {
                count += 1;
                break;
            } else if map[ey as usize][ex as usize] == State::Empty {
                break;
            }
            ex += dx;
            ey += dy;
        }
    }
    count
}

fn main() {
    let mut map: Vec<_> = INPUT.lines().map(|line| line.chars().map(|c| match c {
        'L' => State::Empty,
        '.' => State::Floor,
        _ => panic!()
    }).collect::<Vec<_>>()).collect();
    let mut other_map = map.clone();
    loop {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == State::Floor {
                    continue
                }
                match adjacent_filled(x, y, &map) {
                    0 => other_map[y][x] = State::Full,
                    n if n >= 5 => other_map[y][x] = State::Empty,
                    _ => other_map[y][x] = map[y][x]
                }
            }
        }
        other_map.iter().for_each(|line| {
            println!("{}", line.iter().map(|state| match state {
                State::Floor => '.',
                State::Full => '#',
                State::Empty => 'L'
            }).collect::<String>());
        });
        println!();
        if map == other_map {
            break;
        }
        std::mem::swap(&mut map, &mut other_map);
    }
    let total_count= map.iter().map(|line| line.iter().filter(|&&c| c == State::Full).count()).sum::<usize>();
    println!("{}", total_count);
}
