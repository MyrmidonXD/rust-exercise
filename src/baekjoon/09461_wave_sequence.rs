#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
}

fn solve(tc: TestCase, seq: &mut Vec<usize>) -> usize {
    let n = tc.n - 1;
    if n < seq.len() {
        return seq[n];
    }

    for i in seq.len()..=n {
        seq.push(seq[i - 1] + seq[i - 5]);
    }

    seq[n]
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

    let t = inputs.next().unwrap();
    let mut seq = vec![1, 1, 1, 2, 2, 3, 4, 5, 7, 9];

    for _ in 0..t {
        let n = inputs.next().unwrap();

        let result = solve(TestCase { n }, &mut seq);
        
        let _ = writeln!(stdout, "{result}");
    }
}