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

fn solve(tc: TestCase, w: &mut impl Write) {
    let TestCase { n, m, xs } = tc;
    let mut xs = xs;
    xs.sort_unstable();

    let mut result_indices = vec![0; m];
    let mut result = vec![xs[0]; m];
    let mut idx = m - 1;

    let _ = writeln!(w, "{}", VecFmt(&result));

    'outer: loop {
        // move idx to front
        while result_indices[idx] == n - 1 {
            if idx == 0 { break 'outer; }
            idx -= 1;
        }

        result_indices[idx] += 1;

        // move idx to back
        for i in idx+1..m {
            result_indices[i] = result_indices[idx];
        }
        idx = m - 1;

        for (i, &j) in result_indices.iter().enumerate() {
            result[i] = xs[j];
        } 
        
        let _ = writeln!(w, "{}", VecFmt(&result));
    }
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

    // Solve
    let result = solve(TestCase { n, m, xs }, &mut stdout);
}