#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: u32,
}

fn solve(tc: TestCase) -> u32 {
    let n = tc.n as usize;
    let mut memo = vec![4; n + 1];

    memo[0] = 0;
    memo[1] = 1;
    for i in 1..=n {
        for j in 1..i {
            let j_sqr = j * j;
            if j_sqr > i { break; }

            memo[i] = memo[i].min(1 + memo[i - j_sqr]);
        }
    }

    memo[n]
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

    // Solve
    let result = solve(TestCase { n });
    
    let _ = writeln!(stdout, "{result}");
}