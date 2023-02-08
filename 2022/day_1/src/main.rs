use std::io::{self, BufRead, BufReader, Stdin};

fn solve(reader: BufReader<Stdin>) -> (u32, u32) {
    let mut curr = 0u32;
    let mut arr: [u32; 3] = [0, 0, 0];
    for line in reader.lines() {
        let s = match line {
            Ok(v) => v,
            _ => continue,
        };
        if s.is_empty() {
            for i in 0..3 {
                if arr[i] < curr {
                    for j in (i + 1..3).rev() {
                        arr[j] = arr[j - 1];
                    }
                    arr[i] = curr;
                    break;
                }
            }

            curr = 0;
            continue;
        }
        let n: u32 = s.parse().unwrap();
        curr += n;
    }

    (arr[0], arr[0] + arr[1] + arr[2])
}

fn main() {
    let (part1, part2) = solve(BufReader::new(io::stdin()));

    println!("part one: {part1}");
    println!("part two: {part2}");
}
