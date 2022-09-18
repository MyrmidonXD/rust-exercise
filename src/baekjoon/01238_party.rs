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
    n: usize,
    m: usize,
    x: usize,
    edges: Vec<(usize, usize, usize)>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, x, edges, .. } = tc;
    let x = x - 1;

    let mut adj_list_forward = vec![vec![]; n];
    let mut adj_list_backward = vec![vec![]; n];

    for (u, v, w) in edges {
        adj_list_forward[u-1].push((v-1, w));
        adj_list_backward[v-1].push((u-1, w));
    }

    // Dijkstra on foward direction
    let mut dist_forward = vec![usize::MAX; n];
    dist_forward[x] = 0;

    let mut visited = vec![false; n];
    let mut unvisited = BinaryHeap::with_capacity(n);
    for (i, &d) in dist_forward.iter().enumerate() {
        unvisited.push(MinDist(d, i));
    }

    while let Some(MinDist(d, i)) = unvisited.pop() {
        if d != dist_forward[i] { continue; }

        visited[i] = true;

        for &(j, w) in adj_list_forward[i].iter() {
            if visited[j] { continue; }

            let d_j = d + w;
            if d_j < dist_forward[j] {
                dist_forward[j] = d_j;
                unvisited.push(MinDist(d_j, j));
            }
        }
    }

    // Dijkstra on backward direction
    let mut dist_backward = vec![usize::MAX; n];
    dist_backward[x] = 0;

    for x in &mut visited {
        *x = false;
    }
    for (i, &d) in dist_backward.iter().enumerate() {
        unvisited.push(MinDist(d, i));
    }

    while let Some(MinDist(d, i)) = unvisited.pop() {
        if d != dist_backward[i] { continue; }

        visited[i] = true;

        for &(j, w) in adj_list_backward[i].iter() {
            if visited[j] { continue; }

            let d_j = d + w;
            if d_j < dist_backward[j] {
                dist_backward[j] = d_j;
                unvisited.push(MinDist(d_j, j));
            }
        }
    }

    // find maximum `i` of `dist_forward[i] + dist_backward[i]`
    let mut max = 0;
    for i in 0..n {
        max = max.max(dist_forward[i] + dist_backward[i]);
    }

    max
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
    let x = inputs.next().unwrap();
    
    let mut edges = vec![];
    for _ in 0..m {
        let u = inputs.next().unwrap();
        let v = inputs.next().unwrap();
        let w = inputs.next().unwrap();

        edges.push((u, v, w));
    }

    // Solve
    let result = solve(TestCase { n, m, x, edges });
    
    let _ = writeln!(stdout, "{result}");
}