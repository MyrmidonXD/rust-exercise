#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    edges: Vec<(usize, usize)>,
}

fn solve(tc: TestCase) -> impl Iterator<Item=usize> {
    let TestCase { n, edges } = tc;
    let mut adj_list = vec![vec![]; n];

    for (u, v) in edges {
        adj_list[u-1].push(v-1);
        adj_list[v-1].push(u-1);
    }

    let mut parent = vec![None; n];
    let mut stack = vec![];

    stack.push(0);
    parent[0] = Some(0);

    while let Some(i) = stack.pop() {
        for &j in adj_list[i].iter() {
            match parent[j] {
                Some(_) => continue,
                None => { stack.push(j); parent[j] = Some(i) },
            }
        }
    }

    parent.into_iter().map(|x| x.unwrap() + 1).skip(1)
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

    let mut edges = vec![];
    for _ in 0..n-1 {
        edges.push((inputs.next().unwrap(), inputs.next().unwrap()));
    }

    // Solve
    for result in solve(TestCase { n, edges }) {
        let _ = writeln!(stdout, "{result}");
    }
}