#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
}

// using Pascal's triangle
fn solve(tc: TestCase) -> u128 {
    let TestCase { n, m } = tc;

    let mut triangle = Vec::new();

    for i in 0..=n {
        triangle.push(vec![1u128; i + 1]);

        for j in 1..i {
            triangle[i][j] = triangle[i-1][j-1] + triangle[i-1][j];
        }
    }

    triangle[n][m]
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

    // Solve
    let result = solve(TestCase { n, m });
    
    let _ = writeln!(stdout, "{result}");
}