#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    a: usize,
    b: usize,
    c: usize,
}

fn mult(a: usize, b: usize, c: usize) -> usize {
    if b == 0 {
        return 1 % c;
    } else if b == 1 {
        return a % c;
    } else if b % 2 == 0 {
        let root = mult(a, b/2, c);
        return (root * root) % c;
    } else {
        let root = mult(a, b/2, c);
        return (((root * root) % c) * a) % c;
    }
}

fn solve(tc: TestCase) -> usize {
    let TestCase { a, b, c } = tc;
    mult(a, b, c)
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
    let c = inputs.next().unwrap();

    // Solve
    let result = solve(TestCase { a, b, c });
    
    let _ = writeln!(stdout, "{result}");
}