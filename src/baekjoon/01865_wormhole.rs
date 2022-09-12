#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    w: usize,
    edges: Vec<((usize, usize), i32)>,
}

fn solve(tc: TestCase) -> bool {
    // Bellman-Ford algorithm to check negative cycles
    let inf: i32 = 2_000_000_000;
    let n = tc.n;
    let edges = tc.edges;
    let mut dist = vec![inf; n];

    // starting from 1st vertex is okay for checking negative cycles,
    // because we do update dist[v] even when dist[u] == inf
    // which is not the case in original Bellman-Ford algorithm for calculating a shortest path
    dist[0] = 0;
    for _ in 0..n-1 {
        for &((u, v), w) in edges.iter() {
            if dist[u-1] + w < dist[v-1] {
                dist[v-1] = dist[u-1] + w;
            }
        }
    }

    // checking negative cycles
    for &((u, v), w) in edges.iter() {
        if dist[u-1] + w < dist[v-1] {
            return true;
        }
    }

    false
}

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut buffer = String::new();

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    // Parsing
    let _ = stdin.read_to_string(&mut buffer);
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    let t = inputs.next().unwrap();

    for _ in 0..t {
        let n = inputs.next().unwrap();
        let m = inputs.next().unwrap();
        let w = inputs.next().unwrap();

        let mut edges = HashMap::new();
        for _ in 0..m {
            let u = inputs.next().unwrap();
            let v = inputs.next().unwrap();
            let w = inputs.next().unwrap() as i32;

            edges.entry((u, v)).and_modify(|prev_w| { *prev_w = if w < *prev_w { w } else { *prev_w } }).or_insert(w);
            edges.entry((v, u)).and_modify(|prev_w| { *prev_w = if w < *prev_w { w } else { *prev_w } }).or_insert(w);
        }

        for _ in 0..w {
            let u = inputs.next().unwrap();
            let v = inputs.next().unwrap();
            let w = -(inputs.next().unwrap() as i32);

            edges.entry((u, v)).and_modify(|prev_w| { *prev_w = if w < *prev_w { w } else { *prev_w } }).or_insert(w);
        }

        let edges = edges.into_iter().collect();

        // Solve
        let result = solve(TestCase { n, m, w, edges });
        if result {
            let _ = writeln!(stdout, "YES");
        } else {
            let _ = writeln!(stdout, "NO");
        }
    }
}