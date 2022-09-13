#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    v: usize,
    adj_list: Vec<Vec<(usize, usize)>>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { v, adj_list } = tc;

    let mut visited = vec![false; v];
    let mut dist = vec![0; v];

    // 1st dfs: find farthest node from node 0
    let mut stack = vec![0usize];
    visited[0] = true;

    while let Some(i) = stack.pop() {
        let edges = &adj_list[i];
        for &(j, w) in edges.iter() {
            if !visited[j] {
                dist[j] = dist[i] + w;

                stack.push(j);
                visited[j] = true;
            }
        }
    }

    let mut max_i = 0usize;
    for (i, &d) in dist.iter().enumerate() {
        if d > dist[max_i] {
            max_i = i;
        }
    }

    // 2nd dfs: find longest distance from node `max_i`
    visited = vec![false; v];
    dist = vec![0; v];

    stack.push(max_i);
    visited[max_i] = true;

    while let Some(i) = stack.pop() {
        let edges = &adj_list[i];
        for &(j, w) in edges.iter() {
            if !visited[j] {
                dist[j] = dist[i] + w;

                stack.push(j);
                visited[j] = true;
            }
        }
    }

    dist.into_iter().max().unwrap()
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
    let mut adj_list = vec![vec![]; v];

    for _ in 0..v {
        let i = (inputs.next().unwrap() as usize) - 1;

        let mut j = inputs.next().unwrap();
        while j != -1 {
            let w = inputs.next().unwrap() as usize;
            adj_list[i].push(((j-1) as usize, w));

            j = inputs.next().unwrap();
        }
    }

    // Solve
    let result = solve(TestCase { v, adj_list });
    
    let _ = writeln!(stdout, "{result}");
}