#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase<'a> {
    n: usize,
    clothes: HashMap<&'a str, Vec<&'a str>>,
}

fn solve(tc: TestCase) -> usize {
    let counts = tc.clothes.iter().map(|(_, val)| val.len() + 1);
    counts.product::<usize>() - 1
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
    let mut inputs = buffer.lines();

    let t: usize = inputs.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = inputs.next().unwrap().parse().unwrap();

        let mut clothes = HashMap::new();
        for _ in 0..n {
            let mut s = inputs.next().unwrap().split_whitespace();
            let name = s.next().unwrap();
            let kind = s.next().unwrap();

            let kinds = clothes.entry(kind).or_insert(vec![]);
            kinds.push(name);
        }
    
        let result = solve(TestCase { n, clothes });
        
        let _ = writeln!(stdout, "{result}");
    }
}