use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Generator(u8);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Microchip(u8);

#[derive(Clone, Debug)]
enum Item {
    Generator(Generator),
    Microchip(Microchip),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bitset {
    backing: u8,
}

impl Bitset {
    fn new() -> Self {
        Bitset { backing: 0 }
    }

    fn insert(&mut self, val: u8) {
        self.backing |= 1 << val;
    }

    fn remove(&mut self, val: u8) {
        self.backing &= !(1 << val);
    }

    fn clear(&mut self) {
        self.backing = 0;
    }

    fn contains(&self, val: u8) -> bool {
        let value = self.backing & (1 << val);
        value != 0
    }

    fn is_empty(&self) -> bool {
        self.backing == 0
    }

    fn iter(&self) -> impl Iterator<Item = u8> + Clone {
        let clone = self.clone();
        (0..u8::BITS)
            .filter(move |val| clone.contains(*val as u8))
            .map(|val| val as u8)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Floor {
    generators: Bitset,
    microchips: Bitset,
}

impl Floor {
    fn items_it(&self) -> impl Iterator<Item = Vec<Item>> {
        let generators = self
            .generators
            .iter()
            .map(|g| Item::Generator(Generator(g)));

        let microchips = self
            .microchips
            .iter()
            .map(|c| Item::Microchip(Microchip(c)));

        let all = generators.clone().chain(microchips.clone());
        let pairs = all.array_combinations().map(|[a, b]: [Item; 2]| vec![a, b]);

        let single_gens = generators.map(|g| vec![g]);
        let single_chps = microchips.map(|c| vec![c]);

        pairs.chain(single_gens).chain(single_chps)
    }

    fn clear(&mut self) {
        self.generators.clear();
        self.microchips.clear();
    }

    fn contains(&self, item: &Item) -> bool {
        match item {
            Item::Generator(val) => self.generators.contains(val.0),
            Item::Microchip(val) => self.microchips.contains(val.0),
        }
    }

    fn unstable(&self) -> bool {
        if self.generators.is_empty() {
            return false;
        }
        for item in self.microchips.iter() {
            if !self.generators.contains(item) {
                return true;
            }
        }
        false
    }

    fn is_empty(&self) -> bool {
        self.generators.is_empty() && self.microchips.is_empty()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Reactor {
    cur_idx: usize,
    floors: Vec<Floor>,
}

impl Reactor {
    fn clone(&self) -> Reactor {
        let mut floors = Vec::new();
        for floor in &self.floors {
            let mut cloned_floor = Floor {
                generators: Bitset::new(),
                microchips: Bitset::new(),
            };
            for generator in floor.generators.iter() {
                cloned_floor.generators.insert(generator);
            }
            for chp in floor.microchips.iter() {
                cloned_floor.microchips.insert(chp);
            }
            floors.push(cloned_floor);
        }
        Reactor {
            cur_idx: self.cur_idx,
            floors,
        }
    }

    fn canonicalize(&mut self) {
        let mut gen_pos: HashMap<u8, usize> = HashMap::new();
        let mut chp_pos: HashMap<u8, usize> = HashMap::new();

        for (floor_idx, floor) in self.floors.iter().enumerate() {
            for g in floor.generators.iter() {
                gen_pos.insert(g, floor_idx);
            }
            for c in floor.microchips.iter() {
                chp_pos.insert(c, floor_idx);
            }
        }

        let mut pairs = Vec::new();
        for id in 0..u8::BITS {
            let id = id as u8;
            if let (Some(&gf), Some(&cf)) = (gen_pos.get(&id), chp_pos.get(&id)) {
                pairs.push((gf, cf));
            }
        }

        pairs.sort_unstable();

        for floor in &mut self.floors {
            floor.clear();
        }

        for (new_id, (gf, cf)) in pairs.into_iter().enumerate() {
            self.floors[gf].generators.insert(new_id as u8);
            self.floors[cf].microchips.insert(new_id as u8);
        }
    }

    fn move_items(&self, items: &Vec<Item>, dir: i8) -> Option<Self> {
        for item in items {
            if !self.floors[self.cur_idx].contains(item) {
                return None;
            }
        }

        if (self.cur_idx as i64) < (-dir) as i64 {
            return None;
        }

        if self.cur_idx + dir as usize >= self.floors.len() {
            return None;
        }

        let mut new_reactor = self.clone();

        let old_floor = self.cur_idx;
        let new_floor = (self.cur_idx as i64 + dir as i64) as usize;

        for item in items {
            match item {
                Item::Generator(generator) => {
                    new_reactor.floors[old_floor].generators.remove(generator.0);
                    new_reactor.floors[new_floor].generators.insert(generator.0);
                }
                Item::Microchip(chp) => {
                    new_reactor.floors[old_floor].microchips.remove(chp.0);
                    new_reactor.floors[new_floor].microchips.insert(chp.0);
                }
            }
        }

        if new_reactor.floors[old_floor].unstable() || new_reactor.floors[new_floor].unstable() {
            return None;
        }

        new_reactor.cur_idx = new_floor;

        Some(new_reactor)
    }

    fn end_state(&self) -> bool {
        let last_idx = self.floors.len() - 1;

        if self.floors[last_idx].is_empty() {
            return false;
        }

        for floor in self.floors[..last_idx].iter() {
            if !floor.is_empty() {
                return false;
            }
        }

        true
    }
}

struct Mapper {
    cur: u8,
    map: HashMap<String, u8>,
}

fn main() {
    let example = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    println!("part 1");
    println!(">> example -> {:?}", part_one(example));
    println!(">> input   -> {:?}", part_one(input));

    println!("part 2");
    println!(">> input   -> {:?}", part_two(input));
}

fn parse_input(input: &str) -> (Reactor, Mapper) {
    let mut floors = Vec::with_capacity(4);
    let mut map = Mapper {
        cur: 0,
        map: HashMap::new(),
    };

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        let mut floor = Floor {
            generators: Bitset::new(),
            microchips: Bitset::new(),
        };

        let splits: Vec<&str> = line
            .split([' ', ',', '.'])
            .filter(|x| x.len() > 0 && *x != "and")
            .collect();
        assert_eq!(splits[3], "contains");

        let mut splits = &splits[4..];
        while splits.len() > 0 {
            match splits[0] {
                "a" => {
                    match splits[2] {
                        "generator" => {
                            let generator = splits[1];
                            match map.map.get(generator) {
                                Some(val) => floor.generators.insert(*val),
                                None => {
                                    floor.generators.insert(map.cur);
                                    map.map.insert(generator.to_string(), map.cur);
                                    map.cur += 1;
                                }
                            }
                        }
                        "microchip" => {
                            let microchip = splits[1].split('-').next().unwrap();
                            match map.map.get(microchip) {
                                Some(val) => floor.microchips.insert(*val),
                                None => {
                                    floor.microchips.insert(map.cur);
                                    map.map.insert(microchip.to_string(), map.cur);
                                    map.cur += 1;
                                }
                            }
                        }
                        _ => panic!(),
                    };
                    splits = &splits[3..];
                }
                _ => splits = &splits[1..],
            }
        }
        floors.push(floor);
    }

    (Reactor { floors, cur_idx: 0 }, map)
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    depth: usize,
    state: Reactor,
}

fn part_one(input: &str) -> Option<usize> {
    let (reactor, _) = parse_input(input);
    solve(reactor)
}

fn part_two(input: &str) -> Option<usize> {
    let (mut reactor, map) = parse_input(input);
    let floor = &mut reactor.floors[0];
    floor.generators.insert(map.cur);
    floor.microchips.insert(map.cur);
    floor.generators.insert(map.cur + 1);
    floor.microchips.insert(map.cur + 1);
    solve(reactor)
}

fn solve(reactor: Reactor) -> Option<usize> {
    let mut states = VecDeque::new();
    states.push_back(State {
        depth: 0,
        state: reactor,
    });

    let mut seen = HashSet::<Reactor>::new();

    while let Some(state) = states.pop_front() {
        let reactor = state.state;

        if reactor.end_state() {
            return Some(state.depth);
        }

        let floor = &reactor.floors[reactor.cur_idx];

        for items in floor.items_it() {
            for off in [-1, 1] {
                let Some(mut new_reactor) = reactor.move_items(&items, off) else {
                    continue;
                };
                new_reactor.canonicalize();
                if seen.contains(&new_reactor) {
                    continue;
                }
                seen.insert(new_reactor.clone());
                states.push_back(State {
                    depth: state.depth + 1,
                    state: new_reactor,
                });
            }
        }
    }

    None
}
