use std::io::{self, Read, BufWriter, Write};
use std::collections::HashSet;

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);
    let mut lines = buffer.lines();

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let nm: Vec<usize> = lines.next().unwrap()
                                .split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (n, m) = (nm[0], nm[1]);

    let mut not_heard = HashSet::new();
    let mut not_seen_and_heard = Vec::new();

    for _ in 0..n {
        let name = lines.next().unwrap();
        not_heard.insert(name);
    }

    for _ in 0..m {
        let name = lines.next().unwrap();
        if not_heard.contains(&name) {
            not_seen_and_heard.push(name);
        }
    }

    not_seen_and_heard.sort_unstable();
    let _ = writeln!(stdout, "{}", not_seen_and_heard.len());

    for name in not_seen_and_heard {
        let _ = writeln!(stdout, "{}", name);
    }
}