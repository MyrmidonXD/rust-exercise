#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
enum Formula {
    Num(i32),
    Plus,
    Minus,
}

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    s: String
}

fn solve(tc: TestCase) -> i32 {
    let chars = tc.s.chars();

    let mut formula = vec![];    
    let mut n = 0;
    for c in chars {
        match c {
            '+' => { formula.push(Formula::Num(n)); n = 0; formula.push(Formula::Plus) },
            '-' => { formula.push(Formula::Num(n)); n = 0; formula.push(Formula::Minus) },
            c   => { n = n * 10 + (c as i32 - '0' as i32); }
        }
    }
    formula.push(Formula::Num(n));

    let mut sum = 0;
    let mut minus = false;
    for f in formula {
        match f {
            Formula::Num(n) => if minus { sum -= n } else { sum += n },
            Formula::Minus => { minus = true; },
            _ => ()
        }
    }

    sum
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

    let s = buffer.trim_end().to_string();

    // Solve
    let result = solve(TestCase { s });
    
    let _ = writeln!(stdout, "{result}");
}