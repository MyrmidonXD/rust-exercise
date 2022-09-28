#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    bytes: Vec<usize>,
    costs: Vec<usize>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, m, bytes, costs } = tc;
    let cost_sum: usize = costs.iter().sum();

    let mut memo = vec![0usize; cost_sum+1]; // for each costs in [0, cost_sum], stores maximum memory size

    for i in 0..n {
        for c in (costs[i]..=cost_sum).rev() {
            memo[c] = memo[c].max(memo[c-costs[i]] + bytes[i]);
        }
    }

    let mut min_cost = 0;
    for c in 0..=cost_sum {
        if memo[c] >= m { min_cost = c; break; }
    }

    min_cost
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
    let bytes = inputs.by_ref().take(n).collect();
    let costs = inputs.collect();

    // Solve
    let result = solve(TestCase { n, m, bytes, costs });
    
    let _ = writeln!(stdout, "{result}");
}