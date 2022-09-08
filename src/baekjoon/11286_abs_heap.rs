use std::io::{self, BufReader, BufWriter, Write, Read};
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Abs(i32);

impl Ord for Abs {
    fn cmp(&self, other: &Self) -> Ordering {
        let abs_order = self.0.abs().cmp(&other.0.abs());
        match abs_order {
            Ordering::Equal => self.0.cmp(&other.0),
            _ => abs_order,
        }
    }
}

impl PartialOrd for Abs {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::with_capacity(200000, stdin.lock());
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);
    let mut ops = buffer.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let mut heap = BinaryHeap::with_capacity(100000);

    let stdout = io::stdout();
    let mut stdout = BufWriter::with_capacity(200000, stdout.lock());

    let n = ops.next().unwrap();
    for _ in 0..n {
        let op = ops.next().unwrap();

        if op == 0 {
            let min = if let Some(Reverse(Abs(x))) = heap.pop() {
                x
            } else {
                0
            };
            let _ = writeln!(stdout, "{min}");
        } else {
            heap.push(Reverse(Abs(op)));
        }
    }

    let _ = stdout.flush();
}