#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: u32,
    stairs: Vec<u32>,
}

fn solve(tc: TestCase) -> u32 {
    // TODO: Implement this
    println!("{:?}", tc);
    0
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<u32>().unwrap());

    let n = inputs.next().unwrap();
    let stairs = inputs.collect();

    // Solve
    let result = solve(TestCase { n, stairs });
    
    let _ = writeln!(stdout, "{result}");
}