#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::{Index, IndexMut};
use std::collections::VecDeque;

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

impl<T> Matrix<T> {
    fn neighbor_indices(&self, (y, x): (usize, usize)) -> [Option<(usize, usize)>; 4] {
        [
            if y > 0 { Some((y-1, x)) } else { None },
            if y < self.row.saturating_sub(1) { Some((y+1, x)) } else { None },
            if x > 0 { Some((y, x-1)) } else { None },
            if x < self.col.saturating_sub(1) { Some((y, x+1)) } else { None },
        ]
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    board: Vec<bool>,
}

fn solve(tc: TestCase) -> i32 {
    let TestCase { m, n, board } = tc;

    let board = Matrix {
        row: n,
        col: m,
        data: board,
    };

    let mut visited = Matrix {
        row: n,
        col: m,
        data: vec![false; n * m],
    };

    let inf: usize = 2_000_000_000;
    let mut queue = VecDeque::new();
    let mut forward_dist = Matrix {
        row: n,
        col: m,
        data: vec![inf; n * m],
    };

    // 1st bfs: calculate distance from (0, 0)
    forward_dist[(0, 0)] = 1;
    queue.push_back((0, 0));
    visited[(0, 0)] = true;

    while let Some(idx) = queue.pop_front() {
        for neighbor_idx in IntoIterator::into_iter(board.neighbor_indices(idx)).filter_map(|e| e) {
            if board[neighbor_idx] && !visited[neighbor_idx] {
                forward_dist[neighbor_idx] = forward_dist[idx] + 1;
                queue.push_back(neighbor_idx);
                visited[neighbor_idx] = true;
            }
        }
    }

    // 2nd bfs: calculate distance from (n-1, m-1)
    let mut backward_dist = Matrix {
        row: n,
        col: m,
        data: vec![inf; n * m],
    };
    
    // reset visited matrix
    for v in &mut visited.data {
        *v = false;
    }

    backward_dist[(n-1, m-1)] = 1;
    queue.push_back((n-1, m-1));
    visited[(n-1, m-1)] = true;

    while let Some(idx) = queue.pop_front() {
        for neighbor_idx in IntoIterator::into_iter(board.neighbor_indices(idx)).filter_map(|e| e) {
            if board[neighbor_idx] && !visited[neighbor_idx] {
                backward_dist[neighbor_idx] = backward_dist[idx] + 1;
                queue.push_back(neighbor_idx);
                visited[neighbor_idx] = true;
            }
        }
    }

    // check distances including breaking a wall
    let mut min_dist = forward_dist[(n-1, m-1)];

    for y in 0..n {
        for x in 0..m {
            if !board[(y, x)] { 
                // If this cell is a wall, then check the distance including breaking this wall and passing on it
                let (mut min_forward, mut min_backward) = (inf, inf);

                for neighbor_idx in IntoIterator::into_iter(board.neighbor_indices((y, x))).filter_map(|e| e) {
                    min_forward = min_forward.min(forward_dist[neighbor_idx]);
                    min_backward = min_backward.min(backward_dist[neighbor_idx]);
                }

                min_dist = min_dist.min(min_forward + 1 + min_backward);
            }
        }
    }

    if min_dist == inf {
        -1
    } else {
        min_dist as i32
    }
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

    let mut first_line = inputs.next().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let (n, m) = (
        first_line.next().unwrap(),
        first_line.next().unwrap(),
    );

    let mut board = vec![];
    for _ in 0..n {
        let line = inputs.next().unwrap().chars();
        for c in line {
            board.push(c == '0');
        }
    }

    // Solve
    let result = solve(TestCase { n, m, board });
    
    let _ = writeln!(stdout, "{result}");
}