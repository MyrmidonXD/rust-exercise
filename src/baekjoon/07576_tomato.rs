use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::collections::VecDeque;
use std::convert::TryInto;

#[derive(Debug)]
struct TestCase {
    m: usize,
    n: usize,
    board: Vec<i32>
}

#[derive(Debug)]
struct Matrix<T> {
    row: usize,
    col: usize,
    data: Vec<T>
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match index {
            (y, _) if y >= self.row => { panic!("index out of bounds: the row size is {} but the row index is {}", self.row, y); },
            (_, x) if x >= self.col => { panic!("index out of bounds: the column size is {} but the column index is {}", self.col, x); }
            (y, x) => { return &self.data[y * self.col + x]; }
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match index {
            (y, _) if y >= self.row => { panic!("index out of bounds: the row size is {} but the row index is {}", self.row, y); },
            (_, x) if x >= self.col => { panic!("index out of bounds: the column size is {} but the column index is {}", self.col, x); }
            (y, x) => { return &mut self.data[y * self.col + x]; }
        }
    }
}

#[derive(Debug)]
struct State {
    m: usize,
    n: usize,
    board: Matrix<i32>
}

fn get_initial_state(tc: &TestCase) -> State {
    let (m, n) = (tc.m, tc.n);
    let board = Matrix { row: n, col: m, data: tc.board.to_owned() };

    return State { m, n, board };
}

fn bfs_board(s: &mut State) -> i32 {
    let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::new();

    // find initially ripen tomatoes and put those into the queue
    for y in 0..s.n {
        for x in 0..s.m {
            if s.board[(y, x)] == 1 {
                queue.push_back(((y, x), 0));
            }
        }
    }
    
    let mut max_d = -1;
    while !queue.is_empty() {
        let ((y, x), d) = queue.pop_front().unwrap();
        
        // update max distance
        max_d = d;

        // up
        if y > 0 && s.board[(y-1, x)] == 0 {
            queue.push_back(((y-1, x), d + 1));
            s.board[(y-1, x)] = 1;
        }
        // left
        if x > 0 && s.board[(y, x-1)] == 0 {
            queue.push_back(((y, x-1), d + 1));
            s.board[(y, x-1)] = 1;
        }
        // down
        if y < s.n-1 && s.board[(y+1, x)] == 0 {
            queue.push_back(((y+1, x), d + 1));
            s.board[(y+1, x)] = 1;
        }
        // right
        if x < s.m-1 && s.board[(y, x+1)] == 0 {
            queue.push_back(((y, x+1), d + 1));
            s.board[(y, x+1)] = 1;
        }
    }

    // find an unripen tomato and set max_d as -1 to represent it
    'outer: for y in 0..s.n {
        for x in 0..s.m {
            if s.board[(y, x)] == 0 {
                max_d = -1;
                break 'outer;
            }
        }
    }

    return max_d;
}

fn solve(tc: TestCase) {
    let mut state = get_initial_state(&tc);

    let result = bfs_board(&mut state);
    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let [m, n]: [usize; 2] = lines.next().unwrap().expect("IO error")
                                  .split_whitespace().map(|s| s.parse::<usize>().unwrap())
                                  .collect::<Vec<usize>>().try_into().unwrap();

    let mut board: Vec<i32> = Vec::new();

    while let Some(line) = lines.next() {
        line.unwrap().split_whitespace().for_each(|d| board.push(d.parse::<i32>().unwrap()));
    }

    solve(TestCase { m, n, board });
}