#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    a: Vec<char>,
    b: Vec<char>,
}

fn lcs(memo: &mut Vec<Vec<usize>>, a: &[char], b: &[char]) -> (usize, String) {
    let m = a.len();
    let n = b.len();

    for i in 1..=m {
        for j in 1..=n {
            if a[i-1] == b[j-1] {
                memo[i][j] = memo[i-1][j-1] + 1
            } else {
                memo[i][j] = memo[i][j-1].max(memo[i-1][j]);
            }
        }
    }

    let lcs_len = memo[m][n];
    let mut lcs = Vec::with_capacity(lcs_len);

    let mut i = m;
    let mut j = n;
    while i > 0 && j > 0 {
        if a[i-1] == b[j-1] {
            lcs.push(a[i-1]);
            i -= 1;
            j -= 1;
        } else if memo[i][j-1] >= memo[i-1][j] {
            j -= 1;
        } else {
            i -= 1;
        }
    }

    (lcs_len, lcs.into_iter().rev().collect::<String>())
}

fn solve(tc: TestCase) -> (usize, String) {
    let TestCase { a, b } = tc;
    let m = a.len();
    let n = b.len();

    let mut memo = vec![vec![0; n+1]; m+1];
    lcs(&mut memo, &a, &b)
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
    let mut inputs = buffer.split_whitespace();

    let a = inputs.next().unwrap().chars().collect::<Vec<char>>();
    let b = inputs.next().unwrap().chars().collect::<Vec<char>>();

    // Solve
    let (lcs_len, lcs) = solve(TestCase { a, b });
    
    let _ = writeln!(stdout, "{}", lcs_len);
    let _ = writeln!(stdout, "{}", lcs);
}