#[derive(Copy, Clone, Debug)]
enum Command {
    Up,
    Down,
    Right,
    Left,
}

impl Command {
    fn reverse(self) -> Command {
        match self {
            Command::Up => Command::Down,
            Command::Down => Command::Up,
            Command::Right => Command::Left,
            Command::Left => Command::Right,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum KeypadType {
    Square,
    Circle,
}

#[derive(Copy, Clone, Debug)]
struct Keypad {
    i: i32,
    j: i32,

    tp: KeypadType,
}

impl Keypad {
    fn walk(&mut self, cmd: Command) {
        (self.i, self.j) = match cmd {
            Command::Up => (self.i - 1, self.j),
            Command::Down => (self.i + 1, self.j),
            Command::Right => (self.i, self.j + 1),
            Command::Left => (self.i, self.j - 1),
        };

        if !self.in_bounds() {
            self.walk(cmd.reverse());
        }
    }

    fn in_bounds(&self) -> bool {
        match self.tp {
            KeypadType::Square => (0 <= self.i && self.i < 3) && (0 <= self.j && self.j < 3),
            KeypadType::Circle => (self.i * self.i) + (self.j * self.j) <= 4,
        }
    }

    fn cur_val(&self) -> String {
        match self.tp {
            KeypadType::Square => ((self.i * 3) + self.j + 1).to_string(),
            KeypadType::Circle => match (self.i, self.j) {
                (-2, 0) => "1",
                (-1, -1) => "2",
                (-1, 0) => "3",
                (-1, 1) => "4",
                (0, -2) => "5",
                (0, -1) => "6",
                (0, 0) => "7",
                (0, 1) => "8",
                (0, 2) => "9",
                (1, -1) => "A",
                (1, 0) => "B",
                (1, 1) => "C",
                (2, 0) => "D",
                _ => panic!(),
            }
            .to_string(),
        }
    }
}

fn main() {
    let example = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    println!("part 1");
    println!(">> example -> {}", part_one(example));
    println!(">> input   -> {}", part_one(input));

    println!("part 2");
    println!(">> example -> {}", part_two(example));
    println!(">> input   -> {}", part_two(input));
}

fn parse_commands(input: &str) -> Vec<Vec<Command>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    'U' => Command::Up,
                    'D' => Command::Down,
                    'R' => Command::Right,
                    'L' => Command::Left,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn part_one(input: &str) -> String {
    let commands = parse_commands(input);

    let mut ans = String::new();
    let mut key = Keypad {
        i: 1,
        j: 1,
        tp: KeypadType::Square,
    };

    for line in commands {
        for cmd in line {
            key.walk(cmd);
        }
        ans += key.cur_val().as_str();
    }

    ans
}

fn part_two(input: &str) -> String {
    let commands = parse_commands(input);

    let mut ans = String::new();
    let mut key = Keypad {
        i: 0,
        j: -2,
        tp: KeypadType::Circle,
    };

    for line in commands {
        for cmd in line {
            key.walk(cmd);
        }
        ans += key.cur_val().as_str();
    }

    ans
}
