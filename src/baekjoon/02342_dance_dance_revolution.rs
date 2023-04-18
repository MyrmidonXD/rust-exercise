#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    seq: Vec<usize>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { seq } = tc;

    let mut curr = [usize::MAX; 25];
    curr[0] = 0;

    for cmd in seq {
        if cmd == 0 { break; }

        let prev = curr;
        curr = [usize::MAX; 25];

        for pos in 0..25 {
            let (l_pos, r_pos) = get_position_pair(pos);

            if (l_pos == r_pos) || (l_pos != cmd && r_pos != cmd) { continue; }

            for prev_pos in 0..25 {
                if prev[prev_pos] == usize::MAX { continue; }

                let (l_prev_pos, r_prev_pos) = get_position_pair(prev_pos);
                match get_power((l_prev_pos, r_prev_pos), (l_pos, r_pos)) {
                    Some(power) => { curr[pos] = curr[pos].min(prev[prev_pos] + power); }
                    None => { continue; }
                }
            }
        }
    }

    let min = curr.iter().min().unwrap();
    *min
}

fn get_power(from: (usize, usize), to: (usize, usize)) -> Option<usize> {
    let (a, b) = from;
    let (x, y) = to;

    // Both feet moved at once -> invalid move
    if x != a && y != b { return None };

    let diff_pair = if x != a { (a, x) } else { (b, y) };
    if diff_pair.0 == diff_pair.1 {
        // Has not moved
        return Some(1);
    } else if diff_pair.0 == 0 {
        // One foot starting from 0
        return Some(2);
    } else if diff_pair.0.abs_diff(diff_pair.1) % 2 == 1 {
        // One foot moving 90 degree
        return Some(3);
    } else {
        // One foot moving 180 degree (i.e. opposite side)
        return Some(4);
    }
}

fn get_position_pair(pos_index: usize) -> (usize, usize) {
    (pos_index / 5, pos_index % 5)
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

    let seq = inputs.collect();

    // Solve
    let result = solve(TestCase { seq });
    
    let _ = writeln!(stdout, "{result}");
}