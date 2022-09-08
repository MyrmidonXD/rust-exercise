use std::io::{self, Read, BufWriter, Write};
use std::collections::HashMap;

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

    let mut names = Vec::new();
    let mut name_to_num = HashMap::new();

    for idx in 0..n {
        let name = lines.next().unwrap();
        names.push(name);
        name_to_num.insert(name, idx+1);
    }

    for _ in 0..m {
        let query = lines.next().unwrap();
        if let Ok(n) = query.parse::<usize>() {
            let _ = writeln!(stdout, "{}", names[n-1]);
        } else {
            let _ = writeln!(stdout, "{}", name_to_num[query]);
        }
    }
}