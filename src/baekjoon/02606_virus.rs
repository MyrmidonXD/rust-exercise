use std::io::{self, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    edges: Vec<(usize, usize)>
}

fn dfs(adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, i: usize) -> usize {
    visited[i] = true;

    let mut visited_count = 1;
    for &j in adj_list[i].iter() {
        if !visited[j] {
            visited_count += dfs(adj_list, visited, j);
        }
    }

    visited_count
}

fn solve(tc: TestCase) -> usize {
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); tc.n];
    let mut visited: Vec<bool> = vec![false; tc.n];

    for &(u, v) in tc.edges.iter() {
        adj_list[u-1].push(v-1);
        adj_list[v-1].push(u-1);
    }
    
    dfs(&adj_list, &mut visited, 0) - 1
}


#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);
    let mut lines = buffer.lines();

    let n = lines.next().unwrap().parse().unwrap();
    let m = lines.next().unwrap().parse().unwrap();

    let mut edges = Vec::new();
    while let Some(line) = lines.next() {
        let v: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        edges.push((v[0], v[1]));
    }

    let result = solve(TestCase { n, m, edges });
    println!("{result}");
}