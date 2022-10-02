#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    k: usize,
    costs: Vec<usize>,
    orders: Vec<(usize, usize)>,
    target: usize,
}

fn topological_sort_dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, order: &mut Vec<usize>, i: usize) {
    visited[i] = true;
    for &next in &graph[i] {
        if !visited[next] {
            topological_sort_dfs(graph, visited, order, next);
        }
    }

    order.push(i);
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, costs, orders, target, .. } = tc;
    let target = target - 1;

    let mut forward_graph = vec![Vec::with_capacity(n); n];
    let mut backward_graph = vec![Vec::with_capacity(n); n];

    for (x, y) in orders {
        let (x, y) = (x-1, y-1);
        
        forward_graph[x].push(y);
        backward_graph[y].push(x);
    }

    // topological sort
    let mut visited = vec![false; n];
    let mut topological_order = Vec::with_capacity(n);

    for i in 0..n {
        if !visited[i] {
            topological_sort_dfs(&forward_graph, &mut visited, &mut topological_order, i);
        }
    }

    // DP (minimum required time)
    let mut min_times = vec![0; n];
    for i in (0..n).rev() {
        let building = topological_order[i];
        let cost = costs[building];
        
        let mut max_prev_min_time = 0;
        for &prev in &backward_graph[building] {
            max_prev_min_time = max_prev_min_time.max(min_times[prev]);
        }

        min_times[building] = max_prev_min_time + cost;
    }

    min_times[target]
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

    let t = inputs.next().unwrap();

    for _ in 0..t {
        let n = inputs.next().unwrap();
        let k = inputs.next().unwrap();

        let costs = inputs.by_ref().take(n).collect();
        
        let mut orders = Vec::with_capacity(k);
        for _ in 0..k {
            let x = inputs.next().unwrap();
            let y = inputs.next().unwrap();
            orders.push((x, y))
        }

        let target = inputs.next().unwrap();

        // Solve
        let result = solve(TestCase { n, k, costs, orders, target });
        
        let _ = writeln!(stdout, "{result}");
    }
}