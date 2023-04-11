#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    k: usize,
    t1: Vec<usize>,
    t2: Vec<usize>,
}

fn calc_dist(ix: usize, iy: usize, k: usize) -> usize {
    if k <= 1 { return 0; }

    let mid = 2usize.pow((k-2) as u32);

    if ix < mid && iy < mid {
        calc_dist(ix, iy, k-1)
    } else if ix >= mid && iy >= mid {
        calc_dist(ix-mid, iy-mid, k-1)
    } else {
        2 * (k-1)
    }
}

fn solve(tc: TestCase) -> usize {
    let TestCase { k, t1, t2 } = tc;

    let n = 2usize.pow((k-1) as u32);
    let mut v1 = vec![0usize; n];
    let mut v2 = vec![0usize; n];

    for i in 0..n {
        v1[t1[i]-1] = i;
        v2[t2[i]-1] = i;
    }

    let mut max = 0;
    for x in 0..(n-1) {
        let mut count = 1;
        for y in x+1..n {
            if calc_dist(v1[x], v1[y], k) == calc_dist(v2[x], v2[y], k) {
                count += 1;
            }
        }

        max = max.max(count);
    }

    max
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

    let k = inputs.next().unwrap();
    let t1 = inputs.by_ref().take(2_usize.pow((k-1) as u32)).collect();
    let t2 = inputs.collect();

    // Solve
    let result = solve(TestCase { k, t1, t2 });
    
    let _ = writeln!(stdout, "{result}");
}