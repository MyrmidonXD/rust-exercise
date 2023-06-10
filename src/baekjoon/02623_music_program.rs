#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    orders: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Fresh,
    Visiting,
    Visited
}

fn solve(tc: TestCase) -> Vec<usize> {
    let TestCase { n, orders, .. } = tc;

    // Build an adjacency list
    // Note: edges can be included more that once, but they could be ignored while doing DFS
    let mut adj_list = Vec::with_capacity(n+1);
    for _ in 0..n+1 {
        adj_list.push(Vec::new());
    }
    for order in orders.iter() {
        for idx in 1..order.len() {
            adj_list[order[idx-1]].push(order[idx]);
        }
    }

    // Topological sort using iterative DFS with stack
    let mut visited = vec![VisitState::Fresh; n+1];
    let mut stack = Vec::new();
    let mut result = Vec::new();
    
    'outer: for i in 1..=n {
        if visited[i] == VisitState::Visited { continue; }

        stack.push(i);
        while let Some(j) = stack.pop() {
            match visited[j] {
                VisitState::Fresh => {
                    visited[j] = VisitState::Visiting;
                    stack.push(j);

                    // push neighbors to stack
                    for &next in adj_list[j].iter() {
                        if visited[next] == VisitState::Fresh {
                            stack.push(next);
                        } else if visited[next] == VisitState::Visiting {
                            // Cycle detected - return [0]
                            result.clear();
                            result.push(0);

                            break 'outer;
                        }
                        // else (VisitState::Visited) -> do nothing
                    }

                },
                VisitState::Visiting => {
                    // This must be popped after all neighbors are processed.
                    visited[j] = VisitState::Visited;
                    result.push(j);
                },
                _ => { continue; }
            }
        } 
    } 

    result.reverse();
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
        let order_len = inputs.next().unwrap();
        let mut order = Vec::with_capacity(order_len);
        for _ in 0..order_len {
            order.push(inputs.next().unwrap());
        }

        orders.push(order);
    }

    // Solve
    let result = solve(TestCase { n, m, orders });
    for i in result {
        let _ = writeln!(stdout, "{i}");
    }
}