#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    triangle: Vec<Vec<usize>>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, triangle } = tc;

    let mut memo = vec![vec![]; n];
    for i in 0..n {
        for _ in 0..=i {
            memo[i].push(0);
        }
    }

    memo[0][0] = triangle[0][0];
    for i in 1..n {
        for j in 0..=i {
            let mut max = 0;
            if j > 0 {
                max = memo[i-1][j-1] + triangle[i][j];
            }
            if j < i {
                max = max.max(memo[i-1][j] + triangle[i][j])
            }

            memo[i][j] = max;
        }
    }

    *memo[n-1].iter().max().unwrap()
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

    let mut triangle = vec![vec![]; n];
    for i in 0..n {
        for j in 0..=i {
            triangle[i].push(inputs.next().unwrap());
        }
    }

    // Solve
    let result = solve(TestCase { n, triangle });
    
    let _ = writeln!(stdout, "{result}");
}