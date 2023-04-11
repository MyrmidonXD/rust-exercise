#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    a: u64,
    b: u64,
}

/*
    1000
    1001
    1010
    1011
    1100
    1101
    1110
    1111

    => 2^k <= x < 2^(k+1) 인 경우 k-th 자리는 2^k개, (k-1)-th 자리는 2^(k-1)개, (k-2)-th 자리도 2^(k-1)개, ..., 0-th 자리도 2^(k-1)
    => (k + 2) * 2 ^ (k - 1)

    앞부분 (A ~ 2^(k+1) - 1) 구하기

    1011 0110 <- A 
    1011 0111
    1011 1000
    1011 1001
    1011 1010
    1011 1011
    1011 1100
    1011 1101
    1011 1110
    1011 1111
    ...
    1111 1111

    bit 1: A가 짝수면 (2^(k+1) - A) / 2, 홀수면 (2^(k+1) - A) / 2 + 1
    bit 2: A % 4 = r이면 ((2^(k+1) - A) / 4) * 2 + ((r <= 4 / 2) ? 4 / 2 : 4 - r
    ...
    bit i: A % 2^i = r 이면 ((2^(k+1) - A) / 2^i) * 2^(i-1) + ((r <= 2^(i-1)) ? 2^(i-1) : 2^i - r)

    ---
    뒷부분 (2^k ~ B) 구하기 => 동일, 식 조금만 다름
    bit i: B % 2^i = r 이면 ((B - 2^k) / 2^i) * 2^(i-1) + ((r < 2^(i-1))) ? 0 : r + 1 - 2^(i-1))
 */

// returns (k, l) pair which satisfies n = 2^k + l, 0 <= l < 2^k
fn find_exponent_and_remainder(n: u64) -> (u32, u64) {
    let mut prev = 1;
    let mut count: u32 = 0;

    while n >= 2 * prev {
        prev *= 2;
        count += 1;
    }

    (count, n - prev)
}

// counts sum of number of 1s in the binary representation of x, 2^k <= x < 2^(k+1)
fn count_1s_between_power_of_twos(k: u32) -> u64 {
    if k == 0 { 1 } else { ((k + 2) as u64) * 2u64.pow(k - 1) }
}

// counts sum of number of 1s in the binary representation of x, 2^k+a <= x < 2^(k+1)
fn count_1s_until_next_power_of_two(k: u32, a: u64) -> u64 {
    let mut sum = count_1s_between_power_of_twos(k);
    if a > 0 {
        sum -= count_1s_from_previous_power_of_two(k, a-1);
    }

    sum
}

// counts sum of number of 1s in the binary representation of x, 2^k <= x <= 2^k + b 
fn count_1s_from_previous_power_of_two(k: u32, b: u64) -> u64 {
    let mut sum = b + 1;

    let mut pow_i = 2;
    for _ in 1..=k {
        let r = b % pow_i;
        sum += (b / pow_i) * (pow_i / 2) + (if r < pow_i / 2 { 0 } else { r + 1 - (pow_i / 2) });
        pow_i *= 2;
    }

    sum
}

fn solve(tc: TestCase) -> u64 {
    let TestCase { a, b } = tc;
    let (ka, ra) = find_exponent_and_remainder(a);
    let (kb, rb) = find_exponent_and_remainder(b);

    let mut sum = count_1s_until_next_power_of_two(ka, ra);
    for k in ka+1..kb {
        sum += count_1s_between_power_of_twos(k);
    }
    sum += count_1s_from_previous_power_of_two(kb, rb);

    if ka == kb {
        sum -= count_1s_between_power_of_twos(ka);
    }

    sum
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<u64>().unwrap());

    let a = inputs.next().unwrap();
    let b = inputs.next().unwrap();

    // Solve
    let result = solve(TestCase { a, b });
    
    let _ = writeln!(stdout, "{result}");
}