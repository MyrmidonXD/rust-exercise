use std::io::{self, BufRead};
use std::convert::TryInto;

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: usize,
    m: usize,
    edges: Vec<(usize, usize)>
}

fn dfs(adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, i: usize) {
    visited[i] = true;

    for &j in &adj_list[i] {
        if !visited[j] {
            dfs(adj_list, visited, j);
        }
    }
}

fn solve(tc: TestCase) {
    let mut visited = vec![false; tc.n];
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); tc.n];

    // Build adjacent list
    for (u, v) in tc.edges {
        adj_list[u-1].push(v-1);
        adj_list[v-1].push(u-1);
    }

    let mut result = 0;
    for i in 0..tc.n {
        if !visited[i] {
            result += 1;
            dfs(&adj_list, &mut visited, i);
        }
    }

    println!("{result}");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let [n, m]: [usize; 2] = lines.next().unwrap().expect("IO error")
                                  .split_whitespace().map(|s| s.parse::<usize>().unwrap())
                                  .collect::<Vec<usize>>().try_into().unwrap();

    let mut edges = Vec::new();
    while let Some(line) = lines.next() {
        let [u, v]: [usize; 2] = line.expect("IO error")
                                    .split_whitespace().map(|s| s.parse::<usize>().unwrap())
                                    .collect::<Vec<usize>>().try_into().unwrap();
        
        edges.push((u, v));
    }

    solve(TestCase { m, n, edges });
}