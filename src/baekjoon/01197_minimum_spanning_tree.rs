#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, Ordering};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    v: usize,
    e: usize,
    edges: Vec<(usize, usize, i32)>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    cost: i32,
}

impl Ord for Edge {
    // reverse cmp result so that the heap would be a min-heap
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(tc: TestCase) -> i32 {
    let TestCase { v, e, edges: raw_edges } = tc;

    // Build the graph as an adjacency list
    let mut graph: Vec<Vec<Edge>> = Vec::with_capacity(e);
    for _ in 0..v {
        graph.push(Vec::new());
    }
    for (u, v, c) in raw_edges {
        // map vertex indices from 1-based to 0-based
        let u = u - 1;
        let v = v - 1;

        // add edges as bidirectional
        graph[u].push(Edge { from: u, to: v, cost: c });
        graph[v].push(Edge { from: v, to: u, cost: c });
    }

    let mut visited = vec![false; v];
    let mut heap = BinaryHeap::new();

    let mut cost: i32 = 0;

    // initialize heap
    visited[0] = true;
    for &edge in &graph[0] {
        heap.push(edge);
    }

    // Prim's algorithm
    while let Some(edge) = heap.pop() {
        if visited[edge.to] { continue; }

        visited[edge.to] = true;
        for &next_edge in &graph[edge.to] {
            heap.push(next_edge)
        }

        cost += edge.cost;
    }

    cost
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let v = inputs.next().unwrap() as usize;
    let e = inputs.next().unwrap() as usize;
    let mut edges = Vec::with_capacity(e);
    for _ in 0..e {
        let u = inputs.next().unwrap() as usize;
        let v = inputs.next().unwrap() as usize;
        let c = inputs.next().unwrap();

        edges.push((u, v, c));
    }

    // Solve
    let result = solve(TestCase { v, e, edges });
    
    let _ = writeln!(stdout, "{result}");
}