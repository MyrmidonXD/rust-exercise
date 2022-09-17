#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    stickers: Vec<[usize; 2]>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, stickers } = tc;
    let mut memo = vec![[0; 2]; n];

    memo[0][0] = stickers[0][0];
    memo[0][1] = stickers[0][1];

    if n > 1 {
        memo[1][0] = memo[0][1] + stickers[1][0];
        memo[1][1] = memo[0][0] + stickers[1][1];
    }

    for i in 2..n {
        memo[i][0] = (memo[i-1][1] + stickers[i][0]).max(memo[i-2][1] + stickers[i][0]);
        memo[i][1] = (memo[i-1][0] + stickers[i][1]).max(memo[i-2][0] + stickers[i][1]);
    }

    memo[n-1][0].max(memo[n-1][1])
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

    for _ in 0..t {
        let n = inputs.next().unwrap();

        let mut stickers = vec![[0; 2]; n];

        for i in 0..n {
            stickers[i][0] = inputs.next().unwrap();
        }
        for i in 0..n {
            stickers[i][1] = inputs.next().unwrap();
        }

        // Solve
        let result = solve(TestCase { n, stickers });
        
        let _ = writeln!(stdout, "{result}");
    }
}