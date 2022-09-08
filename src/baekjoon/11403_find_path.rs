#[allow(unused_imports)]
use std::io::{self, BufReader, BufRead, BufWriter, Write, Read};
use std::ops::{Index, IndexMut};
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
struct TestCase {
    n: u8,
    edges: Vec<u8>
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

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.row {
            for j in 0..self.col {
                if j == 0 {
                    write!(f, "{}", self[(i, j)])?;
                } else {
                    write!(f, " {}", self[(i, j)])?;
                }
            }

            if i < self.row - 1 {
                write!(f, "\n")?;
            }
        }
    
        fmt::Result::Ok(())
    }
}

fn solve(tc: TestCase) -> Matrix<u8> {
    let n = tc.n as usize;
    let edges = Matrix { row: n, col: n, data: tc.edges };
    let mut result = edges.clone();

    // Floyd-Warshall
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if result[(i, k)] == 1 && result[(k, j)] == 1 {
                    result[(i, j)] = 1;
                }
            }
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut buffer = String::new();

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    // Parsing
    let _ = stdin.read_to_string(&mut buffer);
    let mut inputs = buffer.split_whitespace().map(|s| s.parse::<u8>().unwrap());

    let n = inputs.next().unwrap();
    let edges = inputs.collect();

    // Solve
    let result = solve(TestCase { n, edges });

    let _ = writeln!(stdout, "{}", result);
}