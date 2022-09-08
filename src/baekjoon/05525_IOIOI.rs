use std::io::{self, BufRead};

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: usize,
    m: usize,
    s: String,
}

fn solve(tc: TestCase) {
    let mut cs = tc.s.chars();

    let mut valid = false;
    let mut next = 'I';
    let mut seq_len: usize = 0;
    let mut result: usize = 0;

    while let Some(c) = cs.next() {
        if !valid && c == next {
            valid = true;
            next = 'O';
        } else if c == next {
            if c == 'I' {
                next = 'O';
                seq_len += 1;
            } else {
                next = 'I';
            }
        } else if valid {
            if c == 'O' {
                valid = false;
                next = 'I';
            } else {
                next = 'O'
            }

            result += (seq_len + 1).saturating_sub(tc.n);
            seq_len = 0;
        }
    }

    if valid {
        result += (seq_len + 1).saturating_sub(tc.n);
    }

    println!("{result}");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n = lines.next().unwrap().expect("IO error").parse().unwrap();
    let m = lines.next().unwrap().expect("IO error").parse().unwrap();
    let s = lines.next().unwrap().expect("IO error");
    
    solve(TestCase { n, m, s });
}