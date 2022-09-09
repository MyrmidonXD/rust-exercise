#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut buffer = String::new();

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    // Parsing
    let _ = stdin.read_to_string(&mut buffer);
    let mut inputs = buffer.split_whitespace();

    let n: usize = inputs.next().unwrap().parse().unwrap();
    let m: usize = inputs.next().unwrap().parse().unwrap();

    let mut map = HashMap::new();
    for _ in 0..n {
        let addr = inputs.next().unwrap();
        let password = inputs.next().unwrap();

        map.insert(addr, password);
    }

    for _ in 0..m {
        let addr = inputs.next().unwrap();
        let password = map[addr];

        let _ = writeln!(stdout, "{}", password);
    }
}