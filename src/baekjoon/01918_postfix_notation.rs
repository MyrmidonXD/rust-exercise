#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    expr: String,
}

fn solve(tc: TestCase) -> String {
    let mut result = String::new();
    let mut op_stack = vec![];

    for c in tc.expr.chars() {
        match c {
            '+' | '-' => {
                while !op_stack.is_empty() && op_stack[op_stack.len() - 1] != '(' {
                    let op = op_stack.pop().unwrap();
                    result.push(op);
                }

                op_stack.push(c);
            },
            '*' | '/' => {
                while !op_stack.is_empty() && !['(', '+', '-'].contains(&op_stack[op_stack.len() - 1]) {
                    let op = op_stack.pop().unwrap();
                    result.push(op);
                }

                op_stack.push(c);
            },
            '(' => {
                op_stack.push(c);
            },
            ')' => {
                while let Some(op) = op_stack.pop() {
                    if op == '(' {
                        break;
                    } else {
                        result.push(op);
                    }
                }
            },
            operand => {
                result.push(operand);
            },
        }
    }

    while let Some(op) = op_stack.pop() {
        result.push(op);
    }

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

    let expr = buffer.trim_end().to_string();

    // Solve
    let result = solve(TestCase { expr });
    
    let _ = writeln!(stdout, "{result}");
}