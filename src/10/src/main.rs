use std::collections::{HashMap, HashSet};

type Bot = usize;
type Out = usize;
type Value = usize;

#[derive(Debug)]
enum Output {
    Out(Out),
    Bot(Bot),
}

#[derive(Debug)]
struct BotRule {
    lo: Output,
    hi: Output,
}

type RuleSet = HashMap<Bot, BotRule>;
type OutState = HashMap<Out, HashSet<Value>>;
type BotState = HashMap<Bot, HashSet<Value>>;
type ValState = HashMap<Value, Bot>;

struct Factory {
    rules: RuleSet,

    outputs: OutState,

    bots_state: BotState,
    vals_state: ValState,

    bots_new_state: BotState,
    vals_new_state: ValState,
}

impl Factory {
    fn step(&mut self) {
        self.bots_new_state.clear();
        self.vals_new_state.clear();

        for (bot, vals) in self.bots_state.iter() {
            match vals.len() {
                2 => {
                    let lo = vals.iter().min().unwrap();
                    let hi = vals.iter().max().unwrap();

                    let rule = &self.rules[bot];

                    match rule.hi {
                        Output::Bot(bot) => {
                            let entry = self.bots_new_state.entry(bot).or_insert(HashSet::new());
                            entry.insert(*hi);

                            assert!(!self.vals_new_state.contains_key(hi));
                            self.vals_new_state.insert(*hi, bot);
                        }
                        Output::Out(out) => {
                            let entry = self.outputs.entry(out).or_insert(HashSet::new());
                            entry.insert(*hi);
                        }
                    }

                    match rule.lo {
                        Output::Bot(bot) => {
                            let entry = self.bots_new_state.entry(bot).or_insert(HashSet::new());
                            entry.insert(*lo);

                            assert!(!self.vals_new_state.contains_key(lo));
                            self.vals_new_state.insert(*lo, bot);
                        }
                        Output::Out(out) => {
                            let entry = self.outputs.entry(out).or_insert(HashSet::new());
                            entry.insert(*lo);
                        }
                    }
                }
                1 => {
                    let val = *vals.iter().next().unwrap();

                    let entry = self.bots_new_state.entry(*bot).or_insert(HashSet::new());
                    entry.insert(val);

                    assert!(!self.vals_new_state.contains_key(&val));
                    self.vals_new_state.insert(val, *bot);
                }
                _ => panic!(),
            }
        }

        std::mem::swap(&mut self.bots_state, &mut self.bots_new_state);
        std::mem::swap(&mut self.vals_state, &mut self.vals_new_state);
    }
}

fn main() {
    // let example = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    println!("part 1");
    println!(">> input -> {}", part_one(input));

    println!("part 2");
    println!(">> input -> {}", part_two(input));
}

fn parse_input(input: &str) -> Factory {
    let mut bots_state = BotState::new();
    let mut vals_state = ValState::new();
    let mut rules = RuleSet::new();

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        let splits: Vec<&str> = line.split(' ').collect();
        match splits[0] {
            "value" => {
                let bot: Bot = splits[5].parse().unwrap();
                let value: Value = splits[1].parse().unwrap();

                let entry = bots_state.entry(bot).or_insert(HashSet::new());
                entry.insert(value);

                assert!(!vals_state.contains_key(&value));
                vals_state.insert(value, bot);
            }
            "bot" => {
                let lo_val = splits[6].parse().unwrap();
                let lo = match splits[5] {
                    "bot" => Output::Bot(lo_val),
                    "output" => Output::Out(lo_val),
                    _ => panic!(),
                };

                let hi_val = splits[11].parse().unwrap();
                let hi = match splits[10] {
                    "bot" => Output::Bot(hi_val),
                    "output" => Output::Out(hi_val),
                    _ => panic!(),
                };

                let bot: Bot = splits[1].parse().unwrap();
                rules.insert(bot, BotRule { lo, hi });
            }
            _ => panic!(),
        }
    }

    Factory {
        rules,
        outputs: HashMap::new(),
        bots_state,
        vals_state,
        bots_new_state: HashMap::new(),
        vals_new_state: HashMap::new(),
    }
}

fn part_one(input: &str) -> Bot {
    let mut factory = parse_input(input);

    loop {
        let Some(bot_w_17) = factory.vals_state.get(&17) else {
            continue;
        };
        let Some(bot_w_61) = factory.vals_state.get(&61) else {
            continue;
        };
        if bot_w_17 == bot_w_61 {
            return *bot_w_17;
        }
        factory.step();
    }
}

fn part_two(input: &str) -> Bot {
    let mut factory = parse_input(input);

    loop {
        factory.step();

        let Some(out_0) = factory.outputs.get(&0) else {
            continue;
        };
        let Some(out_1) = factory.outputs.get(&1) else {
            continue;
        };
        let Some(out_2) = factory.outputs.get(&2) else {
            continue;
        };

        let out_0 = out_0.iter().next().unwrap();
        let out_1 = out_1.iter().next().unwrap();
        let out_2 = out_2.iter().next().unwrap();

        return out_0 * out_1 * out_2;
    }
}
