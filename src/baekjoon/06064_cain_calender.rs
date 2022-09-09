#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    m: i64,
    n: i64,
    x: i64,
    y: i64,
}

fn ex_gcd(a: i64, b: i64) -> (i64, (i64, i64)) {
    let mut old_s = 1;
    let mut old_t = 0;
    let mut old_r = a;
    
    let mut s = 0;
    let mut t = 1;
    let mut r = b;

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - r * q);
        (old_s, s) = (s, old_s - s * q);
        (old_t, t) = (t, old_t - t * q);
    }

    (old_r, (old_s, old_t))
}

fn solve(tc: TestCase) -> Option<i64> {
    if tc.x < 1 || tc.x > tc.m || tc.y < 1 || tc.y > tc.n {
        return None;
    }

    let m = if tc.m >= tc.n { tc.m } else { tc.n };
    let n = if tc.m >= tc.n { tc.n } else { tc.m };

    let a = if tc.m >= tc.n { tc.x - 1 } else { tc.y - 1 };
    let b = if tc.m >= tc.n { tc.y - 1 } else { tc.x - 1 };

    // at this point, let's assume the solution is r, then
    // r = a (mod m), r = b (mod n), m >= n

    // validity check
    let (gcd_mn, (s, t)) = ex_gcd(m, n);
    if a % gcd_mn != b % gcd_mn {
        return None;
    }

    // r = -mu + a = nv + b => mu + nv = a - b, 
    // the solution (u, v) only exists when gcd(m, n) | a - b (=> Bezout's identity)
    // let a - b = k * gcd(m, n), k = (a - b) / gcd(m, n)
    // we can get (u', v') for mu' + nv' = gcd(m, n) by using extended euclidean algorithm (Bezout's coefficient s, t)
    // then multiply each side by k to get this: mu'k + nv'k = a - b = mu + nv => u = u'k, v = v'k
    // we know all variables at this point, so calculate r = -mu + a (or r = nv + b) (mod lcm(m, n))

    let k = (a - b) / gcd_mn;
    let lcm_mn = (m * n) / gcd_mn;
    let mut r = -m;
    r = (r * s) % lcm_mn;
    r = (r * k) % lcm_mn;
    r = (r + a) % lcm_mn;

    if r < 0 { r += lcm_mn; }
    Some(r)
}

// previous counterexample: `10 4 1 3`, expected 11, output 1 

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut buffer = String::new();

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    // Parsing
    let _ = stdin.read_to_string(&mut buffer);
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let t = inputs.next().unwrap();

    // Solve
    for _ in 0..t {
        let m = inputs.next().unwrap();
        let n = inputs.next().unwrap();
        let x = inputs.next().unwrap();
        let y = inputs.next().unwrap();

        let result = solve(TestCase { m, n, x, y });
    
        match result {
            Some(n) => { let _ = writeln!(stdout, "{}", n + 1); }
            None => { let _ = writeln!(stdout, "-1"); }
        }
    }
}