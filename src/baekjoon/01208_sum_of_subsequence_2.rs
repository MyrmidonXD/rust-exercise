#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    s: i32,
    seq: Vec<i32>,
}

fn sum_cases(seq: &[i32], sum: i32, start: usize) -> usize {
    let end = seq.len() - 1;
    if start >= end || (sum < 0 && seq[start] >= 0) || (sum > 0 && seq[end] <= 0) || (sum == 0 && (seq[start] > 0 || seq[end] < 0)) {
        return 0;
    }

    let mut result = 0;

    // Two-pointer algorithm
    let (mut i, mut j) = (start, end);
    while i < j {
        let s = seq[i] + seq[j];
        if s == sum {
            let (mut a, mut b) = (0, 0);
            while i < j {
                if seq[i] == seq[i+1] {
                    a += 1;
                    i += 1;
                } else if seq[j] == seq[j-1] {
                    b += 1;
                    j -= 1;
                } else {
                    break;
                }
            }

            if i == j {
                let tmp = a + b;
                result += (tmp * (tmp + 1)) / 2;
            } else {
                result += (a + 1) * (b + 1);
                i += 1;
                j -= 1;
            }
        } else if s < sum {
            i += 1;
        } else {
            j -= 1;
        }
    }

    // recursive call
    for i in start..(end-1) {
        let next_sum = sum - seq[i];
        result += sum_cases(seq, next_sum, i+1);
    }

    result
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, s, mut seq } = tc;
    seq.sort_unstable();

    // check subsequences of size 1
    let mut result = 0;
    for &x in &seq {
        if x == s {
            result += 1;
        }
    }

    result += sum_cases(&seq, s, 0);

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
    let s = inputs.next().unwrap();

    let seq = inputs.collect();

    // Solve
    let result = solve(TestCase { n, s, seq });
    
    let _ = writeln!(stdout, "{result}");
}