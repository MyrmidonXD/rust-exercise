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
    let mut result_sum_abs = i64::MAX;

    'outer: for i in 0..n-2 {
        for j in i+1..n-1 {
            let sum_tmp = solutions[i] + solutions[j];

            let mut low = j+1;
            let mut high = n-1;

            while low <= high {
                let curr = (low + high) / 2;
                let sum = sum_tmp + solutions[curr];

                if sum.abs() < result_sum_abs {
                    result = (solutions[i], solutions[j], solutions[curr]);
                    result_sum_abs = sum.abs();
                }

                if sum == 0 {
                    result = (solutions[i], solutions[j], solutions[curr]);
                    break 'outer;
                } else if sum > 0 {
                    high = curr - 1;
                } else {
                    low = curr + 1;
                }
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