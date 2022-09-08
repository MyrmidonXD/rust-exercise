use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct TestCase {
    n: usize,
    board: Vec<char>
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

impl<T> Matrix<T> {
    fn map_inplace<F>(&mut self, f: F) where F: Fn(&T) -> T {
        for y in 0..self.row {
            for x in 0..self.col {
                self[(y, x)] = f(&self[(y, x)]);
            }
        }
    }

    #[allow(dead_code)]
    fn map<F, U>(&self, f: F) -> Matrix<U> where F: FnMut(&T) -> U {
        let new_data: Vec<U> = self.data.iter().map(f).collect();

        Matrix { row: self.row, col: self.col, data: new_data }
    }

    fn get_neighbor_indexes(&self, idx: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (y, x) = idx;

        if y > 0 { neighbors.push((y-1, x)) }
        if y < self.row-1 { neighbors.push((y+1, x)) }
        if x > 0 { neighbors.push((y, x-1)) }
        if x < self.col-1 { neighbors.push((y, x+1)) }

        return neighbors;
    }
}

#[derive(Debug)]
struct State {
    n: usize,
    board: Matrix<char>
}

fn get_initial_state(tc: &TestCase) -> State {
    let (m, n) = (tc.n, tc.n);
    let board = Matrix { row: n, col: m, data: tc.board.to_owned() };

    return State { n, board };
}

fn bfs_board(s: &State) -> usize {
    let mut queue: VecDeque<((usize, usize), char)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut n_area = 0;

    for y in 0..s.n {
        for x in 0..s.n {
            if visited.contains(&(y, x)) { continue; }

            n_area += 1;
            
            queue.push_back(((y, x), s.board[(y, x)]));
            visited.insert((y, x));

            while !queue.is_empty() {
                let (idx, c) = queue.pop_front().unwrap();

                for neighbor_idx in s.board.get_neighbor_indexes(idx) {
                    if !visited.contains(&neighbor_idx) && s.board[neighbor_idx] == c {
                        queue.push_back((neighbor_idx, c));
                        visited.insert(neighbor_idx);
                    }
                }
            }
        }
    }

    return n_area;
}

fn solve(tc: TestCase) {
    let mut state = get_initial_state(&tc);

    let non_cvd = bfs_board(&state);
    
    state.board.map_inplace(|&c| if c == 'R' { 'G' } else { c });
    let cvd = bfs_board(&state);

    println!("{} {}", non_cvd, cvd);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().expect("IO error").parse().unwrap();
                                  
    let mut board: Vec<char> = Vec::new();

    while let Some(line) = lines.next() {
        line.unwrap().chars().for_each(|c| board.push(c));
    }

    solve(TestCase { n, board });
}