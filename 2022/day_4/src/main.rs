use std::io::{self, BufRead, BufReader, Stdin};

fn parse_input(s: String) -> [(u32, u32); 2] {
    let mut arr: [u32; 4] = [0, 0, 0, 0];
    let mut pos = 0usize;

    let mut carry = String::new();
    for c in s.chars() {
        match c {
            '-' | ',' => {
                if let Ok(v) = carry.parse::<u32>() {
                    arr[pos] = v;
                }
                pos += 1;
                carry.clear();
            }
            _ => carry.push(c),
        };
    }
    if let Ok(v) = carry.parse::<u32>() {
        arr[pos] = v;
    }

    [(arr[0], arr[1]), (arr[2], arr[3])]
}

fn solve(reader: BufReader<Stdin>) -> (u32, u32) {
    let mut part1 = 0u32;
    let mut part2 = 0u32;

    for line in reader.lines() {
        let s = match line {
            Ok(v) => v,
            _ => continue,
        };

        let [(mut a, mut b), (mut c, mut d)] = parse_input(s);
        if a > c || (a == c && b < d) {
            (a, b, c, d) = (c, d, a, b);
        }

        if a <= c && b >= d {
            part1 += 1;
        }

        if (a <= c && c <= b) || (a <= d && d <= b) {
            part2 += 1;
        }
    }

    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(BufReader::new(io::stdin()));

    println!("part one: {part1}");
    println!("part two: {part2}");
}
