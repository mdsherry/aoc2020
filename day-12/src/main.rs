#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Movement {
    N,
    S,
    E,
    W,
    L,
    R,
    F
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Facing {
    N,
    S,
    E,
    W
}

impl Facing {
    fn cw(self, degrees: u32) -> Self {
        use Facing::*;
        match degrees {
            0 => self,
            90 => match self {
                N => E,
                E => S,
                S => W,
                W => N
            },
            180 => match self {
                N => S,
                S => N,
                E => W,
                W => E,
            }
            270 => self.ccw(90),
            _ => panic!()
        }
    }
    fn ccw(self, degrees: u32) -> Self {
        use Facing::*;
        match degrees {
            0 => self,
            90 => match self {
                N => W,
                E => N,
                S => E,
                W => S
            },
            180 => self.cw(180),
            270 => self.cw(90),
            _ => panic!()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Command {
    movement: Movement,
    value: u32
}

impl Command {
    fn parse(s: &str) -> Command {
        use Movement::*;
        let movement = match s.as_bytes()[0] {
            b'N' => N,
            b'S' => S,
            b'E' => E,
            b'W' => W,
            b'L' => L,
            b'R' => R,
            b'F' => F,
            _ => panic!()
        };
        let value = s[1..].parse().unwrap();
        Command { movement, value }
    }
}

fn rotate_cw(x: i32, y: i32) -> (i32, i32){
    (y, -x)
}
fn rotate_ccw(x: i32, y: i32) -> (i32, i32){
    (-y, x)
}

fn rotate_deg(x: i32, y: i32, degrees: i32) -> (i32, i32) {
    match degrees {
        0 => (x, y),
        90 | -270 => rotate_cw(x, y),
        180 | -180  => (-x, -y),
        270 | -90 => rotate_ccw(x, y),
        _ => panic!()
    }
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    wx: i32,
    wy: i32,
}

impl Ship {
    fn do_move(&mut self, command: Command) {
        match command.movement {
            Movement::N => self.wy += command.value as i32,
            Movement::S => self.wy -= command.value as i32,
            Movement::E => self.wx += command.value as i32,
            Movement::W => self.wx -= command.value as i32,
            Movement::R => {let (nx, ny) = rotate_deg(self.wx, self.wy, command.value as i32); self.wx = nx; self.wy = ny},
            Movement::L => {let (nx, ny) = rotate_deg(self.wx, self.wy, -(command.value as i32)); self.wx = nx; self.wy = ny},
            Movement::F => {
                self.x += self.wx * command.value as i32;
                self.y += self.wy * command.value as i32;
            }
        }
    }
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    let commands: Vec<Command> = INPUT.lines().map(|s| Command::parse(s)).collect();
    let mut ship = Ship { x: 0, y: 0, wx: 10, wy: 1};
    for command in commands {
        println!("{:?}", ship);
        ship.do_move(command);        
    }
    println!("{:?} {} + {} = {}", ship, ship.x.abs(), ship.y.abs(), ship.x.abs() + ship.y.abs());
}
