use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);
    let mut ps: Vec<usize> = buffer.split_whitespace()
                                    .skip(1)
                                    .map(|s| s.parse().unwrap())
                                    .collect();
    ps.sort_unstable();

    let sum: usize = ps.iter().scan(0, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    }).sum();

    println!("{sum}");
}