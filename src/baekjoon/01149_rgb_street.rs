#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    costs: Vec<[usize; 3]>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, costs } = tc;
    let mut min_costs = vec![[0, 0, 0]; n];

    min_costs[0] = costs[0];
    for i in 1..n {
        min_costs[i][0] = min_costs[i-1][1].min(min_costs[i-1][2]) + costs[i][0];
        min_costs[i][1] = min_costs[i-1][0].min(min_costs[i-1][2]) + costs[i][1];
        min_costs[i][2] = min_costs[i-1][0].min(min_costs[i-1][1]) + costs[i][2];
    }

    *min_costs[n-1].iter().min().unwrap()
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

    let mut costs = vec![];
    for _ in 0..n {
        let (r, g, b) = (
            inputs.next().unwrap(),
            inputs.next().unwrap(),
            inputs.next().unwrap()
        );
        costs.push([r, g, b]);
    }

    // Solve
    let result = solve(TestCase { n, costs });
    
    let _ = writeln!(stdout, "{result}");
}