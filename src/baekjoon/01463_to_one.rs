use std::io::{self, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
}

fn solve(tc: TestCase) -> usize {
    let mut m: Vec<usize> = vec![0; tc.n+1]; // n + 1 for 1-based indexing

    for x in 2..=tc.n {
        let mut prev_min = m[x-1];
        if x % 3 == 0 {
            prev_min = prev_min.min(m[x/3]);
        }
        if x % 2 == 0 {
            prev_min = prev_min.min(m[x/2]);
        }
        m[x] = prev_min + 1;
    }

    m[tc.n]
}

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);

    let n = buffer.trim_end().parse().unwrap();

    let result = solve(TestCase { n });

    println!("{result}");
}