use std::io::{self, Read, BufWriter, Write};

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);
    let input = buffer.lines().skip(1).map(|s| s.parse::<usize>().unwrap());

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let answer = [0, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274];

    for n in input {
        let answer = answer[n];
        let _ = writeln!(stdout, "{answer}");
    }

    let _ = stdout.flush();
}