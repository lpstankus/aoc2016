use std::collections::HashMap;

#[derive(Debug)]
struct Entry {
    words: Vec<Vec<char>>,
    sector_id: u64,
    checksum: Vec<char>,
}

impl Entry {
    fn is_valid(&self) -> bool {
        let mut map = HashMap::<char, u32>::new();
        for word in &self.words {
            for letter in word {
                let val = map.get(&letter).or(Some(&0)).unwrap();
                map.insert(*letter, val + 1);
            }
        }

        let mut keys: Vec<char> = map.keys().copied().collect();
        keys.sort_by(|a, b| map[b].cmp(&map[a]).then_with(|| a.cmp(b)));

        let mut i = 0;
        while i < self.checksum.len() {
            if self.checksum[i] != keys[i] {
                return false;
            }
            i += 1;
        }

        return true;
    }

    fn rotate_words(&mut self) {
        for word in &mut self.words {
            for letter in word {
                let mut based = *letter as u64 - 'a' as u64;
                based += self.sector_id;
                based %= 26;
                *letter = (based as u8 + 'a' as u8) as char;
            }
        }
    }

    fn print_words(&self) {
        let mut strings = vec![];
        for word in &self.words {
            strings.push(String::from_iter(word));
        }
        println!("{} -> {}", self.sector_id, strings.join(" "));
    }
}

fn main() {
    let example = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    println!("part 1");
    println!(">> example -> {}", part_one(example));
    println!(">> input   -> {}", part_one(input));

    println!("part 2");
    println!("keys from input:");
    part_two(input);
}

fn parse_entries(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let mut words = vec![];
            let mut letters = vec![];
            let mut section_id = vec![];
            let mut checksum = vec![];

            let mut chars = line.chars();
            for ch in chars.by_ref().take_while(|ch| *ch != '[') {
                if ch.is_digit(10) {
                    section_id.push(ch);
                    continue;
                }
                if ch != '-' {
                    letters.push(ch)
                } else {
                    words.push(letters.clone());
                    letters.clear();
                }
            }
            let sector_id = String::from_iter(section_id)
                .parse()
                .expect("Failed to parse section id");

            for ch in chars.take_while(|ch| *ch != ']') {
                checksum.push(ch);
            }

            Entry {
                words,
                sector_id,
                checksum,
            }
        })
        .collect()
}

fn part_one(input: &str) -> u64 {
    let entries = parse_entries(input);

    let mut ans = 0;

    for entry in entries {
        if entry.is_valid() {
            ans += entry.sector_id;
        }
    }

    ans
}

fn part_two(input: &str) {
    let mut entries = parse_entries(input);
    for entry in &mut entries {
        if !entry.is_valid() {
            continue;
        }
        entry.rotate_words();
        entry.print_words();
    }
}
