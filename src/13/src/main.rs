use std::collections::{HashSet, VecDeque};

#[derive(Eq, PartialEq)]
enum Cell {
    Wall,
    Open,
}

fn get_cell(spice: usize, x: usize, y: usize) -> Cell {
    let mut num = x * x + 3 * x + 2 * x * y + y + y * y;
    num += spice;
    match num.count_ones() % 2 == 0 {
        true => Cell::Open,
        false => Cell::Wall,
    }
}

struct State {
    x: usize,
    y: usize,
    depth: usize,
}

fn main() {
    let example = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    println!("part 1");
    println!(">> example -> {:?}", part_one(example, (7, 4)));
    println!(">> input   -> {:?}", part_one(input, (31, 39)));

    println!("part 2");
    println!(">> input   -> {:?}", part_two(input));
}

fn parse_input(input: &str) -> usize {
    input.trim().parse().expect("Input should be a number")
}

fn part_one(input: &str, targ: (usize, usize)) -> usize {
    let spice = parse_input(input);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(State {
        x: 1,
        y: 1,
        depth: 0,
    });
    visited.insert((1, 1));

    while let Some(curr) = queue.pop_front() {
        if curr.x == targ.0 && curr.y == targ.1 {
            return curr.depth;
        }

        let x0 = curr.x.saturating_sub(1);
        let x1 = curr.x.saturating_add(1);
        let y0 = curr.y.saturating_sub(1);
        let y1 = curr.y.saturating_add(1);

        let mut insert_if_valid = |x: usize, y: usize| {
            if get_cell(spice, x, y) == Cell::Wall {
                return;
            };

            let key = (x, y);
            if visited.contains(&key) {
                return;
            }
            visited.insert(key);

            queue.push_back(State {
                x,
                y,
                depth: curr.depth + 1,
            });
        };

        insert_if_valid(x0, curr.y);
        insert_if_valid(x1, curr.y);
        insert_if_valid(curr.x, y0);
        insert_if_valid(curr.x, y1);
    }

    usize::MAX
}

fn part_two(input: &str) -> usize {
    let spice = parse_input(input);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(State {
        x: 1,
        y: 1,
        depth: 0,
    });
    visited.insert((1, 1));

    while let Some(curr) = queue.pop_front() {
        if curr.depth == 50 {
            continue;
        }

        let x0 = curr.x.saturating_sub(1);
        let x1 = curr.x.saturating_add(1);
        let y0 = curr.y.saturating_sub(1);
        let y1 = curr.y.saturating_add(1);

        let mut insert_if_valid = |x: usize, y: usize| {
            if get_cell(spice, x, y) == Cell::Wall {
                return;
            };

            let key = (x, y);
            if visited.contains(&key) {
                return;
            }
            visited.insert(key);

            queue.push_back(State {
                x,
                y,
                depth: curr.depth + 1,
            });
        };

        insert_if_valid(x0, curr.y);
        insert_if_valid(x1, curr.y);
        insert_if_valid(curr.x, y0);
        insert_if_valid(curr.x, y1);
    }

    visited.len()
}
