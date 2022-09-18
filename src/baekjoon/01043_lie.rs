#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    t: usize,
    truths: Vec<usize>,
    parties: Vec<Vec<usize>>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, m, truths, parties, .. } = tc;

    let mut truth_set = vec![false; n+1];
    for truth in truths {
        truth_set[truth] = true;
    }
    let mut truth_party_set = vec![false; m];
    
    let mut mutated = true;
    while mutated {
        mutated = false;

        for (i, party) in parties.iter().enumerate() {
            if truth_party_set[i] { continue; }

            for &x in party {
                if truth_set[x] {
                    truth_party_set[i] = true;
                    mutated = true;
                    break;
                }
            }

            if truth_party_set[i] {
                for &x in party {
                    truth_set[x] = true;
                }
            }
        }
    }

    truth_party_set.into_iter().filter(|&x| x == false).count()
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
    let m = inputs.next().unwrap();

    let t = inputs.next().unwrap();
    let mut truths = Vec::with_capacity(t);
    for _ in 0..t {
        truths.push(inputs.next().unwrap())
    }

    let mut parties = Vec::with_capacity(m);
    for _ in 0..m {
        let p = inputs.next().unwrap();
        let mut party = Vec::with_capacity(p);
        for _ in 0..p {
            party.push(inputs.next().unwrap())
        }
        parties.push(party);
    }

    // Solve
    let result = solve(TestCase { n, m, t, truths, parties });
    
    let _ = writeln!(stdout, "{result}");
}