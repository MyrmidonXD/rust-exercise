#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
}

// k_n = 2 * k_n-1 + 1
// => (1 -> 3 n개의 disk 옮기기) = (1 -> 2 n-1개의 disk 옮기기) + (1 -> 3 크기 n의 disk 옮기기) + (2 -> 3 n-1개의 disk 옮기기)

fn solve(tc: TestCase) -> (u128, Option<Vec<(usize, usize)>>) {
    let TestCase { n } = tc;
    let k = 2u128.pow(n as u32) - 1;

    if n > 20 {
        (k, None)
    } else {
        let order = hanoi(n);
        (k, Some(order))
    }
}

fn hanoi(n: usize) -> Vec<(usize, usize)> {
    if n == 1 {
        return vec![(1, 3)];
    }

    let tmp = hanoi(n-1);
    let first_half = tmp.iter().map(swap_nums(3, 2));
    let last_half = tmp.iter().map(swap_nums(2, 1));

    first_half
        .chain(std::iter::once((1, 3)))
        .chain(last_half)
        .collect::<Vec<(usize, usize)>>()
}

fn swap_nums(a: usize, b: usize) -> impl FnMut(&(usize, usize)) -> (usize, usize) {
    move |&(u, v): &(usize, usize)| {
        let _u = if u == a { b } else if u == b { a } else { u };
        let _v = if v == a { b } else if v == b { a } else { v };
        (_u, _v)
    }
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

    // Solve
    let (k, order) = solve(TestCase { n });
    
    let _ = writeln!(stdout, "{k}");

    if let Some(ord) = order {
        for (a, b) in ord {
            let _ = writeln!(stdout, "{a} {b}");
        }
    }
}