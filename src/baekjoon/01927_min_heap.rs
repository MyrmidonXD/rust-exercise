use std::io::{self, BufRead, BufWriter, Write, Read};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();
   
    let _ = stdin.read_line(&mut buffer);
    let n: usize = buffer.trim_end().parse().unwrap();
    buffer.clear();

    let _ = stdin.read_to_string(&mut buffer);
    let mut ops = buffer.split_whitespace().map(|s| s.parse::<u32>().unwrap());

    let mut heap = BinaryHeap::with_capacity(100000);

    let stdout = io::stdout();
    let mut stdout = BufWriter::with_capacity(200000, stdout.lock());

    for _ in 0..n {
        let op = ops.next().unwrap();

        if op == 0 {
            let min = if let Some(Reverse(n)) = heap.pop() {
                n
            } else {
                0
            };
            let _ = writeln!(stdout, "{min}");
        } else {
            heap.push(Reverse(op));
        }
    }

    let _ = stdout.flush();
}