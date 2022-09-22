#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    edges: Vec<(usize, usize, usize)>,
    start: usize,
    end: usize
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, m, edges, start, end } = tc;
    let start = start - 1;
    let end = end - 1;

    let mut adj_list = vec![vec![]; n];
    for (u, v, c) in edges {
        adj_list[u-1].push((v-1, c));
    }

    let mut dist = vec![None; n];
    let mut pq = BinaryHeap::with_capacity(m);

    // dijkstra
    pq.push(Reverse((0, start)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if dist[u].is_some() { continue; }

        dist[u] = Some(d);

        if u == end { break; }
        
        for &(v, c) in &adj_list[u] {
            if dist[v].is_some() { continue; }

            pq.push(Reverse((d+c, v)));
        }
    }

    dist[end].expect("No path found")
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

    let n = inputs.next().unwrap();
    let m = inputs.next().unwrap();

    let mut edges = vec![];
    for _ in 0..m {
        let u = inputs.next().unwrap();
        let v = inputs.next().unwrap();
        let c = inputs.next().unwrap();

        edges.push((u, v, c));
    }

    let start = inputs.next().unwrap();
    let end = inputs.next().unwrap();

    // Solve
    let result = solve(TestCase { n, m, edges, start, end });
    
    let _ = writeln!(stdout, "{result}");
}