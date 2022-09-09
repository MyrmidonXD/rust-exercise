#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    v: Vec<usize>,
    q: Vec<(usize, usize)>
}

fn solve(tc: TestCase, w: &mut impl Write) {
    let mut acc_v = vec![0];

    let mut acc = 0;
    for x in tc.v {
        acc += x;
        acc_v.push(acc);
    }

    for (i, j) in tc.q {
        let _ = writeln!(w, "{}", acc_v[j] - acc_v[i - 1]);
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

    let n = inputs.next().unwrap();
    let m = inputs.next().unwrap();

    let mut v = vec![];
    for _ in 0..n {
        v.push(inputs.next().unwrap());
    }

    let mut q = vec![];
    for _ in 0..m {
        let i = inputs.next().unwrap();
        let j = inputs.next().unwrap();
        q.push((i, j));
    }

    // Solve
    solve(TestCase { n, m, v, q }, &mut stdout);
}