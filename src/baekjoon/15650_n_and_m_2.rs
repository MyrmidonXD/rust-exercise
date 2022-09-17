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
}

fn solve(tc: TestCase, w: &mut impl Write) {
    let TestCase { n, m } = tc;

    let mut seq = vec![0; m];
    let mut bound = vec![0; m];

    // init
    for i in 0..m {
        seq[i] = i + 1;
        bound[i] = n - ((m - 1) - i);
    }

    let mut i = m-1;
    'outer: loop {
        let _ = writeln!(w, "{}", VecFmt(&seq));

        // decrease pointer
        while seq[i] == bound[i] {
            if i == 0 { break 'outer; }
            i -= 1;
        }

        seq[i] += 1;

        // increase pointer, with setting seq and bound
        for j in i+1..m {
            let low = seq[i] + 1;
            let high = n - ((m - 1) - j);

            seq[j] = low;
            bound[j] = high;

            i = j;
        }
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

    solve(TestCase { n, m }, &mut stdout);
}