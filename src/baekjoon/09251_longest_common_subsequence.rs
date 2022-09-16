#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    a: String,
    b: String,
}

fn lcs(memo: &mut Vec<Vec<usize>>, a: &[char], b: &[char]) -> usize {
    let n = a.len();
    let m = b.len();

    for i in 1..=n {
        for j in 1..=m {
            if a[i-1] == b[j-1] {
                memo[i][j] = memo[i-1][j-1] + 1;
            } else {
                memo[i][j] = memo[i][j-1].max(memo[i-1][j]);
            }
        }
    }

    memo[n][m]
}

fn solve(tc: TestCase) -> usize {
    let a: Vec<char> = tc.a.chars().collect();
    let b: Vec<char> = tc.b.chars().collect();

    let n = a.len();
    let m = b.len();

    let mut memo = vec![vec![0; m+1]; n+1];

    lcs(&mut memo, &a, &b)
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
    let mut inputs = buffer.lines();

    let a = inputs.next().unwrap().to_string();
    let b = inputs.next().unwrap().to_string();

    // Solve
    let result = solve(TestCase { a, b });
    
    let _ = writeln!(stdout, "{result}");
}