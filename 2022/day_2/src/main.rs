use std::io::{self, BufRead, BufReader, Stdin};

fn result(a: u32, b: u32) -> u32 {
    if (b > a && b - a == 1) || (a > b && a - b == 2) {
        6
    } else if b == a {
        3
    } else {
        0
    }
}

fn solve(reader: BufReader<Stdin>) -> (u32, u32) {
    let mut part1 = 0u32;
    let mut part2 = 0u32;
    for line in reader.lines() {
        let s = match line {
            Ok(v) => v,
            _ => continue,
        };

        let a = match s.chars().next().unwrap() {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => continue,
        };

        let b = match s.chars().nth(2).unwrap() {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => continue,
        };

        part1 += b + result(a, b);
        part2 += (b - 1) * 3
            + match b {
                1 => (a + 1) % 3 + 1,
                2 => a,
                3 => a % 3 + 1,
                _ => continue,
            }
    }

    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(BufReader::new(io::stdin()));

    println!("part one: {part1}");
    println!("part two: {part2}");
}
