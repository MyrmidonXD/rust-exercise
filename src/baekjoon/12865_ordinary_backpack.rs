#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    k: usize,
    items: Vec<(usize, usize)>,
}

fn knapsack_dp(items: &Vec<(usize, usize)>, memo: &mut HashMap<(usize, usize), usize>, i: usize, k: usize) -> usize {
    if memo.contains_key(&(i, k)) {
        return memo[&(i, k)];
    }

    if i >= items.len() || k == 0 {
        return 0;
    }

    let (w, v) = items[i];
    if w > k {
        let max = knapsack_dp(items, memo, i+1, k);

        memo.insert((i, k), max);
        max
    } else {
        let mut max = v + knapsack_dp(items, memo, i+1, k-w);
        max = max.max(knapsack_dp(items, memo, i+1, k));

        memo.insert((i, k), max);
        max
    }
}

fn solve(tc: TestCase) -> usize {
    let mut items = tc.items;
    let k = tc.k;

    items.sort_unstable_by_key(|e| Reverse(*e));

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    knapsack_dp(&items, &mut memo, 0, k)
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

    let mut items = vec![];

    for _ in 0..n {
        let w = inputs.next().unwrap();
        let v = inputs.next().unwrap();

        items.push((w, v));
    }

    // Solve
    let result = solve(TestCase { n, k, items });
    
    let _ = writeln!(stdout, "{result}");
}