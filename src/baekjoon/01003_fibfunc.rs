use std::io::{self, BufRead};

fn fib_memo(m: &mut Vec<u32>, n: u32) -> u32 {
    if let Some(&x) = m.get(n as usize) {
        x
    } else {
        let x = fib_memo(m, n - 1) + fib_memo(m, n - 2);
        m.push(x);
        x
    }
}

fn main() {
    let mut fib_seq = vec![0u32, 1u32, 1u32];

    let stdin = io::stdin();
    for line in stdin.lock().lines().skip(1) {
        let case: u32 = line.unwrap().parse().unwrap();
        if case == 0 {
            println!("1 0");
        } else {
            println!("{} {}", fib_memo(&mut fib_seq, case - 1), fib_memo(&mut fib_seq, case));
        }
    }
}