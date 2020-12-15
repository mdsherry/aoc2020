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

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    facing: Facing
}

impl Ship {
    fn do_move(&mut self, command: Command) {
        match command.movement {
            Movement::N => self.y -= command.value as i32,
            Movement::S => self.y += command.value as i32,
            Movement::E => self.x += command.value as i32,
            Movement::W => self.x -= command.value as i32,
            Movement::R => self.facing = self.facing.cw(command.value),
            Movement::L => self.facing = self.facing.ccw(command.value),
            Movement::F => match self.facing {
                Facing::N => self.y -= command.value as i32,
                Facing::S => self.y += command.value as i32,
                Facing::E => self.x += command.value as i32,
                Facing::W => self.x -= command.value as i32,
            }
        }
    }
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    let commands: Vec<Command> = INPUT.lines().map(|s| Command::parse(s)).collect();
    let mut ship = Ship { x: 0, y: 0, facing: Facing::E};
    for command in commands {
        ship.do_move(command);
    }
    println!("{:?} {} + {} = {}", ship, ship.x.abs(), ship.y.abs(), ship.x.abs() + ship.y.abs());
}
