use std::io::{self, BufRead};
use std::convert::TryInto;

struct TestCase {
    n: u32,
    r: u32,
    c: u32
}

fn rec_calc(acc: u32, n: u32, r: u32, c: u32) -> u32 {
    if n == 0 { return acc } // base case

    let mid = u32::pow(2, n-1);
    match (r, c) {
        (r, c) if r <  mid && c <  mid  => rec_calc(4 * acc,     n - 1, r,       c),
        (r, c) if r <  mid && c >= mid  => rec_calc(4 * acc + 1, n - 1, r,       c - mid),
        (r, c) if r >= mid && c <  mid  => rec_calc(4 * acc + 2, n - 1, r - mid, c),
        _                               => rec_calc(4 * acc + 3, n - 1, r - mid, c - mid)
    }
}

fn solve(tc: TestCase) {
    let result = rec_calc(0, tc.n, tc.r, tc.c);
    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let [n, r, c]: [u32; 3] = lines.next().unwrap().expect("IO error")
                                    .split(' ').map(|s| s.parse::<u32>().unwrap())
                                    .collect::<Vec<u32>>().try_into().unwrap();

    solve(TestCase { n, r, c });
}