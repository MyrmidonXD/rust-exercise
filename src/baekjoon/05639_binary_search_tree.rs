#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    xs: Vec<usize>,
}

fn to_postfix(prefix: &[usize]) -> Vec<usize> {
    let root = prefix[0];

    let mut right_child = prefix.len();
    for i in 1..prefix.len() {
        if prefix[i] > root {
            right_child = i;
            break;
        }
    }

    let left = &prefix[1..right_child];
    let right = &prefix[right_child..];

    let result = if left.len() == 0 && right.len() == 0 {
        vec![root]
    } else if left.len() > 0 && right.len() == 0 {
        let mut left_postfix = to_postfix(left);
        left_postfix.push(root);
        left_postfix
    } else if left.len() == 0 && right.len() > 0 {
        let mut right_postfix = to_postfix(right);
        right_postfix.push(root);
        right_postfix
    } else {
        let mut left_postfix = to_postfix(left);
        let right_postfix = to_postfix(right);
        left_postfix.extend(right_postfix);
        left_postfix.push(root);
        left_postfix
    };

    result
}

fn solve(tc: TestCase) -> Vec<usize> {
    to_postfix(&tc.xs[..])    
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
    let inputs = buffer.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    let xs = inputs.collect();

    // Solve
    let result = solve(TestCase { xs });
    
    for x in result {
        let _ = writeln!(stdout, "{x}");
    }
}