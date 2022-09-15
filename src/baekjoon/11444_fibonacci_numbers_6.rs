#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: u64,
}

type Matrix = [[u64; 2]; 2];
const MOD: u64 = 1_000_000_007;

#[inline]
fn mat_mul(a: &Matrix, b: &Matrix) -> Matrix {
    [
        [ 
            (a[0][0] * b[0][0] + a[0][1] * b[1][0]) % MOD,
            (a[0][0] * b[0][1] + a[0][1] * b[1][1]) % MOD
        ],
        [
            (a[1][0] * b[0][0] + a[1][1] * b[1][0]) % MOD,
            (a[1][0] * b[0][1] + a[1][1] * b[1][1]) % MOD
        ]
    ]
}

fn fib_mat(n: u64) -> Matrix {
    if n == 1 {
        return [[1, 1], [1, 0]];
    }

    let root = fib_mat(n/2);
    let mut result = mat_mul(&root, &root);

    if n % 2 != 0 {
        result = mat_mul(&result, &[[1, 1], [1, 0]]);
    }

    result
}

fn solve(tc: TestCase) -> u64 {
    let result = fib_mat(tc.n);
    result[0][1]
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<u64>().unwrap());

    let n = inputs.next().unwrap();

    // Solve
    let result = solve(TestCase { n });
    
    let _ = writeln!(stdout, "{result}");
}