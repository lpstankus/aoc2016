type Triangle = (u32, u32, u32);

fn main() {
    let example1 = include_str!("../example1.txt");
    let example2 = include_str!("../example2.txt");
    let input = include_str!("../input.txt");

    println!("part 1");
    println!(">> example1 -> {}", part_one(example1));
    println!(">> example2 -> {}", part_one(example2));
    println!(">> input    -> {}", part_one(input));

    println!("part 2");
    println!(">> example2 -> {}", part_two(example2));
    println!(">> input    -> {}", part_two(input));
}

fn parse_tris(input: &str) -> Vec<Triangle> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line
                .split(" ")
                .filter(|x| x.len() != 0)
                .map(|x| x.parse().unwrap())
                .collect();
            assert!(nums.len() == 3);
            (nums[0], nums[1], nums[2])
        })
        .collect()
}

fn part_one(input: &str) -> u32 {
    let tris = parse_tris(input);
    let val: Vec<Triangle> = tris.into_iter().filter(|x| is_valid(x)).collect();
    val.len() as u32
}

fn part_two(input: &str) -> u32 {
    let tris = parse_tris(input);

    let (mut verts, mut soa1, mut soa2) =
        tris.into_iter()
            .fold((vec![], vec![], vec![]), |mut acc, x| {
                acc.0.push(x.0);
                acc.1.push(x.1);
                acc.2.push(x.2);
                return acc;
            });

    verts.append(&mut soa1);
    verts.append(&mut soa2);

    let mut count = 0;
    while !verts.is_empty() {
        let a = verts.pop().unwrap();
        let b = verts.pop().unwrap();
        let c = verts.pop().unwrap();

        let tri = (a, b, c);
        if is_valid(&tri) {
            count += 1;
        }
    }

    count
}

fn is_valid(tri: &Triangle) -> bool {
    !((tri.0 + tri.1 <= tri.2) || (tri.0 + tri.2 <= tri.1) || (tri.1 + tri.2 <= tri.0))
}
