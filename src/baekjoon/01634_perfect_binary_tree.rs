#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    k: usize,
    t1: Vec<usize>,
    t2: Vec<usize>,
}

/*
    k = 1일 때
    (0 1), (1 0) => { 0, 1 }. 항상 path 길이 같다

    k = 2일 때
    (0 1 2 3) (0 1 3 2) => { 0, 1, 2, 3 }
    (0 1 2 3) (1 2 0 3) => { 0, 2 } or { 1, 3 }

    k = 4
    1 2 3 4 5 6 7 8
    1 2 3 5 4 6 7 8

    (1, 2) -> o  (2, 3) -> o  (3, 4) -> x  (4, 5) -> o  (5, 6) -> x  (6, 7) -> o  (7, 8) -> o
    (1, 3) -> o  (2, 4) -> x  (3, 5) -> x  (4, 6) -> x  (5, 7) -> x  (6, 8) -> o
    (1, 4) -> x  (2, 5) -> x  (3, 6) -> o  (4, 7) -> x  (5, 8) -> x
    (1, 5) -> x  (2, 6) -> o  (3, 7) -> o  (4, 8) -> x
    (1, 6) -> o  (2, 7) -> o  (3, 8) -> o
    (1, 7) -> o  (2, 8) -> o
    (1, 8) -> o

 */

fn calc_dist(ix: usize, iy: usize, k: usize) -> usize {
    if k <= 1 { return 0; }

    let mid = 2usize.pow((k-2) as u32);

    if ix < mid && iy < mid {
        calc_dist(ix, iy, k-1)
    } else if ix >= mid && iy >= mid {
        calc_dist(ix-mid, iy-mid, k-1)
    } else {
        2 * (k-1)
    }
}

fn solve(tc: TestCase) -> usize {
    let TestCase { k, t1, t2 } = tc;
    if k == 1 { return 1; }

    let n = 2usize.pow((k-1) as u32);
    let mut v1 = vec![0usize; n];
    let mut v2 = vec![0usize; n];

    for i in 0..n {
        v1[t1[i]-1] = i;
        v2[t2[i]-1] = i;
    }

    let mut graph = vec![vec![]; n];
    for x in 0..(n-1) {
        for y in x+1..n {
            if calc_dist(v1[x], v1[y], k) == calc_dist(v2[x], v2[y], k) {
                graph[x].push(y);
            }
        }
    }

    let mut max = 1;
    let mut visited = vec![false; n];
    for x in 0..n {
        max = max.max(dfs_count(&graph, &mut visited, x));
    }

    max
}

fn dfs_count(graph: &Vec<Vec<usize>>, visited: &mut [bool], node: usize) -> usize {
    let mut count = 0;

    if !visited[node] {
        visited[node] = true;
        count = 1;

        for &next in graph[node].iter() {
            count += dfs_count(graph, visited, next);
        }
    }

    count
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

    let k = inputs.next().unwrap();
    let t1 = inputs.by_ref().take(2_usize.pow((k-1) as u32)).collect();
    let t2 = inputs.collect();

    // Solve
    let result = solve(TestCase { k, t1, t2 });
    
    let _ = writeln!(stdout, "{result}");
}