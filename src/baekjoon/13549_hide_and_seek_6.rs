#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::VecDeque;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    k: usize,
}

// 0-1 BFS
fn solve(tc: TestCase) -> usize {
    let TestCase { n, k } = tc;

    let mut dist = vec![usize::MAX; 100001];
    let mut queue = VecDeque::with_capacity(100);

    if n == k { return 0; }

    dist[n] = 0;
    queue.push_back(n);

    while let Some(x) = queue.pop_front() {
        if x * 2 == k {
            dist[k] = dist[x];
            break;
        } else if x.saturating_sub(1) == k || x + 1 == k {
            dist[k] = dist[x] + 1;
            break;
        }

        // 2 * X
        if 2 * x <= 100000 && dist[2*x] == usize::MAX {
            queue.push_front(2*x);
            dist[2*x] = dist[x];
        }

        // X - 1
        if x > 0 && dist[x-1] == usize::MAX {
            queue.push_back(x-1);
            dist[x-1] = dist[x] + 1;
        }

        // X + 1
        if x < 100000 && dist[x+1] == usize::MAX {
            queue.push_back(x+1);
            dist[x+1] = dist[x] + 1;
        }
    }

    dist[k]
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
    
    let _ = writeln!(stdout, "{result}");
}