#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::VecDeque;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    orders: Vec<(usize, usize)>,
}

fn solve(tc: TestCase) -> Vec<usize> {
    let TestCase { n, orders, .. } = tc;
    let mut inbound_connections = vec![0i32; n+1];
    let mut adj_list = vec![vec![]; n+1];

    for (a, b) in orders {
        adj_list[a].push(b);
        inbound_connections[b] += 1;
    }

    let mut outbound_only_queue = VecDeque::new();
    for i in 1..=n {
        if inbound_connections[i] == 0 {
            outbound_only_queue.push_back(i);
        }
    }

    let mut result = Vec::with_capacity(n);
    while let Some(i) = outbound_only_queue.pop_front() {
        result.push(i);

        for &j in adj_list[i].iter() {
            inbound_connections[j] -= 1;
            if inbound_connections[j] == 0 {
                outbound_only_queue.push_back(j);
            }
        } 
    }

    result
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
    let m = inputs.next().unwrap();

    let mut orders = Vec::with_capacity(m);
    for _ in 0..m {
        let a = inputs.next().unwrap();
        let b = inputs.next().unwrap();
        orders.push((a, b));
    }

    // Solve
    let result = solve(TestCase { n, m, orders });
    let result_str = result.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    
    let _ = writeln!(stdout, "{result_str}");
}