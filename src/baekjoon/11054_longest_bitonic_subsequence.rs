#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    seq: Vec<usize>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, seq } = tc;
    let mut lis = vec![1; n];
    let mut lds = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            if seq[j] < seq[i] {
                lis[i] = lis[i].max(lis[j] + 1);
            } else if seq[j] > seq[i] {
                lds[i] = lds[i].max(lds[j] + 1).max(lis[j] + 1);
            }
        }
    }

    (lis.into_iter().max().unwrap()).max(lds.into_iter().max().unwrap())
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
    let seq = inputs.collect();

    // Solve
    let result = solve(TestCase { n, seq });
    
    let _ = writeln!(stdout, "{result}");
}