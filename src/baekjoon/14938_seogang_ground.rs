#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    r: usize,
    item_counts: Vec<usize>,
    edges: Vec<(usize, usize, usize)>,
}

fn dijkstra_bounded(
    adj_list: &Vec<Vec<(usize, usize)>>,
    pq: &mut BinaryHeap<Reverse<(usize, usize)>>,
    dist: &mut Vec<Option<usize>>,
    start: usize,
    dist_bound: usize
) {
    // initialize pq and dist
    pq.clear();
    for v in dist.iter_mut() {
        *v = None;
    }

    pq.push(Reverse((0, start)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist_bound { break; }
        if dist[u].is_some() { continue; }

        dist[u] = Some(d);

        for &(v, l) in &adj_list[u] {
            if dist[v].is_some() { continue; }

            pq.push(Reverse((d + l, v)));
        }
    }
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, m, item_counts, edges, .. } = tc;
    
    let mut adj_list = vec![vec![]; n];
    for (a, b, l) in edges {
        adj_list[a-1].push((b-1, l));
        adj_list[b-1].push((a-1, l));
    }
    let mut dist = vec![None; n];
    let mut pq = BinaryHeap::with_capacity(n*n);

    let mut max_item_count = 0;

    for i in 0..n {
        dijkstra_bounded(&adj_list, &mut pq, &mut dist, i, m);

        let item_count = dist.iter().enumerate().fold(0, |acc, (j, d)| if d.is_some() { acc + item_counts[j] } else { acc });
        max_item_count = max_item_count.max(item_count);
    }

    max_item_count
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
    let r = inputs.next().unwrap();

    let item_counts = inputs.by_ref().take(n).collect();
    
    let mut edges = vec![];
    for _ in 0..r {
        let a = inputs.next().unwrap();
        let b = inputs.next().unwrap();
        let l = inputs.next().unwrap();

        edges.push((a, b, l))
    }

    // Solve
    let result = solve(TestCase { n, m, r, item_counts, edges });
    
    let _ = writeln!(stdout, "{result}");
}