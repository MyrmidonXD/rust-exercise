use std::io::{self, BufWriter, Write, Read};
use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);
    let xs: Vec<i32> = buffer.split_whitespace().skip(1).map(|s| s.parse::<i32>().unwrap()).collect();
    let mut sorted_xs = xs.clone();
    sorted_xs.sort_unstable();

    let mut compressing_map = HashMap::new();
    let mut idx: usize = 0;
    for x in sorted_xs {
        if !compressing_map.contains_key(&x) {
            compressing_map.insert(x, idx);
            idx += 1;
        }
    }

    let stdout = io::stdout();
    let mut stdout = BufWriter::with_capacity(xs.len(), stdout.lock());

    let mut is_first = true;
    for x in xs {
        if is_first {
            let _ = write!(stdout, "{}", compressing_map[&x]);
            is_first = false;
        } else {
            let _ = write!(stdout, " {}", compressing_map[&x]);
        }
    }

    let _ = stdout.flush();
}