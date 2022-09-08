use std::io::{self, BufRead};
use std::convert::TryInto;
use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: usize,
    meetings: Vec<(usize, usize)>
}

fn solve(tc: TestCase) {
    let mut meetings = tc.meetings;
    meetings.sort_by(|a, b| if a.1 < b.1 { Ordering::Less } else if a.1 > b.1 { Ordering::Greater } else { a.0.cmp(&b.0) } );

    let mut last_end: usize = 0;
    let mut result = 0;

    for (s, e) in meetings {
        if last_end <= s {
            result += 1;
            last_end = e;
        }
    }

    println!("{result}");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n = lines.next().unwrap().expect("IO error").parse().unwrap();

    let mut meetings = Vec::new();
    for _ in 0..n {
        let [s, e]: [usize; 2] = lines.next().unwrap().expect("IO error").split_whitespace()
                                    .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                                    .try_into().unwrap();
        meetings.push((s, e))
    }
    
    solve(TestCase { n, meetings });
}