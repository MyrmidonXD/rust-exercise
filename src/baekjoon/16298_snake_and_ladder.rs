use std::io::{self, BufRead};
use std::convert::{TryInto};
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: usize,
    m: usize,
    ladders: Vec<(usize, usize)>,
    snakes: Vec<(usize, usize)>,
}

fn solve(tc: TestCase) {
    let mut visited: [bool; 100] = [false; 100];
    let mut distance: [usize; 100] = [0; 100];
    let mut redirect: [usize; 100] = (0..100).collect::<Vec<_>>().try_into().unwrap();
    let mut queue: VecDeque<usize> = VecDeque::new();

    // set redirection (ladders and snakes)
    for (x, y) in tc.ladders {
        redirect[x-1] = y-1;
    }
    for (u, v) in tc.snakes {
        redirect[u-1] = v-1;
    }

    // set starting point
    visited[0] = true;
    queue.push_back(0);

    // do the BFS
    while let Some(curr) = queue.pop_front() {
        for face in 1..=6 {
            let mut next = curr + face; // roll the dice and get the next cell index
            if next == 99 {
                println!("{}", distance[curr] + 1);
                return;
            }

            next = redirect[next]; // follow a ladder or a snake if it has one
            if !visited[next] {
                distance[next] = distance[curr] + 1;

                queue.push_back(next);
                visited[next] = true;
            }
        }
    }

    println!("Cannot reach the 100th cell, this should not happen!");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let [n, m]: [usize; 2] = lines.next().unwrap().expect("IO error").split_whitespace()
                                    .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                                    .try_into().unwrap();

    let mut ladders = Vec::new();
    for _ in 0..n {
        let [x, y]: [usize; 2] = lines.next().unwrap().expect("IO error").split_whitespace()
                                    .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                                    .try_into().unwrap();
        ladders.push((x, y));
    }

    let mut snakes = Vec::new();
    for _ in 0..m {
        let [u, v]: [usize; 2] = lines.next().unwrap().expect("IO error").split_whitespace()
                                    .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                                    .try_into().unwrap();
        snakes.push((u, v));
    }

    solve(TestCase { n, m, ladders, snakes });
}