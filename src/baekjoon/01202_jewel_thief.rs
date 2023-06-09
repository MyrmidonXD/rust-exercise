#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::BinaryHeap;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    k: usize,
    jewels: Vec<(usize, usize)>,
    bags: Vec<usize>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, k, mut jewels, mut bags } = tc;

    jewels.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    bags.sort_unstable();

    let mut jewel_value_sum = 0;
    let mut jewel_value_queue = BinaryHeap::new();
    let mut jewel_index = 0;

    for bag_index in 0..k {
        let bag_size = bags[bag_index];

        while jewel_index < n && jewels[jewel_index].0 <= bag_size {
            jewel_value_queue.push(jewels[jewel_index].1);
            jewel_index += 1;
        }

        if let Some(jewel_max_value) = jewel_value_queue.pop() {
            jewel_value_sum += jewel_max_value;
        }
    }

    jewel_value_sum
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
    let k = inputs.next().unwrap();

    let mut jewels = Vec::with_capacity(n);
    for _ in 0..n {
        let m = inputs.next().unwrap();
        let v = inputs.next().unwrap();
        jewels.push((m, v));
    }

    let bags = inputs.collect();

    // Solve
    let result = solve(TestCase { n, k, jewels, bags });
    
    let _ = writeln!(stdout, "{result}");
}