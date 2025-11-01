use std::collections::HashSet;
type ABA = HashSet<(char, char, char)>;

struct Entry {
    sequences: Vec<Sequence>,
    hyprnets: Vec<Sequence>,
}

struct Sequence {
    chars: Vec<char>,
}

impl Sequence {
    fn has_abba(&self) -> bool {
        for win in self.chars.windows(4) {
            if win[0] == win[3] && win[1] == win[2] && win[0] != win[1] {
                return true;
            }
        }
        return false;
    }

    fn populate_aba(&self, map: &mut ABA) {
        for win in self.chars.windows(3) {
            if win[0] == win[2] && win[0] != win[1] {
                map.insert((win[1], win[0], win[1]));
            }
        }
    }

    fn has_aba(&self, map: &ABA) -> bool {
        for win in self.chars.windows(3) {
            if map.contains(&(win[0], win[1], win[2])) {
                return true;
            }
        }
        return false;
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

fn parse_input(input: &str) -> Vec<Entry> {
    let lines = input.lines();

    let mut ret = Vec::<Entry>::with_capacity(lines.clone().count());

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let splits = line.split(&['[', ']']);
        let size = splits.clone().count();

        let mut hyprnets = Vec::with_capacity(size);
        let mut sequences = Vec::with_capacity(size);

        let mut in_bracket = false;
        for split in splits {
            match in_bracket {
                true => &mut hyprnets,
                false => &mut sequences,
            }
            .push(Sequence {
                chars: split.chars().collect(),
            });
            in_bracket = !in_bracket;
        }

        ret.push(Entry {
            sequences,
            hyprnets,
        });
    }

    ret
}

fn part_one(input: &str) -> u64 {
    let entries = parse_input(input);

    let mut count = 0;

    'outer: for entry in entries {
        for hypr in &entry.hyprnets {
            if hypr.has_abba() {
                continue 'outer;
            }
        }
        for seq in &entry.sequences {
            if seq.has_abba() {
                count += 1;
                continue 'outer;
            }
        }
    }

    count
}

fn part_two(input: &str) -> u64 {
    let entries = parse_input(input);

    let mut count = 0;

    'outer: for entry in entries {
        let mut map = ABA::new();
        for hypr in &entry.hyprnets {
            hypr.populate_aba(&mut map);
        }
        for seq in &entry.sequences {
            if seq.has_aba(&map) {
                count += 1;
                continue 'outer;
            }
        }
    }

    count
}
