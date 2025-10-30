use hex;
use md5::{Digest, Md5};

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

fn part_one(input: &str) -> String {
    let id = input.trim().to_string();

    let mut ans = String::new();

    let mut i = 0;
    for _ in 0..8 {
        let mut hash;
        loop {
            let key = id.clone() + i.to_string().as_str();

            let mut hasher = Md5::new();
            hasher.update(key.as_bytes());
            hash = hex::encode(hasher.finalize());

            i += 1;
            if hash.starts_with("00000") {
                break;
            }
        }
        ans.push(hash.chars().nth(5).unwrap());
    }

    ans
}

fn part_two(input: &str) -> String {
    let id = input.trim().to_string();

    let mut ans = vec!['-'; 8];

    let mut i = 0;
    let mut count = 0;
    while count < 8 {
        let mut hash;
        loop {
            let key = id.clone() + i.to_string().as_str();

            let mut hasher = Md5::new();
            hasher.update(key.as_bytes());
            hash = hex::encode(hasher.finalize());

            i += 1;
            if hash.starts_with("00000") {
                break;
            }
        }

        let mut chars = hash.chars();
        let pos = chars.nth(5);
        let val = chars.nth(0);

        if let (Some(pos), Some(val)) = (pos, val) {
            if pos < '0' || pos > '7' {
                continue;
            }
            let pos = (pos as u8 - '0' as u8) as usize;
            if ans[pos] != '-' {
                continue;
            }
            ans[pos] = val;
            count += 1;
        }
    }

    String::from_iter(ans)
}
