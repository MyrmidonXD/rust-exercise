#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    in_order: Vec<usize>,
    post_order: Vec<usize>,
}

fn build_pre_order(pre_order: &mut Vec<usize>, in_order: &[usize], post_order: &[usize]) {
    if post_order.len() == 0 {
        return;
    }

    let root = post_order[post_order.len() - 1];
    let inorder_root_idx = in_order.iter().position(|&x| x == root).unwrap();

    let in_order_left = &in_order[0..inorder_root_idx];
    let in_order_right = &in_order[inorder_root_idx+1..];

    let post_order_left = &post_order[0..inorder_root_idx];
    let post_order_right = &post_order[inorder_root_idx..post_order.len()-1];

    pre_order.push(root);
    build_pre_order(pre_order, in_order_left, post_order_left);
    build_pre_order(pre_order, in_order_right, post_order_right);
}

fn solve(tc: TestCase) -> Vec<usize> {
    let mut pre_order = vec![];
    build_pre_order(&mut pre_order, &tc.in_order, &tc.post_order);

    pre_order
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
    
    let mut in_order = vec![];
    for _ in 0..n {
        in_order.push(inputs.next().unwrap());
    }

    let mut post_order = vec![];
    for _ in 0..n {
        post_order.push(inputs.next().unwrap());
    }

    // Solve
    let result = solve(TestCase { n, in_order, post_order });
    let result_str = result.into_iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" ");
    
    let _ = writeln!(stdout, "{result_str}");
}