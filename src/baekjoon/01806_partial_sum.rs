#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    s: usize,
    seq: Vec<usize>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, s, seq } = tc;

    let mut accumuated_sum: Vec<usize> = seq.iter().scan(0, |state, &x| {
        let prev = *state;
        *state += x;

        Some(prev)
    }).collect();
    accumuated_sum.push(accumuated_sum[n-1] + seq[n-1]);

    let mut front = 0;
    let mut back = 1;
    let mut min_length = usize::MAX;

    while back <= n {
        let partial_sum = accumuated_sum[back] - accumuated_sum[front];
        
        // update min length
        if partial_sum >= s {
            min_length = min_length.min(back - front);
        }

        // move front or back
        if partial_sum >= s && back - front > 1 {
            front += 1;
        } else {
            back += 1;
        }
    }

    if min_length == usize::MAX { min_length = 0; }
    min_length
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
    let s = inputs.next().unwrap();
    let seq = inputs.collect();

    // Solve
    let result = solve(TestCase { n, s, seq });
    
    let _ = writeln!(stdout, "{result}");
}