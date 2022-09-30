#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    seq: Vec<usize>,
    m: usize,
    queries: Vec<(usize, usize)>,
}

fn solve(tc: TestCase) -> Vec<usize> {
    let TestCase { n, seq, m, queries } = tc;
    
    let mut memo = Vec::with_capacity(n); // memo[s][l] == result of query (s, s+l)
    for i in 0..n {
        memo.push(vec![0usize; n-i]);
    }

    // init memo
    for s in 0..n {
        memo[s][0] = 1;
    }
    for s in 0..n-1 {
        if seq[s] == seq[s+1] {
            memo[s][1] = 1;
        }
    }

    // fill the rest of memo
    for l in 2..n {
        for s in 0..n-l {
            memo[s][l] = if seq[s] == seq[s+l] { memo[s+1][l-2] } else { 0 };
        }
    }

    // generate output
    let mut results = Vec::with_capacity(m);
    for (s, e) in queries {
        let (s, e) = (s-1, e-1);
        results.push(memo[s][e-s]);
    }

    results
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
    let seq = inputs.by_ref().take(n).collect();

    let m = inputs.next().unwrap();
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let s = inputs.next().unwrap();
        let e = inputs.next().unwrap();

        queries.push((s, e));
    }

    // Solve
    let results = solve(TestCase { n, seq, m, queries });
    for result in results {
        let _ = writeln!(stdout, "{result}");
    }
}