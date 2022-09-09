#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
}

fn solve(tc: TestCase) -> u32 {
    let mut num_2 = 0;
    let mut num_5 = 0;

    for i in 1..=tc.n {
        let mut x = i;

        while x % 2 == 0 {
            num_2 += 1;
            x = x / 2;
        }

        while x % 5 == 0 {
            num_5 += 1;
            x = x / 5;
        }
    }

    num_2.min(num_5)
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

    // Solve
    let result = solve(TestCase { n });
    
    let _ = writeln!(stdout, "{result}");
}