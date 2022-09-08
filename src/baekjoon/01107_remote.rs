use std::io::{self, BufRead};
use std::convert::{TryInto};

struct TestCase {
    ch: u32,
    buttons: Vec<u32>
}

fn is_possible(buttons: &Vec<u32>, channel: u32) -> bool {
    if channel == 0 {
        return buttons.contains(&0);
    }

    let mut n = channel;
    let mut result = true;

    while n > 0 {
        if !buttons.contains(&(n % 10)) {
            result = false;
            break;
        } else {
            n = n / 10;
        }
    }
    return result;
}

fn num_digits(n: u32) -> u32 {
    if n == 0 { return 1 }

    let mut n_temp = n;
    let mut n_digits = 0;

    while n_temp > 0 {
        n_digits += 1;
        n_temp /= 10;
    }
    return n_digits;
}

fn diff(a: u32, b: u32) -> u32 {
    if a < b { b - a } else { a - b }
}

fn solve(tc: TestCase) {
    let n_button = tc.buttons.len();
    let mut push_cases: Vec<u32> = Vec::new();

    // case 1: manual +/- from 100
    push_cases.push(diff(tc.ch, 100));

    // case 2: 0 -> manual +
    if n_button > 0 && tc.buttons[0] == 0 {
        push_cases.push(1 + tc.ch);
    }

    // case 3~
    if n_button > 0 && (tc.buttons[0] != 0 || n_button > 1) {
        // case 3: find smaller channel
        for channel in (1..tc.ch).rev() {
            if is_possible(&tc.buttons, channel) {
                push_cases.push(num_digits(channel) + diff(tc.ch, channel));
                break;
            }
        }

        // case 4: find larger or equal channel
        for channel in tc.ch.. {
            if is_possible(&tc.buttons, channel) {
                push_cases.push(num_digits(channel) + diff(channel, tc.ch));
                break;
            }
        }
    }

    let result = push_cases.into_iter().min().unwrap();
    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let ch: u32 = lines.next().unwrap().expect("IO error").parse().unwrap();
    let n_broken: u32 = lines.next().unwrap().expect("IO error").parse().unwrap();

    let mut broken: Vec<u32> = Vec::new();
    if n_broken > 0 {
        broken = lines.next().unwrap().expect("IO error").split(' ').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>().try_into().unwrap();
    }

    let mut buttons: Vec<u32> = Vec::new();
    for i in 0..=9 {
        if !broken.contains(&i) { buttons.push(i as u32) }
    }

    solve(TestCase { ch, buttons });
}