#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    m: u32,
    n: u32,
    x: u32,
    y: u32,
}

fn gen_tc(t: u32) -> Vec<TestCase> {
    let m = 39983;
    let n = 39989;
    let mut x = 39949;
    let mut y = 39961;

    let mut v = vec![];
    for _ in 0..t {
        x += 1;
        y += 1;

        if x > m { x -= m }
        if y > n { y -= n }

        v.push(TestCase { m, n, x, y });
    }

    v
}

#[allow(unused_variables)]
fn main() {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let t = 100;
    let tcs = gen_tc(t);

    let _ = writeln!(stdout, "{t}");
    for tc in tcs {
        let _ = writeln!(stdout, "{} {} {} {}", tc.m, tc.n, tc.x, tc.y);
    }
}