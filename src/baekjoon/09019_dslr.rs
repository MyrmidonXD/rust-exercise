use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::convert::TryInto;

#[derive(Debug)]
struct TestCase {
    a: usize,
    b: usize,
}

fn bfs_dslr(a: usize, b: usize) -> String {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited: [(usize, char); 10000] = [(10001, ' '); 10000];

    queue.push_back(a);
    visited[a] = (a, '-');

    while !queue.is_empty() {
        let n = queue.pop_front().unwrap();

        let d = (2 * n) % 10000;
        if visited[d].1 == ' ' {
            visited[d] = (n, 'D');

            if d == b { break; }

            queue.push_back(d);
        }

        let s = if n == 0 { 9999 } else { n - 1 };
        if visited[s].1 == ' ' {
            visited[s] = (n, 'S');

            if s == b { break; }

            queue.push_back(s);
        }

        let l = (n % 1000) * 10 + (n / 1000);
        if visited[l].1 == ' ' {
            visited[l] = (n, 'L');

            if l == b { break; }

            queue.push_back(l);
        }

        let r = (n % 10) * 1000 + (n / 10);
        if visited[r].1 == ' ' {
            visited[r] = (n, 'R');

            if r == b { break; }

            queue.push_back(r);
        }
    }

    // backtrace on visited array (b -> a)
    let mut seq = String::new();
    let mut curr = b;
    while curr != a {
        let (before, direction) = visited[curr];
        seq.push(direction);

        curr = before;
    }

    return seq.chars().rev().collect();
}

fn solve(tc: TestCase) {
    let result = bfs_dslr(tc.a, tc.b);
    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().expect("IO error").parse().unwrap();

    for _ in 0..t {
        let [a, b]: [usize; 2] = lines.next().unwrap().expect("IO error")
                                    .split_whitespace().map(|s| s.parse::<usize>().unwrap())
                                    .collect::<Vec<usize>>().try_into().unwrap();

        solve(TestCase { a, b })
    }
}