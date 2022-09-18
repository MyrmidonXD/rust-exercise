#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase<'a> {
    s: &'a str,
    bomb: &'a str,
}

fn solve(tc: TestCase) -> String {
    let TestCase { s, bomb } = tc;

    let bomb: Vec<char> = bomb.chars().collect();
    let bomb_len = bomb.len();

    let mut stack = vec![];
    for c in s.chars() {
        stack.push(c);

        let stack_len = stack.len();
        if stack_len < bomb_len { continue; }

        for i in (0..bomb_len).rev() {
            if stack[(stack_len-1)-((bomb_len-1) - i)] != bomb[i] { break; }

            if i == 0 { // matched a bomb
                for _ in 0..bomb_len {
                    stack.pop();
                }
            }
        }
    }

    if stack.is_empty() {
        "FRULA".to_string()
    } else {
        stack.into_iter().collect::<String>()
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
    let mut lines = buffer.lines();

    let s = lines.next().unwrap();
    let bomb = lines.next().unwrap();

    // Solve
    let result = solve(TestCase { s, bomb });
    
    let _ = writeln!(stdout, "{result}");
}