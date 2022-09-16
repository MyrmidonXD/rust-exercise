#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::BinaryHeap;
use std::cmp::{ Ordering, Ord, PartialOrd };

#[derive(Debug, PartialEq, Eq)]
struct MinDist(usize, usize); // (distance, vertex_index)

impl Ord for MinDist {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for MinDist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    v: usize,
    e: usize,
    start: usize,
    edges: Vec<(usize, usize, usize)>,
}

fn solve(tc: TestCase) -> impl Iterator<Item=Option<usize>> {
    let TestCase { v, start, edges, .. } = tc;
    
    let start = start - 1;

    let mut adj_list = vec![vec![]; v];
    for (u, v, w) in edges {
        adj_list[u-1].push((v-1, w));
    }

    let inf = 2_000_000_000_usize;
    let mut dist = vec![inf; v];
    dist[start] = 0;

    let mut visited = vec![false; v];

    let mut unvisited = BinaryHeap::with_capacity(v);
    for (i, &d) in dist.iter().enumerate() {
        unvisited.push(MinDist(d, i));
    }

    while let Some(MinDist(d, u)) = unvisited.pop() {
        if dist[u] != d { continue; }

        visited[u] = true;

        for &(v, w) in adj_list[u].iter() {
            if visited[v] { continue; }

            let d_v = d + w;
            if d_v < dist[v] {
                dist[v] = d_v;
                unvisited.push(MinDist(d_v, v));
            }
        }
    }

    dist.into_iter().map(move |x| if x == inf { None } else { Some(x) })
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

    let (v, e) = (inputs.next().unwrap(), inputs.next().unwrap());
    let start = inputs.next().unwrap();

    let mut edges = vec![];
    for _ in 0..e {
        let edge = (
            inputs.next().unwrap(),
            inputs.next().unwrap(),
            inputs.next().unwrap(),
        );

        edges.push(edge);
    }

    // Solve
    let results = solve(TestCase { v, e, start, edges });

    for result in results {
        match result {
            Some(x) => { let _ = writeln!(stdout, "{x}"); },
            None => { let _ = writeln!(stdout, "INF"); }
        }
    }
}