use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct TestCase {
    n: usize,
    board: Vec<u32>
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
            (y, _) if y >= self.row => { panic!("index out of bounds: the row size is {} but the row index is {}", y, self.row); },
            (_, x) if x >= self.col => { panic!("index out of bounds: the column size is {} but the column index is {}", x, self.col); }
            (y, x) => { return &self.data[y * self.row + x]; }
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match index {
            (y, _) if y >= self.row => { panic!("index out of bounds: the row size is {} but the row index is {}", y, self.row); },
            (_, x) if x >= self.col => { panic!("index out of bounds: the column size is {} but the column index is {}", x, self.col); }
            (y, x) => { return &mut self.data[y * self.row + x]; }
        }
    }
}

#[derive(Debug)]
struct State {
    n: usize,
    board: Matrix<u32>,
    pos: (usize, usize), // (y, x)
    elapsed: u32,
    size: u32,
    eaten: u32,
}

fn get_initial_state(tc: &TestCase) -> State {
    let n = tc.n;
    let board = Matrix { row: n, col: n, data: tc.board.to_owned() };

    let mut pos = (0, 0);
    'outer: for y in 0..n {
        for x in 0..n {
            if board[(y, x)] == 9 {
                pos = (y, x);
                break 'outer;
            }
        }
    }

    return State { n, board, pos, elapsed: 0u32, size: 2, eaten: 0 };
}

fn bfs_board(s: &State) -> Option<((usize, usize), usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

    let mut min_d = s.n * s.n;
    let mut possible_next: Vec<(usize, usize)> = Vec::new();
    queue.push_back((s.pos, 0));
    visited.insert(s.pos);

    while !queue.is_empty() {
        let ((y, x), d) = queue.pop_front().unwrap();
        if d > min_d { break; }
        if s.board[(y, x)] > 0 && s.board[(y, x)] < s.size && s.board[(y, x)] < 9 {
            min_d = d;
            possible_next.push((y, x));
        }
        
        // up
        if y > 0 && !visited.contains(&(y-1, x)) && s.board[(y-1, x)] <= s.size {
            queue.push_back(((y-1, x), d + 1));
            visited.insert((y-1, x));
        }
        // left
        if x > 0 && !visited.contains(&(y, x-1)) && s.board[(y, x-1)] <= s.size {
            queue.push_back(((y, x-1), d + 1));
            visited.insert((y, x-1));
        }
        // down
        if y < s.n-1 && !visited.contains(&(y+1, x)) && s.board[(y+1, x)] <= s.size {
            queue.push_back(((y+1, x), d + 1));
            visited.insert((y+1, x));
        }
        // right
        if x < s.n-1 && !visited.contains(&(y, x+1)) && s.board[(y, x+1)] <= s.size {
            queue.push_back(((y, x+1), d + 1));
            visited.insert((y, x+1));
        }
    }

    // find topmost, leftmost point
    possible_next.sort();
    match possible_next.get(0) {
        Some(&(y, x)) => Some(((y, x), min_d)),
        None => None
    }
}

fn solve(tc: TestCase) {
    let mut state = get_initial_state(&tc);

    while let Some((next_pos, d)) = bfs_board(&state) {
        state.board[state.pos] = 0;
        state.board[next_pos] = 9;
        state.pos = next_pos;
        state.elapsed += d as u32;
        state.eaten += 1;

        if state.eaten == state.size {
            state.size += 1;
            state.eaten = 0;
        }
    }

    println!("{}", state.elapsed);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().expect("IO error").parse().unwrap();

    let mut board: Vec<u32> = Vec::new();

    while let Some(line) = lines.next() {
        line.unwrap().split_whitespace().for_each(|d| board.push(d.parse::<u32>().unwrap()));
    }

    solve(TestCase { n, board });
}