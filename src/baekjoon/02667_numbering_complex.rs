#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
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

fn dfs(board: &Matrix<u8>, visited: &mut Matrix<bool>, index: (usize, usize)) -> usize {
    let mut stack = vec![index];
    visited[index] = true;

    let mut count = 0;

    while let Some((y, x)) = stack.pop() {
        count += 1;

        // up
        if y > 0 && board[(y-1, x)] == 1 && !visited[(y-1, x)] {
            stack.push((y-1, x));
            visited[(y-1, x)] = true;
        }
        // down
        if y < board.row - 1 && board[(y+1, x)] == 1 && !visited[(y+1, x)] {
            stack.push((y+1, x));
            visited[(y+1, x)] = true;
        }
        // left
        if x > 0 && board[(y, x-1)] == 1 && !visited[(y, x-1)] {
            stack.push((y, x-1));
            visited[(y, x-1)] = true;
        }
        // right
        if x < board.col - 1 && board[(y, x+1)] == 1 && !visited[(y, x+1)] {
            stack.push((y, x+1));
            visited[(y, x+1)] = true;
        }
    }

    count
}

fn solve(tc: TestCase, w: &mut impl Write) {
    let board = Matrix { row: tc.n, col: tc.n, data: tc.board };
    let mut visited = Matrix { row: tc.n, col: tc.n, data: vec![false; tc.n * tc.n] };

    let mut n_complex: usize = 0;
    let mut n_houses: Vec<usize> = vec![];

    for y in 0..board.row {
        for x in 0..board.col {
            if board[(y, x)] == 1 && !visited[(y, x)] {
                n_complex += 1;
                n_houses.push(dfs(&board, &mut visited, (y, x)));
            }
        }
    }

    n_houses.sort_unstable();

    let _ = writeln!(w, "{}", n_complex);
    for n_house in n_houses {
        let _ = writeln!(w, "{}", n_house);
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
    let mut lines = buffer.lines();

    let n = lines.next().unwrap().parse().unwrap();
    let mut board = vec![];

    while let Some(s) = lines.next() {
        let chars = s.chars();
        chars.for_each(|c| board.push(c.to_digit(10).unwrap() as u8));
    }

    // Solve
    let result = solve(TestCase { n, board }, &mut stdout);
}