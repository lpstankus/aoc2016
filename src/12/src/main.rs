type Val = i32;
type Reg = usize;

#[derive(Clone, Copy)]
enum Instr {
    CpyReg((Reg, Reg)),
    CpyVal((Val, Reg)),
    Inc(Reg),
    Dec(Reg),
    JnzReg((Reg, Val)),
    JnzVal((Val, Val)),
}

struct Computer {
    pc: Val,
    regs: [Val; 4],
    instrs: Vec<Instr>,
}

impl Computer {
    fn run(&mut self) {
        while 0 <= self.pc && (self.pc as usize) < self.instrs.len() {
            let instr = self.instrs[self.pc as usize];
            match instr {
                Instr::CpyReg((orig, dest)) => {
                    self.regs[dest] = self.regs[orig];
                    self.pc += 1;
                }
                Instr::CpyVal((val, dest)) => {
                    self.regs[dest] = val;
                    self.pc += 1;
                }
                Instr::Inc(reg) => {
                    self.regs[reg] += 1;
                    self.pc += 1;
                }
                Instr::Dec(reg) => {
                    self.regs[reg] -= 1;
                    self.pc += 1;
                }
                Instr::JnzReg((reg, off)) => {
                    if self.regs[reg] != 0 {
                        self.pc += off;
                    } else {
                        self.pc += 1;
                    }
                }
                Instr::JnzVal((val, off)) => {
                    if val != 0 {
                        self.pc += off;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }

    fn set(&mut self, reg: &str, val: Val) {
        let reg = reg_to_idx(reg);
        if reg < self.regs.len() {
            self.regs[reg] = val;
        }
    }

    fn inspect(&self, reg: &str) -> Option<Val> {
        let reg = reg_to_idx(reg);
        match reg < self.regs.len() {
            true => Some(self.regs[reg]),
            false => None,
        }
    }
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

fn reg_to_idx(reg: &str) -> usize {
    match reg {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        val => panic!("not a reg: {}", val),
    }
}

fn parse_input(input: &str) -> Computer {
    let mut instrs = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        let splits: Vec<&str> = line.split(" ").collect();
        let instr = match splits[0] {
            "cpy" => {
                assert_eq!(splits.len(), 3);
                match splits[1].parse() {
                    Ok(val) => Instr::CpyVal((val, reg_to_idx(splits[2]))),
                    Err(_) => Instr::CpyReg((reg_to_idx(splits[1]), reg_to_idx(splits[2]))),
                }
            }
            "inc" => {
                assert_eq!(splits.len(), 2);
                Instr::Inc(reg_to_idx(splits[1]))
            }
            "dec" => {
                assert_eq!(splits.len(), 2);
                Instr::Dec(reg_to_idx(splits[1]))
            }
            "jnz" => {
                assert_eq!(splits.len(), 3);
                let off = splits[2].parse().expect("Invalid jump offset");
                match splits[1].parse() {
                    Ok(val) => Instr::JnzVal((val, off)),
                    Err(_) => Instr::JnzReg((reg_to_idx(splits[1]), off)),
                }
            }
            val => panic!("not an instruction: {}", val),
        };
        instrs.push(instr);
    }

    Computer {
        pc: 0,
        regs: [0; 4],
        instrs,
    }
}

fn part_one(input: &str) -> Option<Val> {
    let mut computer = parse_input(input);
    computer.run();
    computer.inspect("a")
}

fn part_two(input: &str) -> Option<Val> {
    let mut computer = parse_input(input);
    computer.set("c", 1);
    computer.run();
    computer.inspect("a")
}
