#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::collections::{HashMap};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    s: i32,
    seq: Vec<i32>,
}

fn subseq_sum_count(seq: &[i32], result: &mut HashMap<i32, usize>, start: usize, end: usize, sum: i32) {
    if start == end {
        result.entry(sum).and_modify(|cnt| *cnt += 1).or_insert(1);
        result.entry(sum + seq[start]).and_modify(|cnt| *cnt += 1).or_insert(1);
    } else {
        subseq_sum_count(seq, result, start+1, end, sum);
        subseq_sum_count(seq, result, start+1, end, sum + seq[start]);
    }
}

fn subseq_sum_count_with_precalc(seq: &[i32], precalc: &HashMap<i32, usize>, start: usize, end: usize, sum: i32, target: i32) -> usize {
    if start == end {
        let mut result = 0;

        let tail_subseq_sum = sum;
        if let Some(cnt) = precalc.get(&(target - tail_subseq_sum)) {
            result += cnt;
        }

        let tail_subseq_sum = sum + seq[start];
        if let Some(cnt) = precalc.get(&(target - tail_subseq_sum)) {
            result += cnt;
        }

        result
    } else {
        subseq_sum_count_with_precalc(seq, precalc, start+1, end, sum, target) + subseq_sum_count_with_precalc(seq, precalc, start+1, end, sum + seq[start], target)
    }
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, s, seq } = tc;

    if n == 1 {
        if seq[0] == s { return 1; } else { return 0; }
    }

    let half = (n / 2) - 1;

    let mut sums_count: HashMap<i32, usize> = HashMap::new();
    subseq_sum_count(&seq, &mut sums_count, 0, half, 0);
    
    let mut result = subseq_sum_count_with_precalc(&seq, &sums_count, half+1, n-1, 0, s);
    if s == 0 {
        result -= 1; // corner case when the selected subsequence is of 0 elements.
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = inputs.next().unwrap() as usize;
    let s = inputs.next().unwrap();

    let seq = inputs.collect();

    // Solve
    let result = solve(TestCase { n, s, seq });
    
    let _ = writeln!(stdout, "{result}");
}