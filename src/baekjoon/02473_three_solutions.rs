#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    solutions: Vec<i64>,
}

fn solve(tc: TestCase) -> (i64, i64, i64) {
    let TestCase { n, solutions } = tc;
    let mut solutions = solutions;
    solutions.sort_unstable();

    let mut result = (0, 0, 0);
    let mut min_sum_abs = i64::MAX;

    'outer: for i in 0..n-2 {
        let a = solutions[i];
        
        let mut start = i+1;
        let mut end = n-1;

        while start < end {
            let b = solutions[start];
            let c = solutions[end];

            let sum = a + b + c;

            if sum.abs() < min_sum_abs {
                result = (a, b, c);
                min_sum_abs = sum.abs();
            }

            if sum == 0 {
                break 'outer;
            } else if sum > 0 {
                end -= 1;
            } else {
                start += 1;
            }
        }
    }

    result
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let n = inputs.next().unwrap() as usize;
    let solutions = inputs.collect();

    // Solve
    let result = solve(TestCase { n, solutions });
    
    let _ = writeln!(stdout, "{} {} {}", result.0, result.1, result.2);
}