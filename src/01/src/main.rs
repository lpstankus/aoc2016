use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Command {
    rot: Rotation,
    dist: i32,
}

impl Direction {
    fn rotate(&mut self, cmd: Rotation) {
        *self = match (&self, cmd) {
            (Direction::North, Rotation::Right) => Direction::East,
            (Direction::East, Rotation::Right) => Direction::South,
            (Direction::South, Rotation::Right) => Direction::West,
            (Direction::West, Rotation::Right) => Direction::North,

            (Direction::North, Rotation::Left) => Direction::West,
            (Direction::West, Rotation::Left) => Direction::South,
            (Direction::South, Rotation::Left) => Direction::East,
            (Direction::East, Rotation::Left) => Direction::North,
        };
    }
}

#[derive(Debug)]
struct Pos {
    dir: Direction,
    x: i32,
    y: i32,
}

impl Pos {
    fn execute(&mut self, cmd: Command) {
        self.dir.rotate(cmd.rot);
        self.walk(cmd.dist);
    }

    fn execute2(&mut self, cmd: Command, set: &mut HashSet<(i32, i32)>) -> bool {
        self.dir.rotate(cmd.rot);
        if self.walk2(cmd.dist, &mut *set) {
            return true;
        }
        false
    }

    fn walk(&mut self, dist: i32) {
        (self.x, self.y) = match &self.dir {
            Direction::North => (self.x, self.y + dist),
            Direction::East => (self.x + dist, self.y),
            Direction::South => (self.x, self.y - dist),
            Direction::West => (self.x - dist, self.y),
        }
    }

    fn walk2(&mut self, dist: i32, set: &mut HashSet<(i32, i32)>) -> bool {
        let (new_x, new_y) = match &self.dir {
            Direction::North => (self.x, self.y + dist),
            Direction::East => (self.x + dist, self.y),
            Direction::South => (self.x, self.y - dist),
            Direction::West => (self.x - dist, self.y),
        };

        let step = if self.x < new_x { 1 } else { -1 };
        while self.x != new_x {
            if !set.insert((self.x, self.y)) {
                return true;
            }
            self.x += step;
        }

        let step = if self.y < new_y { 1 } else { -1 };
        while self.y != new_y {
            if !set.insert((self.x, self.y)) {
                return true;
            }
            self.y += step;
        }

        (self.x, self.y) = (new_x, new_y);
        false
    }

    fn dist(&self) -> u32 {
        return self.x.abs() as u32 + self.y.abs() as u32;
    }
}

fn main() {
    let example1 = include_str!("../example1.txt");
    let example2 = include_str!("../example2.txt");
    let example3 = include_str!("../example3.txt");
    let example4 = include_str!("../example4.txt");
    let input = include_str!("../input.txt");

    println!("part 1");
    println!(">> example1 -> {}", part_one(example1));
    println!(">> example2 -> {}", part_one(example2));
    println!(">> example3 -> {}", part_one(example3));
    println!(">> example4 -> {}", part_one(example4));
    println!(">> input    -> {}", part_one(input));

    println!("part 2");
    println!(">> example4 --> {}", part_two(example4));
    println!(">> input    --> {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let commands = parse_commands(input);

    let mut state = Pos {
        dir: Direction::North,
        x: 0,
        y: 0,
    };
    for cmd in commands {
        state.execute(cmd);
    }

    state.dist()
}

fn part_two(input: &str) -> u32 {
    let commands = parse_commands(input);

    let mut state = Pos {
        dir: Direction::North,
        x: 0,
        y: 0,
    };
    let mut set = HashSet::new();
    for cmd in commands {
        if state.execute2(cmd, &mut set) {
            return state.dist();
        }
    }

    panic!("did not find position");
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .trim()
        .split(", ")
        .map(|cmd| match cmd.chars().nth(0) {
            Some('L') => Command {
                rot: Rotation::Left,
                dist: (cmd[1..].parse().unwrap()),
            },
            Some('R') => Command {
                rot: Rotation::Right,
                dist: (cmd[1..].parse().unwrap()),
            },
            _ => panic!(),
        })
        .collect()
}
