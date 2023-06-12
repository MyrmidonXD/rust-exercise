#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    edges: Vec<Vec<usize>>,
    plan: Vec<usize>,
}

fn union(disjoint_set: &mut [usize], a_index: usize, b_index: usize, buffer: &mut Vec<usize>) {
    let a_root = find(disjoint_set, a_index, buffer);
    let b_root = find(disjoint_set, b_index, buffer);

    if a_root < b_root {
        disjoint_set[b_root] = disjoint_set[a_root];
    } else if a_root > b_root {
        disjoint_set[a_root] = disjoint_set[b_root];
    }
}

fn find(disjoint_set: &mut [usize], index: usize, buffer: &mut Vec<usize>) -> usize {
    let mut i = index;

    while disjoint_set[i] != i {
        buffer.push(i);
        i = disjoint_set[i];
    }

    for &j in buffer.iter() {
        disjoint_set[j] = i;
    }
    buffer.clear();

    i
}

fn solve(tc: TestCase) -> bool {
    let TestCase { n, m, edges, plan } = tc;
    if n == 0 || m == 0 { return false; }
    
    let mut disjoint_set: Vec<usize> = (0..=n).collect();
    let mut buffer = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if edges[i][j] == 1 {
                union(&mut disjoint_set, i + 1, j + 1, &mut buffer); 
            }
        }
    }

    let start_set = find(&mut disjoint_set, plan[0], &mut buffer);
    plan.iter().all(|&city| find(&mut disjoint_set, city, &mut buffer) == start_set)
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

    let mut edges = Vec::with_capacity(n);
    for _ in 0..n {
        let mut edges_row = Vec::with_capacity(n);
        for _ in 0..n {
            edges_row.push(inputs.next().unwrap());
        }
        edges.push(edges_row);
    }

    let plan = inputs.collect();

    // Solve
    let result = solve(TestCase { n, m, edges, plan });
    
    let _ = writeln!(stdout, "{}", if result { "YES" } else { "NO" });
}