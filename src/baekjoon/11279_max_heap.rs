use std::io::{self, BufRead, Write};
use std::collections::BinaryHeap;

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
   
    let n: usize = lines.next().unwrap().expect("IO error").parse().unwrap();
    let mut ops = lines.map(|s| s.expect("IO error").parse::<usize>().unwrap());

    let mut heap = BinaryHeap::with_capacity(100000);
    let mut output = String::with_capacity(200000);
    let mut output_empty = true;

    for _ in 0..n {
        let x = ops.next().unwrap();

        if x == 0 {
            output.push_str(&format!("{}\n", heap.pop().unwrap_or(0)));
            output_empty = false;
        } else {
            heap.push(x);
        }
    }

    if !output_empty {
        let _ = output.pop(); // remove trailing newline
    }

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let _ = write!(stdout, "{}", output);
}