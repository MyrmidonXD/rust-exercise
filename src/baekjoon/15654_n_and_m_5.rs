#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::fmt;

struct VecFmt<'a, T>(&'a Vec<T>);
impl<'a, T: fmt::Display> fmt::Display for VecFmt<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() == 0 { return write!(f, "") }

        for (i, e) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", *e)?;
            } else {
                write!(f, " {}", *e)?;
            }
        }

        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    xs: Vec<usize>,
}

fn backtrack(w: &mut impl Write, result: &mut Vec<usize>, xs: &Vec<usize>, used: &mut Vec<bool>, i: usize) {
    if i == result.len() {
        let _ = writeln!(w, "{}", VecFmt(result));
        return;
    }

    for (j, &x) in xs.iter().enumerate() {
        if used[j] { continue; }

        result[i] = x;
        used[j] = true;
        backtrack(w, result, xs, used, i+1);
        used[j] = false;
    }
}

fn solve(tc: TestCase, w: &mut impl Write) {
    let TestCase { n, m, xs } = tc;
    let mut xs = xs;

    xs.sort_unstable();
    let mut result = vec![0; m];
    let mut used = vec![false; n];

    backtrack(w, &mut result, &xs, &mut used, 0);
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
    let m = inputs.next().unwrap();
    let xs = inputs.collect();

    solve(TestCase { n, m, xs }, &mut stdout);
}