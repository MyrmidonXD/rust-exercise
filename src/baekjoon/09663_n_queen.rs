#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
}

fn n_queen(queens: &mut Vec<Option<usize>>, i: usize) -> usize {
    let n = queens.len();

    let mut possiblities = vec![];

    'outer: for y in 0..n {
        // check previous queens
        for x in 0..i {
            let y_x = queens[x].unwrap();

            let same_row = y == y_x;
            let upper_diag = y_x >= (i - x) && y_x - (i - x) == y;
            let lower_diag = y_x + (i - x) == y; // if this is true, then this implies y_x + (i - x) < n because y < n

            if same_row || upper_diag || lower_diag {
                continue 'outer;
            }
        }

        possiblities.push(y);
    }

    if i == n-1 {
        return possiblities.len();
    } else {
        let mut sum = 0;
        for y in possiblities {
            queens[i] = Some(y);
            sum += n_queen(queens, i+1);
        }

        queens[i] = None;
        return sum;
    }
}

fn solve(tc: TestCase) -> usize {
    let n = tc.n;
    let mut queens = vec![None; n];

    n_queen(&mut queens, 0)
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