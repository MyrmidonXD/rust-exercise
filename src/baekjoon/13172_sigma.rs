#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    dices: Vec<(usize, usize)>,
}

const MOD: usize = 1_000_000_007_usize;

fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = if a >= b { (a, b) } else { (b, a) };

    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn pow_mod(x: usize, n: usize) -> usize {
    if n == 0 {
        1
    } else if n == 1 {
        x
    } else {
        let mut result = pow_mod(x, n/2);
        result = (result * result) % MOD;

        if n % 2 == 1 {
            result = (result * x) % MOD;
        }

        result
    }
}

fn get_modular_fraction(numerator: usize, denominator: usize) -> usize {
    let gcd = gcd(numerator, denominator);
    let (numerator, denominator) = (numerator/gcd, denominator/gcd);

    let denom_inverse = pow_mod(denominator, MOD-2);

    (numerator * denom_inverse) % MOD
}

fn solve(tc: TestCase) -> usize {
    let dices = tc.dices;
    dices.into_iter().fold(0, |acc, (n_i, s_i)| (acc + get_modular_fraction(s_i, n_i)) % MOD)
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
    let mut dices = vec![];
    for _ in 0..n {
        let n_i = inputs.next().unwrap();
        let s_i = inputs.next().unwrap();

        dices.push((n_i, s_i));
    }

    // Solve
    let result = solve(TestCase { n, dices });
    
    let _ = writeln!(stdout, "{result}");
}