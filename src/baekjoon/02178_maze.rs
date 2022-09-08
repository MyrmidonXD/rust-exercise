#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::{Index, IndexMut};
use std::collections::VecDeque;

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    m: usize,
    board: Vec<u8>,
}

#[derive(Debug, Clone)]
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

fn bfs(board: &Matrix<u8>) -> usize {
    let mut visited = Matrix { row: board.row, col: board.col, data: vec![false; board.data.len()] };
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

    let target = (board.row - 1, board.col - 1);

    queue.push_back(((0, 0), 1));
    visited[(0, 0)] = true;

    while let Some(((y, x), d)) = queue.pop_front() {
        let next = (y.saturating_sub(1), x);
        if y > 0 && board[next] == 1 && !visited[next] {
            if target == next { return d + 1; }

            queue.push_back((next, d + 1));
            visited[next] = true;
        }

        let next = (y+1, x);
        if y < board.row - 1 && board[next] == 1 && !visited[next] {
            if target == next { return d + 1; }

            queue.push_back((next, d + 1));
            visited[next] = true;
        }

        let next = (y, x.saturating_sub(1));
        if x > 0 && board[next] == 1 && !visited[next] {
            if target == next { return d + 1; }

            queue.push_back((next, d + 1));
            visited[next] = true;
        }

        let next = (y, x+1);
        if x < board.col - 1 && board[next] == 1 && !visited[next] {
            if target == next { return d + 1; }

            queue.push_back((next, d + 1));
            visited[next] = true;
        }
    }

    return 0; // <- impossible case
}

fn solve(tc: TestCase) -> usize {
    let board = Matrix { row: tc.n, col: tc.m, data: tc.board };
    bfs(&board)
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
    let mut lines = buffer.lines();

    let nm: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (n, m) = (nm[0], nm[1]);
    let mut board = vec![];

    while let Some(s) = lines.next() {
        let chars = s.chars();
        chars.for_each(|c| board.push(c.to_digit(10).unwrap() as u8));
    }

    // Solve
    let result = solve(TestCase { n, m, board });

    let _ = writeln!(stdout, "{result}");
}