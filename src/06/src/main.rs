use std::collections::HashMap;

type Entry = Vec<char>;

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

    let mut ret = vec![vec![]; lines.clone().next().unwrap().len()];
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        for (i, ch) in line.chars().enumerate() {
            ret[i].push(ch);
        }
    }

    ret
}

fn part_one(input: &str) -> String {
    let entries = parse_input(input);

    let mut ans = Vec::with_capacity(entries.len());

    let mut map = HashMap::new();
    for entry in entries {
        let mut entry: Vec<char> = entry.clone();

        map.clear();
        for ch in &entry {
            *map.entry(*ch).or_insert(0) += 1;
        }

        entry.sort_by_key(|ch| std::cmp::Reverse(map[ch]));
        let letter = entry[0];

        ans.push(letter);
    }

    String::from_iter(ans)
}

fn part_two(input: &str) -> String {
    let entries = parse_input(input);

    let mut ans = Vec::with_capacity(entries.len());

    let mut map = HashMap::new();
    for entry in entries {
        let mut entry: Vec<char> = entry.clone();

        map.clear();
        for ch in &entry {
            *map.entry(*ch).or_insert(0) += 1;
        }

        entry.sort_by_key(|ch| map[ch]);
        let letter = entry[0];

        ans.push(letter);
    }

    String::from_iter(ans)
}
