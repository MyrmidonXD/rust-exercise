#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::VecDeque;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    k: usize,
}

fn solve(tc: TestCase) -> (usize, usize) {
    let TestCase { n, k } = tc;

    let mut queue = VecDeque::new();
    let mut visited = vec![false; 100001];
    let mut dist = vec![usize::MAX; 100001];
    let mut arrival_cases = vec![0; 100001];

    queue.push_back(n);
    visited[n] = false;
    dist[n] = 0;
    arrival_cases[n] = 1;

    while let Some(x) = queue.pop_front() {
        if x == k { break; }
        let d = dist[x];

        // X - 1
        if x > 0 {
            let next = x-1;

            if !visited[next] {
                queue.push_back(next);
                visited[next] = true;
                dist[next] = d + 1;
            }

            if dist[next] == d + 1 {
                arrival_cases[next] += arrival_cases[x];
            }
        }

        // X + 1
        if x < 100000 {
            let next = x+1;

            if !visited[next] {
                queue.push_back(next);
                visited[next] = true;
                dist[next] = d + 1;
            }

            if dist[next] == d + 1 {
                arrival_cases[next] += arrival_cases[x];
            }
        }

        // 2 * x
        if x <= 50000 {
            let next = 2 * x;

            if !visited[next] {
                queue.push_back(next);
                visited[next] = true;
                dist[next] = d + 1;
            }

            if dist[next] == d + 1 {
                arrival_cases[next] += arrival_cases[x];
            }
        }
    }

    (dist[k], arrival_cases[k])
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

    // Solve
    let result = solve(TestCase { n, k });
    
    let _ = writeln!(stdout, "{}", result.0);
    let _ = writeln!(stdout, "{}", result.1);
}