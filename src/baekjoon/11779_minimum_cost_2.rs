#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::fmt;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct VecFmt<'a, T>(&'a Vec<T>);
impl<'a, T: fmt::Display> fmt::Display for VecFmt<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() == 0 { return write!(f, "") }

        for (i, e) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", *e)?;
            } else {
                write!(f, " {}", *e)?;
            }
        }

        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    edges: Vec<(usize, usize, usize)>,
    start: usize,
    end: usize,
}

fn solve(tc: TestCase) -> (usize, usize, Vec<usize>) {
    let TestCase { n, edges, start, end, .. } = tc;

    let mut adj_list = vec![vec![]; n+1];
    for (u, v, c) in edges {
        adj_list[u].push((v, c));
    }

    let mut dist = vec![None; n+1];
    let mut prev = vec![None; n+1];

    let mut queue = BinaryHeap::with_capacity(n);
    queue.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = queue.pop() {
        if dist[u].is_some() && dist[u] != Some(d) { continue; }

        dist[u] = Some(d);

        for &(v, c) in &adj_list[u] {
            let d_v = d + c;
            if dist[v].is_none() || d_v < dist[v].unwrap() {
                queue.push(Reverse((d_v, v)));
                dist[v] = Some(d_v);
                prev[v] = Some(u);
            }
        }
    }

    let cost = dist[end].unwrap();
    let mut path = vec![end];
    while let Some(u) = prev[path[path.len() - 1]] {
        path.push(u);
    }

    (cost, path.len(), path.into_iter().rev().collect())
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
    
    let _ = writeln!(stdout, "{}", result.0);
    let _ = writeln!(stdout, "{}", result.1);
    let _ = writeln!(stdout, "{}", VecFmt(&result.2));
}