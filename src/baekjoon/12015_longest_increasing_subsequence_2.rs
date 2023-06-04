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

    let mut lis_len = vec![1usize; n];
    let mut longest = 1usize;
    let mut seq_tails = vec![0usize; n+1];

    seq_tails[1] = seq[0];
    for i in 1..n {
        if seq[i] < seq_tails[1] {
            seq_tails[1] = seq[i];
            lis_len[i] = 1;
        } else if seq[i] > seq_tails[longest] {
            seq_tails[longest + 1] = seq[i];
            lis_len[i] = longest + 1;

            longest += 1;
        } else {
            let k = binary_search(&seq_tails, longest, seq[i]);

            seq_tails[k] = seq[i];
            lis_len[i] = k;
        }
    }

    longest
}

fn binary_search<T: Ord>(arr: &[T], len: usize, item: T) -> usize {
    let (mut low, mut high) = (1, len);

    while low < high {
        let mid = (low + high) / 2;

        if item <= arr[mid] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    high
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