#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    board: Vec<[u32; 3]>,
}

fn solve(tc: TestCase) -> (u32, u32) {
    let TestCase { n, board } = tc;
    let mut min_score = vec![[0, 0, 0]; n];
    let mut max_score = vec![[0, 0, 0]; n];

    min_score[0] = board[0];
    max_score[0] = board[0];

    for i in 1..n {
        min_score[i][0] = board[i][0] + (min_score[i-1][0].min(min_score[i-1][1]));
        min_score[i][1] = board[i][1] + (min_score[i-1][0].min(min_score[i-1][1]).min(min_score[i-1][2]));
        min_score[i][2] = board[i][2] + (min_score[i-1][1].min(min_score[i-1][2]));

        max_score[i][0] = board[i][0] + (max_score[i-1][0].max(max_score[i-1][1]));
        max_score[i][1] = board[i][1] + (max_score[i-1][0].max(max_score[i-1][1]).max(max_score[i-1][2]));
        max_score[i][2] = board[i][2] + (max_score[i-1][1].max(max_score[i-1][2]));
    }

    (IntoIterator::into_iter(max_score[n-1]).max().unwrap(), IntoIterator::into_iter(min_score[n-1]).min().unwrap())
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

    let n = inputs.next().unwrap() as usize;
    let mut board = vec![];
    for _ in 0..n {
        let a = inputs.next().unwrap();
        let b = inputs.next().unwrap();
        let c = inputs.next().unwrap();

        board.push([a, b, c]);
    }
    
    // Solve
    let result = solve(TestCase { n, board });
    
    let _ = writeln!(stdout, "{} {}", result.0, result.1);
}