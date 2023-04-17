#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize
}

/*
    S(k, 0) =             S(k-1, 1)
    S(k, 1) = S(k-1, 0) + S(k-1, 2)
    S(k, 2) = S(k-1, 1) + S(k-1, 3)
    ...
    S(k, 7) = S(k-1, 6) + S(k-1, 8)
    S(k, 8) = S(k-1, 7) + S(k-1, 9)
    S(k, 9) = S(k-1, 8)
 */
fn solve(tc: TestCase) -> usize {
    const MOD: u32 = 1_000_000_000;
    let TestCase { n } = tc;

    let mut stair: Vec<[[u32; 1024]; 10]> = vec![[[0; 1024]; 10]; n];
    for i in 1..=9 {
        stair[0][i as usize][(1 << i) as usize] = 1;
    }

    for i in 1..n {
        for j in 0..=9 {
            if j > 0 {
                for k in 0..1024 {
                    stair[i][j][k | (1 << j)] = (stair[i][j][k | (1 << j)] + stair[i-1][j-1][k]) % MOD;
                }
            }
            if j < 9 {
                for k in 0..1024 {
                    stair[i][j][k | (1 << j)] = (stair[i][j][k | (1 << j)] + stair[i-1][j+1][k]) % MOD;
                }
            }
        }
    }

    let mut result = 0;
    for j in 0..=9 {
        result += stair[n-1][j][1023];
        result = result % MOD;
    }

    result as usize
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