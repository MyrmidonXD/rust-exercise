#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: u32,
    stairs: Vec<u32>,
}

fn solve(tc: TestCase) -> u32 {
    let mut step = vec![];
    let mut jump = vec![];

    let n = tc.n as usize;

    for i in 0..n {
        if i == 0 {
            step.push(tc.stairs[0]);
            jump.push(0);
        } else if i == 1 {
            step.push(tc.stairs[0] + tc.stairs[1]);
            jump.push(tc.stairs[1]);
        } else {
            step.push(jump[i-1] + tc.stairs[i]);
            jump.push(jump[i-2].max(step[i-2]) + tc.stairs[i]);
        }
    }

    jump[n-1].max(step[n-1])
}

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut buffer = String::new();

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let _ = stdin.read_to_string(&mut buffer);
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<u32>().unwrap());

    let n = inputs.next().unwrap();
    let stairs = inputs.collect();

    let result = solve(TestCase { n, stairs });
    
    let _ = writeln!(stdout, "{result}");
}