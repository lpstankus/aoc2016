fn main() {
    let example1 = include_str!("../example1.txt");
    let example2 = include_str!("../example2.txt");
    let example3 = include_str!("../example3.txt");
    let example4 = include_str!("../example4.txt");
    let example5 = include_str!("../example5.txt");
    let example6 = include_str!("../example6.txt");
    let example7 = include_str!("../example7.txt");
    let example8 = include_str!("../example8.txt");
    let input = include_str!("../input.txt");

    println!("part 1");
    println!(">> example1 -> {}", expand(example1, false));
    println!(">> example2 -> {}", expand(example2, false));
    println!(">> example3 -> {}", expand(example3, false));
    println!(">> example4 -> {}", expand(example4, false));
    println!(">> example5 -> {}", expand(example5, false));
    println!(">> example6 -> {}", expand(example6, false));
    println!(">> input    -> {}", expand(input, false));

    println!("part 2");
    println!(">> example3 -> {}", expand(example3, true));
    println!(">> example6 -> {}", expand(example6, true));
    println!(">> example7 -> {}", expand(example7, true));
    println!(">> example8 -> {}", expand(example8, true));
    println!(">> input    -> {}", expand(input, true));
}

fn expand(input: &str, should_recurse: bool) -> usize {
    let input = input.trim().as_bytes();

    let mut ans = 0;
    let mut i = 0;
    while i < input.len() {
        if input[i] as char == '(' {
            let marker = expand_marker(&input[i..], should_recurse);
            i += marker.marker_size + marker.contnt_size;
            ans += marker.expanded_size;
            continue;
        }
        ans += 1;
        i += 1;
    }

    ans
}

struct ExpandedMarker {
    marker_size: usize,
    contnt_size: usize,
    expanded_size: usize,
}

fn expand_marker(input: &[u8], should_recurse: bool) -> ExpandedMarker {
    let marker_size = 1 + input
        .iter()
        .enumerate()
        .find(|x| *x.1 as char == ')')
        .unwrap()
        .0;
    let inner_marker = str::from_utf8(&input[1..marker_size - 1]).unwrap();

    let args: Vec<&str> = inner_marker.split('x').collect();
    assert_eq!(args.len(), 2);

    let contnt_size: usize = args[0].parse().unwrap();
    let reps: usize = args[1].parse().unwrap();

    let mut expanded_size: usize = 0;
    match should_recurse {
        true => {
            let total_size = contnt_size + marker_size;
            let mut i = marker_size;
            while i < total_size {
                if input[i] as char == '(' {
                    let marker = expand_marker(&input[i..total_size], should_recurse);
                    expanded_size += marker.expanded_size;
                    i += marker.marker_size + marker.contnt_size;
                    assert!(i <= total_size);
                    continue;
                }
                expanded_size += 1;
                i += 1;
            }
        }
        false => {
            expanded_size = contnt_size;
        }
    }
    expanded_size *= reps;

    ExpandedMarker {
        marker_size,
        contnt_size,
        expanded_size,
    }
}
