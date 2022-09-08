use std::io::{self, BufRead};
use std::fmt::{Result, Write};

#[derive(Debug)]
struct TestCase {
    p: String,
    n: usize,
    v: Vec<u8>
}

fn solve(tc: TestCase) -> Result {
    let TestCase { p, n, v } = tc;

    let mut reversed = false;
    let mut pop_front = 0;
    let mut pop_back = 0;

    for c in p.chars() {
        match c {
            'R' => reversed = !reversed,
            'D' => if reversed { pop_back += 1; } else { pop_front += 1; },
            _ => ()
        }
    }

    if pop_front + pop_back > n {
        println!("error");
        return Ok(());
    }

    let result_array: Vec<&u8> = if reversed { 
        v[pop_front..v.len()-pop_back].iter().rev().collect()
    } else {
        v[pop_front..v.len()-pop_back].iter().collect()
    };

    let result_len = result_array.len();
    let mut result_str = String::from("[");
    for n in result_array {
        write!(&mut result_str, "{},", n)?;
    }

    // removes trailing comma and adds closing braket
    if result_len > 0 {
        result_str.pop();
    }
    result_str.push(']');

    println!("{}", result_str);
    Ok(())
}

fn main() -> Result {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().expect("IO error").parse().unwrap();

    for _ in 0..t {
        let p = lines.next().unwrap().expect("IO error");
        let n = lines.next().unwrap().expect("IO error").parse().unwrap();
        
        let v = lines.next().unwrap().expect("IO error");
        let v = &v[1..v.len()-1];
        let v: Vec<u8> = if n == 0 { Vec::new() } else { v.split(',').map(|x| x.parse().unwrap()).collect() };

        solve(TestCase { p, n, v })?;
    }

    Ok(())
}