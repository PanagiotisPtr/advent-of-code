use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader, Stdin},
};

fn solve(reader: BufReader<Stdin>) -> (u32, u32) {
    let mut part1 = 0u32;
    let mut part2 = 0u32;

    let mut map: HashMap<char, u32> = HashMap::new();
    for (line, n) in reader.lines().zip(1..) {
        let s = match line {
            Ok(v) => v,
            _ => continue,
        };

        let mut set: HashSet<char> = HashSet::new();
        let mut all_set: HashSet<char> = HashSet::new();

        for (c, i) in s.chars().zip(0..s.len()) {
            all_set.insert(c);

            let char_val = match c {
                'A'..='Z' => 27u32 + (c as u32) - ('A' as u32),
                'a'..='z' => 1u32 + (c as u32) - ('a' as u32),
                _ => continue,
            };

            if i >= s.len() / 2 && set.contains(&c) {
                part1 += char_val;
            }
            if i < s.len() / 2 {
                set.insert(c);
            }
        }

        for c in all_set {
            let mut count = 1;
            if let Some(val) = map.get(&c) {
                count = val + 1;
            }
            map.insert(c, count);

            if count == 3 {
                part2 += match c {
                    'A'..='Z' => 27u32 + (c as u32) - ('A' as u32),
                    'a'..='z' => 1u32 + (c as u32) - ('a' as u32),
                    _ => continue,
                };
            }
        }

        if n % 3 == 0 {
            map.clear();
        }
    }

    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(BufReader::new(io::stdin()));

    println!("part one: {part1}");
    println!("part two: {part2}");
}
