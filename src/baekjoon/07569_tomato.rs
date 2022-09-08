use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::collections::VecDeque;
use std::convert::TryInto;

#[derive(Debug)]
struct TestCase {
    m: usize,
    n: usize,
    h: usize,
    board: Vec<i32>
}

#[derive(Debug)]
struct Matrix3D<T> {
    row: usize,
    col: usize,
    height: usize,
    data: Vec<T>
}

impl<T> Index<(usize, usize, usize)> for Matrix3D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        match index {
            (h, _, _) if h >= self.height => { panic!("index out of bounds: the height size is {} but the height index is {}", self.height, h); },
            (_, y, _) if y >= self.row => { panic!("index out of bounds: the row size is {} but the row index is {}", self.row, y); },
            (_, _, x) if x >= self.col => { panic!("index out of bounds: the column size is {} but the column index is {}", self.col, x); }
            (h, y, x) => { return &self.data[h * self.row * self.col + y * self.col + x]; }
        }
    }
}

impl<T> IndexMut<(usize, usize, usize)> for Matrix3D<T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        match index {
            (h, _, _) if h >= self.height => { panic!("index out of bounds: the height size is {} but the height index is {}", self.height, h); },
            (_, y, _) if y >= self.row => { panic!("index out of bounds: the row size is {} but the row index is {}", self.row, y); },
            (_, _, x) if x >= self.col => { panic!("index out of bounds: the column size is {} but the column index is {}", self.col, x); }
            (h, y, x) => { return &mut self.data[h * self.row * self.col + y * self.col + x]; }
        }
    }
}

#[derive(Debug)]
struct State {
    m: usize,
    n: usize,
    h: usize,
    board: Matrix3D<i32>
}

fn get_initial_state(tc: &TestCase) -> State {
    let (m, n, h) = (tc.m, tc.n, tc.h);
    let board = Matrix3D { row: n, col: m, height: h, data: tc.board.to_owned() };

    return State { m, n, h, board };
}

fn bfs_board(s: &mut State) -> i32 {
    let mut queue: VecDeque<((usize, usize, usize), i32)> = VecDeque::new();

    // find initially ripen tomatoes and put those into the queue
    for h in 0..s.h {
        for y in 0..s.n {
            for x in 0..s.m {
                if s.board[(h, y, x)] == 1 {
                    queue.push_back(((h, y, x), 0));
                }
            }
        }
    }
    
    let mut max_d = -1;
    while !queue.is_empty() {
        let ((h, y, x), d) = queue.pop_front().unwrap();
        
        // update max distance
        max_d = d;

        // up
        if h > 0 && s.board[(h-1, y, x)] == 0 {
            queue.push_back(((h-1, y, x), d + 1));
            s.board[(h-1, y, x)] = 1;
        }
        // down
        if h < s.h-1 && s.board[(h+1, y, x)] == 0 {
            queue.push_back(((h+1, y, x), d + 1));
            s.board[(h+1, y, x)] = 1;
        }
        // front
        if y > 0 && s.board[(h, y-1, x)] == 0 {
            queue.push_back(((h, y-1, x), d + 1));
            s.board[(h, y-1, x)] = 1;
        }
        // left
        if x > 0 && s.board[(h, y, x-1)] == 0 {
            queue.push_back(((h, y, x-1), d + 1));
            s.board[(h, y, x-1)] = 1;
        }
        // back
        if y < s.n-1 && s.board[(h, y+1, x)] == 0 {
            queue.push_back(((h, y+1, x), d + 1));
            s.board[(h, y+1, x)] = 1;
        }
        // right
        if x < s.m-1 && s.board[(h, y, x+1)] == 0 {
            queue.push_back(((h, y, x+1), d + 1));
            s.board[(h, y, x+1)] = 1;
        }
    }

    // find an unripen tomato and set max_d as -1 to represent it
    'outer: for h in 0..s.h {
        for y in 0..s.n {
            for x in 0..s.m {
                if s.board[(h, y, x)] == 0 {
                    max_d = -1;
                    break 'outer;
                }
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

    let [m, n, h]: [usize; 3] = lines.next().unwrap().expect("IO error")
                                  .split_whitespace().map(|s| s.parse::<usize>().unwrap())
                                  .collect::<Vec<usize>>().try_into().unwrap();

    let mut board: Vec<i32> = Vec::new();

    while let Some(line) = lines.next() {
        line.unwrap().split_whitespace().for_each(|d| board.push(d.parse::<i32>().unwrap()));
    }

    solve(TestCase { m, n, h, board });
}