#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    a: usize,
    b: usize,
}

fn dfs(a: usize, b: usize) -> Option<usize> {
    let twice = a * 2;
    let add_one = a * 10 + 1;

    if twice == b || add_one == b {
        return Some(1);
    }

    let twice = if twice > b { None } else { dfs(twice, b) };
    let add_one = if add_one > b { None } else { dfs(add_one, b) };

    match (twice, add_one) {
        (Some(t), Some(a)) => Some(1 + t.min(a)),
        (Some(t), None) => Some(1 + t),
        (None, Some(a)) => Some(1 + a),
        (None, None) => None
    }
}

fn solve(tc: TestCase) -> i32 {
    match dfs(tc.a, tc.b) {
        Some(x) => (x + 1) as i32,
        None => -1
    }
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

    let a = inputs.next().unwrap();
    let b = inputs.next().unwrap();
    
    // Solve
    let result = solve(TestCase { a, b });
    
    let _ = writeln!(stdout, "{result}");
}