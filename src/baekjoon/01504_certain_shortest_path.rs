#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    e: usize,
    edges: Vec<(usize, usize, usize)>,
    v1: usize,
    v2: usize,
}

fn solve(tc: TestCase) -> Option<usize> {
    let TestCase { n, edges, v1, v2, .. } = tc;

    let mut adj_list = vec![vec![]; n];
    for (u, v, c) in edges {
        adj_list[u].push((v, c));
        adj_list[v].push((u, c));
    }

    let mut priority_queue = BinaryHeap::new();

    // 1st Dijkstra: to find 1 -> v1, 1 -> v2 shortest distances
    let mut dist_1 = vec![None; n];
    priority_queue.push(Reverse((0, 0)));

    while let Some(Reverse((d, u))) = priority_queue.pop() {
        if dist_1[u].is_some() { continue; }

        dist_1[u] = Some(d);

        for &(v, c) in &adj_list[u] {
            if dist_1[v].is_some() { continue; }

            priority_queue.push(Reverse((d+c, v)));
        }

        // early break on whether v1 and v2 has visited
        // because we only need shortest distance of 1 -> v1 and 1 -> v2
        if dist_1[v1].is_some() && dist_1[v2].is_some() {
            break;
        }
    }

    // early return when v1 and v2 are both unreachable from vertex 1
    if dist_1[v1].is_none() && dist_1[v2].is_none() {
        return None;
    }

    priority_queue.clear();

    // 2nd Dijkstra: to find v1 -> v2, v1 -> n shortest distances
    let mut dist_v1 = vec![None; n];
    priority_queue.push(Reverse((0, v1)));

    while let Some(Reverse((d, u))) = priority_queue.pop() {
        if dist_v1[u].is_some() { continue; }

        dist_v1[u] = Some(d);

        for &(v, c) in &adj_list[u] {
            if dist_v1[v].is_some() { continue; }

            priority_queue.push(Reverse((d+c, v)));
        }

        // early break on whether v2 and n has visited
        // because we only need shortest distance of v1 -> v2 and v1 -> n
        if dist_v1[v2].is_some() && dist_v1[n-1].is_some() {
            break;
        }
    }

    // early return when v2 is unreachable from v1
    if dist_v1[v2].is_none() {
        return None;
    }

    priority_queue.clear();

    // 3rd Dijkstra: to find v2 -> n shortest distance
    let mut dist_v2 = vec![None; n];
    priority_queue.push(Reverse((0, v2)));

    while let Some(Reverse((d, u))) = priority_queue.pop() {
        if dist_v2[u].is_some() { continue; }

        dist_v2[u] = Some(d);

        for &(v, c) in &adj_list[u] {
            if dist_v2[v].is_some() { continue; }

            priority_queue.push(Reverse((d+c, v)));
        }

        // early break on whether n has visited
        // because we only need shortest distance of v2 -> n
        if dist_v2[n-1].is_some() {
            break;
        }
    }

    // 1 -> v1 -> v2 -> n
    let a1 = dist_1[v1];
    let b = dist_v1[v2];
    let c1 = dist_v2[n-1];

    // 1 -> v2 -> v1 -> n
    let a2 = dist_1[v2];
    let c2 = dist_v1[n-1];

    if c1.is_some() && c2.is_some() {
        Some((a1.unwrap() + b.unwrap() + c1.unwrap()).min(a2.unwrap() + b.unwrap() + c2.unwrap()))
    } else if c1.is_some() {
        Some(a1.unwrap() + b.unwrap() + c1.unwrap())
    } else if c2.is_some() {
        Some(a2.unwrap() + b.unwrap() + c2.unwrap())
    } else {
        None
    }
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
    let e = inputs.next().unwrap();
    
    let mut edges = vec![];
    for _ in 0..e {
        let u = inputs.next().unwrap() - 1;
        let v = inputs.next().unwrap() - 1;
        let c = inputs.next().unwrap();

        edges.push((u, v, c));
    }

    let v1 = inputs.next().unwrap() - 1;
    let v2 = inputs.next().unwrap() - 1;

    // Solve
    let result = solve(TestCase { n, e, edges, v1, v2 });
    
    match result {
        Some(n) => { let _ = writeln!(stdout, "{n}",); },
        None => { let _ = writeln!(stdout, "-1",); },
    }
}