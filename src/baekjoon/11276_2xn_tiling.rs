use std::io::{self, Read, BufWriter, Write};

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);
    let n: usize = buffer.trim_end().parse().unwrap();

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut answer = vec![0; n];
    answer[0] = 1;
    if n >= 2 {
        answer[1] = 2;
    }

    for i in 2..n {
        answer[i] = (answer[i-1] + answer[i-2]) % 10007;
    }

    let _ = writeln!(stdout, "{}", answer[n-1]);
    let _ = stdout.flush();
}