#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    k: usize,
    items: Vec<(usize, usize)>,
}

fn knapsack_dp(items: &Vec<(usize, usize)>, k: usize) -> usize {
    let mut dp = vec![0usize; k + 1];

    for (w, v) in items {
        for i in (*w..=k).rev() {
            dp[i] = dp[i].max(dp[i-*w] + *v);
        }
    }

    dp[k]
}

fn solve(tc: TestCase) -> usize {
    let items = tc.items;
    let k = tc.k;

    knapsack_dp(&items, k)
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
    let k = inputs.next().unwrap();

    let mut items = vec![];

    for _ in 0..n {
        let w = inputs.next().unwrap();
        let v = inputs.next().unwrap();

        items.push((w, v));
    }

    // Solve
    let result = solve(TestCase { n, k, items });
    
    let _ = writeln!(stdout, "{result}");
}