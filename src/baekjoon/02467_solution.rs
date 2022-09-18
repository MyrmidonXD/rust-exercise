#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    solutions: Vec<i32>,
}

fn solve(tc: TestCase) -> (i32, i32) {
    let TestCase { n, solutions } = tc;

    let mut abs_min = i32::MAX;
    let mut result = (0, 0);

    'outer: for i in 0..n-1 {
        let mut low = i + 1;
        let mut high = n - 1;
        let mut curr = (low + high) / 2;

        while low + 1 < high {
            let sum = solutions[i] + solutions[curr];

            if sum == 0 {
                result = (solutions[i], solutions[curr]);
                break 'outer;
            } else if sum > 0 {
                high = curr;
                curr = (low + high) / 2;
            } else {
                low = curr;
                curr = (low + high) / 2;
            }
        }

        let sum_low = solutions[i] + solutions[low];
        let sum_high = solutions[i] + solutions[high];

        if sum_low.abs() < abs_min {
            abs_min = sum_low.abs();
            result = (solutions[i], solutions[low]);
        }
        if sum_high.abs() < abs_min {
            abs_min = sum_high.abs();
            result = (solutions[i], solutions[high]);
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = inputs.next().unwrap() as usize;
    let solutions = inputs.collect();

    // Solve
    let result = solve(TestCase { n, solutions });
    
    let _ = writeln!(stdout, "{} {}", result.0, result.1);
}