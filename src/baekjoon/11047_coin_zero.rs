#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    k: usize,
    coins: Vec<usize>,
}

fn solve(tc: TestCase) -> usize {
    let coins = tc.coins;
    let n = tc.n;
    let mut k = tc.k;

    let mut count = 0;
    for i in (0..n).rev() {
        let coin = coins[i];

        let coin_count = k / coin;
        count += coin_count;
        k -= coin * coin_count;

        if k == 0 { break; }
    }

    count
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
    let k = inputs.next().unwrap();
    let coins = inputs.collect();

    // Solve
    let result = solve(TestCase { n, k, coins });
    
    let _ = writeln!(stdout, "{result}");
}