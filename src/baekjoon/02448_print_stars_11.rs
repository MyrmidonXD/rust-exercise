#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
}

fn get_stars(n: usize) -> Vec<String> {
    if n == 3 {
        return vec![
            "  *  ".to_string(),
            " * * ".to_string(),
            "*****".to_string(),
        ];
    }

    let part = get_stars(n/2);
    let mut result = vec![];

    let margin = (part[0].chars().count() + 1) / 2;
    for s in &part {
        result.push(format!("{1}{0}{1}", s, (0..margin).map(|_| ' ').collect::<String>()));
    }
    for s in &part {
        result.push(format!("{0} {0}", s));
    }

    result
}

fn solve(tc: TestCase) -> Vec<String> {
    let n = tc.n;
    get_stars(n)
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
    for s in result {
        let _ = writeln!(stdout, "{s}");
    }
}