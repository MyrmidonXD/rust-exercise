#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    a: Vec<char>,
    b: Vec<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PrevState {
    Diagonal,
    Left,
    Up,
    None
}

fn lcs(memo: &mut Vec<Vec<(usize, PrevState)>>, a: &[char], b: &[char]) -> (usize, String) {
    let m = a.len();
    let n = b.len();

    for i in 1..=m {
        for j in 1..=n {
            if a[i-1] == b[j-1] {
                memo[i][j] = (memo[i-1][j-1].0 + 1, PrevState::Diagonal)
            } else {
                if memo[i][j-1].0 >= memo[i-1][j].0 {
                    memo[i][j] = (memo[i][j-1].0, PrevState::Left);
                } else {
                    memo[i][j] = (memo[i-1][j].0, PrevState::Up);
                }
            }
        }
    }

    let lcs_len = memo[m][n].0;
    let mut lcs = Vec::new();

    let mut i = m;
    let mut j = n;
    while i > 0 && j > 0 {
        match memo[i][j].1 {
            PrevState::Diagonal => {
                lcs.push(a[i-1]);
                i -= 1;
                j -= 1;
            },
            PrevState::Left => {
                j -= 1;
            },
            PrevState::Up => {
                i -= 1;
            }
            _ => { break; }
        }
    }

    (lcs_len, lcs.into_iter().rev().collect::<String>())
}

fn solve(tc: TestCase) -> (usize, String) {
    let TestCase { a, b } = tc;
    let m = a.len();
    let n = b.len();

    let mut memo = vec![vec![(0, PrevState::None); n+1]; m+1];
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