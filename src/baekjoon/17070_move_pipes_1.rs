#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::{Index, IndexMut};

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

#[derive(Debug)]
#[allow(dead_code)]
struct TestCase {
    n: usize,
    board: Vec<usize>,
}

fn solve(tc: TestCase) -> usize {
    let TestCase { n, board } = tc;

    let board = Matrix { row: n, col: n, data: board };
    let mut case_hori = Matrix { row: n, col: n, data: vec![0usize; n*n] };
    let mut case_vert = Matrix { row: n, col: n, data: vec![0usize; n*n] };
    let mut case_diag = Matrix { row: n, col: n, data: vec![0usize; n*n] };

    case_hori[(0, 1)] = 1;
    for i in 0..n {
        for j in 1..n {
            // if this cell is a wall, then all case[(i, j)] == 0
            if board[(i, j)] == 1 { continue; }

            // check horizontal (ignore starting point (0, 1))
            if (i, j) != (0, 1) && board[(i, j-1)] != 1 {
                case_hori[(i, j)] = case_hori[(i, j-1)] + case_diag[(i, j-1)];
            }

            // check vertical
            if i > 0 && board[(i-1, j)] != 1 {
                case_vert[(i, j)] = case_vert[(i-1, j)] + case_diag[(i-1, j)];
            }

            // check diagonal
            if i > 0 && board[(i-1, j-1)] != 1 && board[(i-1, j)] != 1 && board[(i, j-1)] != 1 {
                case_diag[(i, j)] = case_diag[(i-1, j-1)] + case_hori[(i-1, j-1)] + case_vert[(i-1, j-1)];
            }
        }
    }

    case_hori[(n-1, n-1)] + case_vert[(n-1, n-1)] + case_diag[(n-1, n-1)]
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
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    let n = inputs.next().unwrap();
    let board = inputs.collect();

    // Solve
    let result = solve(TestCase { n, board });
    
    let _ = writeln!(stdout, "{result}");
}