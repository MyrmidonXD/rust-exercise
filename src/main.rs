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

fn solve(tc: &TestCase) -> Option<i64> {
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
    let (gcd_mn, _) = ex_gcd(m, n);
    if a % gcd_mn != b % gcd_mn {
        return None;
    }

    // make m, n as coprime, by dividing n with gcd(m, n)
    let n = n / gcd_mn;
    let b = b % n;

    // r = m*j + a = b (mod n) => j = m^-1 * (b - a) (mod n)
    // m^-1 for mod n can be calculated by this Bezout's identity: mx + ny = 1 (<-> mx = 1 (mod n))
    let (_, (m_inv, _)) = ex_gcd(m, n);
    
    // j = n * k + (m^-1 * (b - a))
    // r = m * j + a
    //   = m * (n * k + (m^-1 * (b - a))) + a
    //   = (m * n) * k + ( m * m^-1 * (b - a) + a )
    //   = m * m^-1 * (b - a) + a (mod m * n)

    // let mut r = (m * m_inv * (b - a) + a) % (m * n);

    let mut j = (m_inv * (b - a)) % n;
    if j < 0 { j += n };

    let mut r = (m * j) % (m * n);
    if r < 0 { r += m * n };

    r = (r + a) % (m * n);
    if r < 0 { r += m * n };

    Some(r)
}

fn solve_naive(tc: &TestCase) -> Option<i64> {
    let a = tc.x - 1;
    let b = tc.y - 1;

    let m = tc.m;
    let n = tc.n;

    let (gcd, _) = ex_gcd(m, n);
    let lcm = (m * n) / gcd;

    let mut r = a;
    while r < lcm {
        if r % n == b {
            return Some(r);
        }

        r += m;
    }

    None
}

fn main() {
    //let m = rand::random::<u32>() % 10 + 1;
    //let n = rand::random::<u32>() % 10 + 1;

    let m = 10;
    let n = 8;

    println!("Testing for m: {m}, n: {n}");


    let mut x = 1;
    let mut y = 1;

    let chunk= if m * n > 100 { (m * n) / 100 } else { 1 };

    for i in 0..(m*n) {
        let tc = TestCase { m: m as i64, n: n as i64, x: x as i64, y: y as i64 };
        assert_eq!(solve(&tc), solve_naive(&tc), "Counterexample found: `{} {} {} {}`", m, n, x, y);

        y += 1;
        if y > n {
            y -= n;
            x += 1;
        }

        if (i + 1) % chunk == 0 {
            println!("{} / {} ({}%)", i + 1, m * n, if chunk == 1 { (100 * i) / (m * n) } else { (i + 1) % chunk });
        }
    }

    println!("No counterexample found for m: {}, n: {}", m, n);
}
