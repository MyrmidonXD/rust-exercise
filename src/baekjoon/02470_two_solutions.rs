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
    let mut solutions = solutions;

    solutions.sort_unstable();

    let mut abs_min = i32::MAX;
    let mut result = (0, 0);

    let mut low = 0;
    let mut high = n-1;

    while low < high {
        let sum = solutions[low] + solutions[high];

        if sum.abs() < abs_min {
            abs_min = sum.abs();
            result = (solutions[low], solutions[high]);
        }

        if sum == 0 { break; }

        if sum > 0 {
            high = high - 1;
        } else {
            low = low + 1;
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