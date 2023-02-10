use std::{
    collections::VecDeque,
    io::{self, BufRead, BufReader, Stdin},
};

fn parse_input(reader: &mut BufReader<Stdin>) -> Vec<VecDeque<char>> {
    let mut initialised = false;
    let mut rv: Vec<VecDeque<char>> = Vec::new();

    loop {
        let mut line = String::new();
        let res = reader.read_line(&mut line);
        if res.is_err() {
            break;
        }

        if !initialised {
            let n_cols = line.len() / 4;
            rv = vec![VecDeque::<char>::new(); n_cols];

            initialised = true;
        }

        if line == "\n" {
            break;
        }

        let l: Vec<char> = line.chars().collect();
        for i in (0..l.len()).step_by(4) {
            match l[i + 1] {
                'A'..='Z' => rv[i / 4].push_front(l[i + 1]),
                _ => continue,
            };
        }
    }

    rv
}

fn parse_move(mut s: String) -> [usize; 3] {
    let mut rv = [0, 0, 0];
    let mut pos = 0;

    let mut back = 0;
    let mut is_number = false;
    s.push(' ');
    for (c, front) in s.chars().zip(0..) {
        if c == ' ' {
            if is_number {
                rv[pos] = s[back..front].parse::<usize>().unwrap_or(0);
                pos += 1;
            }
            is_number = !is_number;
            back = front + 1;
        }
    }

    rv
}

fn solve(mut reader: BufReader<Stdin>, part1: bool) -> String {
    let mut ans = String::new();

    let mut data = parse_input(&mut reader);

    for line in reader.lines() {
        let s = match line {
            Ok(v) => v,
            _ => continue,
        };
        let [n, mut from, mut to] = parse_move(s);

        from -= 1;
        to -= 1;

        let len = data[from].len();
        let mut a = data[from].split_off(len - n);
        if part1 {
            a = a.into_iter().rev().collect();
        }

        data[to].append(&mut a);
    }

    for row in data.iter() {
        ans.push(*row.back().unwrap_or(&' '));
    }

    ans
}

fn main() {
    // true to solve part 1 and false to solve part 2
    let ans = solve(BufReader::new(io::stdin()), false);

    println!("Ans: {ans}");
}
