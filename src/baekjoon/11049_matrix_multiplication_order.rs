#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    matrices: Vec<(usize, usize)>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, matrices } = tc;
    let mut memo = vec![vec![0; n]; n];

    for d in 1..n {
        for i in 0..(n-d) {
            memo[i][i+d] = usize::MAX;
            for j in i..(i+d) {
                let cost = matrices[i].0 * matrices[j].1 * matrices[i+d].1;
                memo[i][i+d] = memo[i][i+d].min(memo[i][j] + memo[j+1][i+d] + cost);
            }
        }
    }

    memo[0][n-1]
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
    let mut matrices = vec![];
    for _ in 0..n {
        let r = inputs.next().unwrap();
        let c = inputs.next().unwrap();
        matrices.push((r, c));
    }

    // Solve
    let result = solve(TestCase { n, matrices });
    
    let _ = writeln!(stdout, "{result}");
}