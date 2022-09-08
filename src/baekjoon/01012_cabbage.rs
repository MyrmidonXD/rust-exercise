use std::io::{self, BufRead, Lines, StdinLock};
use std::convert::TryInto;

struct TestCase {
    ps: Vec<i32>,
    board_size: (i32, i32)
}

fn get_point(board_size: (i32, i32), p: i32, ofs: (i32, i32)) -> Option<i32> {
    let (m, n) = board_size;
    let (x, y) = (p % m, p / m);
    let (dx, dy) = ofs;

    let (px, py) = (x + dx, y + dy);
    if px >= 0  && px < m && py >= 0 && py < n {
        Some(px + py * m)
    } else {
        None
    }
}

fn dfs_point(board: &mut Vec<bool>, board_size: (i32, i32), p: i32) {
    board[p as usize] = false;

    for ofs in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        match get_point(board_size, p, ofs) {
            Some(next) if board[next as usize] => {
                dfs_point(board, board_size, next)
            },
            _ => ()
        };
    }
}

fn get_next_tc(lines: &mut Lines<StdinLock>) -> TestCase {
    let [m, n, k]: [i32; 3] = lines.next()
                                   .unwrap()
                                   .expect("IO error")
                                   .split(' ')
                                   .map(|s| s.parse::<i32>().unwrap())
                                   .collect::<Vec<i32>>()
                                   .try_into()
                                   .unwrap();

    let mut ps = Vec::new();
    for _ in 0..k {
        let [x, y]: [i32; 2] = lines.next()
                                    .unwrap()
                                    .expect("IO error")
                                    .split(' ')
                                    .map(|s| s.parse::<i32>().unwrap())
                                    .collect::<Vec<i32>>()
                                    .try_into()
                                    .unwrap();
        ps.push(x + y * m);
    }

    TestCase { ps, board_size: (m, n) }
}

fn solve(tc: TestCase) {
    let (m, n) = tc.board_size;
    let mut board: Vec<bool> = vec![false; (m as usize)*(n as usize)];

    for p in tc.ps {
        board[p as usize] = true;
    }

    let mut result = 0;

    for i in 0..(m*n) {
        if board[i as usize] {
            result += 1;
            dfs_point(&mut board, tc.board_size, i);
        }
    }

    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let tc_num: u32 = lines.next().unwrap().expect("IO error").parse().unwrap();
    for _ in 0..tc_num {
        let tc = get_next_tc(&mut lines);
        solve(tc);
    }
}