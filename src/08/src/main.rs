#[derive(Debug)]
enum Command {
    Rect((usize, usize)),
    RotateCol((usize, usize)),
    RotateRow((usize, usize)),
}

struct LCD {
    width: usize,
    height: usize,

    screen: Vec<Vec<bool>>,
    screen_copy: Vec<Vec<bool>>,
}

impl LCD {
    fn new(width: usize, height: usize) -> Self {
        let vec = vec![vec![false; width as usize]; height as usize];
        LCD {
            width,
            height,
            screen: vec.clone(),
            screen_copy: vec,
        }
    }

    fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.screen[i][j] {
                    print!("#")
                } else {
                    print!(".")
                };
            }
            println!();
        }
    }

    fn count_pixels(&self) -> usize {
        let mut count = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if self.screen[i][j] {
                    count += 1;
                }
            }
        }
        count
    }

    fn rect(&mut self, x: usize, y: usize) {
        for i in 0..y {
            for j in 0..x {
                self.screen[i][j] = true;
                self.screen_copy[i][j] = true;
            }
        }
    }

    fn rotate_col(&mut self, x: usize, n: usize) {
        for i in 0..self.height {
            let idx = (i + n) % self.height;
            self.screen_copy[idx][x] = self.screen[i][x];
        }
        for i in 0..self.height {
            self.screen[i][x] = self.screen_copy[i][x];
        }
    }

    fn rotate_row(&mut self, y: usize, n: usize) {
        for j in 0..self.width {
            let idx = (j + n) % self.width;
            self.screen_copy[y][idx] = self.screen[y][j];
        }
        for j in 0..self.width {
            self.screen[y][j] = self.screen_copy[y][j];
        }
    }
}

fn main() {
    let example = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    println!("part 1");
    println!(">> example -> {}", part_one(example, true));
    println!(">> input   -> {}", part_one(input, false));

    println!("part 2");
    part_two(input);
}

fn parse_input(input: &str) -> Vec<Command> {
    let lines = input.lines();

    let mut ret = Vec::<Command>::with_capacity(lines.clone().count());

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let splits: Vec<&str> = line.split(' ').collect();
        match splits[0] {
            "rect" => {
                let args: Vec<&str> = splits[1].split('x').collect();
                ret.push(Command::Rect((
                    args[0].parse().unwrap(),
                    args[1].parse().unwrap(),
                )));
            }
            "rotate" => match splits[1] {
                "column" => {
                    let mut it_x = splits[2].split('=');
                    it_x.next();

                    let arg_x = it_x.next().unwrap().parse().unwrap();
                    let arg_n = splits[4].parse().unwrap();

                    ret.push(Command::RotateCol((arg_x, arg_n)));
                }
                "row" => {
                    let mut it_y = splits[2].split('=');
                    it_y.next();

                    let arg_y = it_y.next().unwrap().parse().unwrap();
                    let arg_n = splits[4].parse().unwrap();

                    ret.push(Command::RotateRow((arg_y, arg_n)));
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    ret
}

fn part_one(input: &str, example: bool) -> u64 {
    let entries = parse_input(input);

    let mut lcd = if example {
        LCD::new(7, 3)
    } else {
        LCD::new(50, 6)
    };

    for entry in entries {
        match entry {
            Command::Rect((x, y)) => lcd.rect(x, y),
            Command::RotateCol((x, n)) => lcd.rotate_col(x, n),
            Command::RotateRow((y, n)) => lcd.rotate_row(y, n),
        }
    }

    lcd.count_pixels() as u64
}

fn part_two(input: &str) {
    let entries = parse_input(input);

    let mut lcd = LCD::new(50, 6);

    for entry in entries {
        match entry {
            Command::Rect((x, y)) => lcd.rect(x, y),
            Command::RotateCol((x, n)) => lcd.rotate_col(x, n),
            Command::RotateRow((y, n)) => lcd.rotate_row(y, n),
        }
    }

    lcd.print();
}
