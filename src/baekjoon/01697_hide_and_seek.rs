use std::io::{self, BufRead};
use std::convert::TryInto;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: usize,
    k: usize,
}

fn solve(tc: TestCase) {
    let mut distance = [usize::MAX; 100001];
    let mut queue = VecDeque::new();

    if tc.n != tc.k {
        queue.push_back(tc.n);
    }
    distance[tc.n] = 0;

    while let Some(x) = queue.pop_front() {
        if x - 1 == tc.k || x + 1 == tc.k || 2 * x == tc.k {
            distance[tc.k] = distance[x] + 1;
            break;
        }

        // X - 1
        if x > 0 && distance[x-1] == usize::MAX {
            queue.push_back(x-1);
            distance[x-1] = distance[x] + 1;
        }

        // X + 1
        if x < 100000 && distance[x+1] == usize::MAX {
            queue.push_back(x+1);
            distance[x+1] = distance[x] + 1;
        }

        // 2 * X
        if x <= 50000 && distance[2*x] == usize::MAX {
            queue.push_back(2*x);
            distance[2*x] = distance[x] + 1;
        }
    }

    println!("{}", distance[tc.k]);

}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let [n, k]: [usize; 2] = lines.next().unwrap().expect("IO error").split_whitespace()
                                .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                                .try_into().unwrap();
    
    solve(TestCase { n, k });
}